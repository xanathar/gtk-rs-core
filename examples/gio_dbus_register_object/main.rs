use gio::prelude::*;

glib::wrapper! {
    pub struct SampleApplication(ObjectSubclass<imp::SampleApplication>)
        @extends gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for SampleApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property(
                "application-id",
                "com.github.gtk-rs.examples.RegisterDBusObject",
            )
            .build()
    }
}

mod imp {
    use std::cell::RefCell;
    use std::time::Duration;

    use gio::prelude::*;
    use gio::subclass::prelude::*;
    use gio::{DBusConnection, DBusError};

    const EXAMPLE_XML: &str = r#"
<node>
  <interface name='com.github.gtk_rs.examples.HelloWorld'>
    <method name='Hello'>
      <arg type='s' name='name' direction='in'/>
      <arg type='s' name='greet' direction='out'/>
    </method>
    <method name='SlowHello'>
      <arg type='s' name='name' direction='in'/>
      <arg type='u' name='delay' direction='in'/>
      <arg type='s' name='greet' direction='out'/>
    </method>
    <method name='GoodBye'></method>
  </interface>
</node>
"#;

    #[derive(Debug, glib::Variant)]
    struct Hello {
        name: String,
    }

    #[derive(Debug, glib::Variant)]
    struct SlowHello {
        name: String,
        delay: u32,
    }

    #[derive(Debug)]
    enum HelloMethod {
        Hello(Hello),
        SlowHello(SlowHello),
        GoodBye,
    }

    impl DBusMethodCall for HelloMethod {
        fn parse_call(
            _obj_path: &str,
            _interface: Option<&str>,
            method: &str,
            params: glib::Variant,
        ) -> Result<Self, glib::Error> {
            match method {
                "Hello" => Ok(params.get::<Hello>().map(Self::Hello)),
                "SlowHello" => Ok(params.get::<SlowHello>().map(Self::SlowHello)),
                "GoodBye" => Ok(Some(Self::GoodBye)),
                _ => Err(glib::Error::new(DBusError::UnknownMethod, "No such method")),
            }
            .and_then(|p| {
                p.ok_or_else(|| glib::Error::new(DBusError::InvalidArgs, "Invalid parameters"))
            })
        }
    }

    #[derive(Default)]
    pub struct SampleApplication {
        registration_id: RefCell<Option<gio::RegistrationId>>,
    }

    impl SampleApplication {
        fn register_object(
            &self,
            connection: &DBusConnection,
        ) -> Result<gio::RegistrationId, glib::Error> {
            let example = gio::DBusNodeInfo::for_xml(EXAMPLE_XML)
                .ok()
                .and_then(|e| e.lookup_interface("com.github.gtk_rs.examples.HelloWorld"))
                .expect("Example interface");

            connection
                .register_object("/com/github/gtk_rs/examples/HelloWorld", &example)
                .typed_method_call::<HelloMethod>()
                .invoke_and_return_future_local(glib::clone!(
                    #[weak_allow_none(rename_to = app)]
                    self.obj(),
                    move |_, sender, call| {
                        println!("Method call from {sender:?}");
                        let app = app.clone();
                        async move {
                            match call {
                                HelloMethod::Hello(Hello { name }) => {
                                    let greet = format!("Hello {name}!");
                                    println!("{greet}");
                                    Ok(Some(greet.to_variant()))
                                }
                                HelloMethod::SlowHello(SlowHello { name, delay }) => {
                                    glib::timeout_future(Duration::from_secs(delay as u64)).await;
                                    let greet = format!("Hello {name} after {delay} seconds!");
                                    println!("{greet}");
                                    Ok(Some(greet.to_variant()))
                                }
                                HelloMethod::GoodBye => {
                                    if let Some(app) = app {
                                        app.quit();
                                    }
                                    Ok(None)
                                }
                            }
                        }
                    }
                ))
                .build()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SampleApplication {
        const NAME: &'static str = "SampleApplication";

        type Type = super::SampleApplication;

        type ParentType = gio::Application;
    }

    impl ObjectImpl for SampleApplication {}

    impl ApplicationImpl for SampleApplication {
        fn dbus_register(
            &self,
            connection: &DBusConnection,
            object_path: &str,
        ) -> Result<(), glib::Error> {
            self.parent_dbus_register(connection, object_path)?;
            self.registration_id
                .replace(Some(self.register_object(connection)?));
            println!("registered object on session bus");
            Ok(())
        }

        fn dbus_unregister(&self, connection: &DBusConnection, object_path: &str) {
            self.parent_dbus_unregister(connection, object_path);
            if let Some(id) = self.registration_id.take() {
                if connection.unregister_object(id).is_ok() {
                    println!("Unregistered object");
                } else {
                    eprintln!("Could not unregister object");
                }
            }
        }

        fn shutdown(&self) {
            self.parent_shutdown();
            println!("Good bye!");
        }

        fn activate(&self) {
            println!("Waiting for DBus Hello method to be called. Call the following command from another terminal:");
            println!("dbus-send --print-reply --dest=com.github.gtk-rs.examples.RegisterDBusObject /com/github/gtk_rs/examples/HelloWorld com.github.gtk_rs.examples.HelloWorld.Hello string:YourName");
            println!("Quit with the following command:");
            println!("dbus-send --print-reply --dest=com.github.gtk-rs.examples.RegisterDBusObject /com/github/gtk_rs/examples/HelloWorld com.github.gtk_rs.examples.HelloWorld.GoodBye");
        }
    }
}

fn main() -> glib::ExitCode {
    let app = SampleApplication::default();
    let _guard = app.hold();
    app.run()
}
