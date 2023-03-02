use glib::{subclass::prelude::*, translate::*, Cast};

use crate::SocketControlMessage;

pub trait SocketControlMessageImpl: ObjectImpl + SocketControlMessageImplExt + Send {

    fn level(&self) -> i32 {
        self.parent_level()
    }

    fn msg_type(&self) -> i32 {
        self.parent_msg_type()
    }

    fn size(&self) -> usize {
        self.parent_size()
    }
}

pub trait SocketControlMessageImplExt: ObjectSubclass {
    fn parent_level(&self) -> i32;

    fn parent_msg_type(&self) -> i32;

    fn parent_size(&self) -> usize;
}

impl<T: SocketControlMessageImpl> SocketControlMessageImplExt for T {
    fn parent_level(&self) -> i32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GSocketControlMessageClass;
            let f = (*parent_class)
                .get_level
                .expect("No parent class implementation for \"level\"");

            f(self.obj().unsafe_cast_ref::<SocketControlMessage>().to_glib_none().0)
        }
    }

    fn parent_msg_type(&self) -> i32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GSocketControlMessageClass;
            let f = (*parent_class)
                .get_type
                .expect("No parent class implementation for \"msg_type\"");

            f(self.obj().unsafe_cast_ref::<SocketControlMessage>().to_glib_none().0)
        }

    }

    fn parent_size(&self) -> usize {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GSocketControlMessageClass;
            let f = (*parent_class)
                .get_size
                .expect("No parent class implementation for \"size\"");

            f(self.obj().unsafe_cast_ref::<SocketControlMessage>().to_glib_none().0)
        }
    }
}

unsafe impl<T: SocketControlMessageImpl> IsSubclassable<T> for SocketControlMessage {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.get_level = Some(socket_control_message_get_level::<T>);
        klass.get_type = Some(socket_control_message_get_type::<T>);
        klass.get_size = Some(socket_control_message_get_size::<T>);
    }
}

unsafe extern "C" fn socket_control_message_get_level<T: SocketControlMessageImpl>(
    ptr: *mut ffi::GSocketControlMessage,
) -> i32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.level()
}

unsafe extern "C" fn socket_control_message_get_type<T: SocketControlMessageImpl>(
    ptr: *mut ffi::GSocketControlMessage,
) -> i32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.msg_type()
}

unsafe extern "C" fn socket_control_message_get_size<T: SocketControlMessageImpl>(
    ptr: *mut ffi::GSocketControlMessage,
) -> usize {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.size()
}
