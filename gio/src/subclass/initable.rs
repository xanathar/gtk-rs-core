// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::Cast;
use glib::translate::*;
use glib::Error;

use glib::subclass::prelude::*;

use std::ptr;

use crate::Cancellable;
use crate::Initable;

pub trait InitableImpl: ObjectImpl + Send {
    fn init(&self, initable: &Self::Type, cancellable: Option<&Cancellable>) -> Result<(), Error>;
}

pub trait InitableImplExt: ObjectSubclass {
    fn parent_init(
        &self,
        initable: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error>;
}

impl<T: InitableImpl> InitableImplExt for T {
    fn parent_init(
        &self,
        initable: &Self::Type,
        cancellable: Option<&Cancellable>,
    ) -> Result<(), Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Initable>() as *const ffi::GInitableIface;

            let func = (*parent_iface)
                .init
                .expect("no parent \"init\" implementation");

            let mut err = ptr::null_mut();
            func(
                initable.unsafe_cast_ref::<Initable>().to_glib_none().0,
                cancellable.to_glib_none().0,
                &mut err,
            );

            if err.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(err))
            }
        }
    }
}

unsafe impl<T: InitableImpl> IsImplementable<T> for Initable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.init = Some(initable_init::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn initable_init<T: InitableImpl>(
    initable: *mut ffi::GInitable,
    cancellable: *mut ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(initable as *mut T::Instance);
    let imp = instance.impl_();

    match imp.init(
        from_glib_borrow::<_, Initable>(initable).unsafe_cast_ref(),
        Option::<Cancellable>::from_glib_borrow(cancellable)
            .as_ref()
            .as_ref(),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(e) => {
            if !error.is_null() {
                *error = e.into_raw();
            }
            glib::ffi::GFALSE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glib::object::*;

    pub mod imp {
        use super::*;
        use crate::Cancellable;
        use crate::Initable;
        use std::cell::Cell;

        pub struct InitableTestType(pub Cell<u64>);

        #[glib::object_subclass]
        impl ObjectSubclass for InitableTestType {
            const NAME: &'static str = "InitableTestType";
            type Type = super::InitableTestType;
            type ParentType = glib::Object;
            type Interfaces = (Initable,);

            fn new() -> Self {
                Self(Cell::new(0))
            }
        }

        impl InitableImpl for InitableTestType {
            fn init(
                &self,
                initable: &Self::Type,
                _cancellable: Option<&Cancellable>,
            ) -> Result<(), glib::Error> {
                InitableTestType::from_instance(initable)
                    .0
                    .set(0x123456789abcdef);
                Ok(())
            }
        }

        impl ObjectImpl for InitableTestType {}
    }

    pub mod ffi {
        use super::*;
        pub type InitableTestType = <imp::InitableTestType as ObjectSubclass>::Instance;

        #[no_mangle]
        pub unsafe extern "C" fn initable_test_type_get_type() -> glib::ffi::GType {
            imp::InitableTestType::type_().into_glib()
        }

        #[no_mangle]
        pub unsafe extern "C" fn initable_test_type_get_value(this: *mut InitableTestType) -> u64 {
            let this = super::InitableTestType::from_glib_borrow(this);
            imp::InitableTestType::from_instance(&this).0.get()
        }
    }

    glib::wrapper! {
        pub struct InitableTestType(ObjectSubclass<imp::InitableTestType>)
            @implements Initable;
    }

    impl InitableTestType {
        pub fn new() -> Self {
            glib::Object::new(&[]).expect("Failed creation of InitableTestType object")
        }
    }

    #[test]
    fn test_initable_through_ffi() {
        unsafe {
            let test = InitableTestType::new();
            let test: *mut ffi::InitableTestType = test.as_ptr();
            let mut error: *mut glib::ffi::GError = std::ptr::null_mut();

            assert_ne!(0x123456789abcdef, ffi::initable_test_type_get_value(test));

            let result = crate::ffi::g_initable_init(
                test as *mut crate::ffi::GInitable,
                std::ptr::null_mut(),
                &mut error,
            );

            assert_eq!(glib::ffi::GTRUE, result);
            assert_eq!(0x123456789abcdef, ffi::initable_test_type_get_value(test));
        }
    }
}
