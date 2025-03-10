// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Cancellable, DBusAuthObserver, DBusConnection, DBusServerFlags, Initable};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GDBusServer")]
    pub struct DBusServer(Object<ffi::GDBusServer>) @implements Initable;

    match fn {
        type_ => || ffi::g_dbus_server_get_type(),
    }
}

impl DBusServer {
    #[doc(alias = "g_dbus_server_new_sync")]
    pub fn new_sync(
        address: &str,
        flags: DBusServerFlags,
        guid: &str,
        observer: Option<&DBusAuthObserver>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<DBusServer, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_dbus_server_new_sync(
                address.to_glib_none().0,
                flags.into_glib(),
                guid.to_glib_none().0,
                observer.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_server_get_client_address")]
    #[doc(alias = "get_client_address")]
    #[doc(alias = "client-address")]
    pub fn client_address(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_dbus_server_get_client_address(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_dbus_server_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> DBusServerFlags {
        unsafe { from_glib(ffi::g_dbus_server_get_flags(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_dbus_server_get_guid")]
    #[doc(alias = "get_guid")]
    pub fn guid(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_dbus_server_get_guid(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_dbus_server_is_active")]
    #[doc(alias = "active")]
    pub fn is_active(&self) -> bool {
        unsafe { from_glib(ffi::g_dbus_server_is_active(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_dbus_server_start")]
    pub fn start(&self) {
        unsafe {
            ffi::g_dbus_server_start(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_dbus_server_stop")]
    pub fn stop(&self) {
        unsafe {
            ffi::g_dbus_server_stop(self.to_glib_none().0);
        }
    }

    pub fn address(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "address")
    }

    #[doc(alias = "authentication-observer")]
    pub fn authentication_observer(&self) -> Option<DBusAuthObserver> {
        ObjectExt::property(self, "authentication-observer")
    }

    #[doc(alias = "new-connection")]
    pub fn connect_new_connection<F: Fn(&Self, &DBusConnection) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn new_connection_trampoline<
            F: Fn(&DBusServer, &DBusConnection) -> bool + 'static,
        >(
            this: *mut ffi::GDBusServer,
            connection: *mut ffi::GDBusConnection,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(connection)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"new-connection".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    new_connection_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active")]
    pub fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&DBusServer) + 'static>(
            this: *mut ffi::GDBusServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::active".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "client-address")]
    pub fn connect_client_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_address_trampoline<F: Fn(&DBusServer) + 'static>(
            this: *mut ffi::GDBusServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::client-address".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_client_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
