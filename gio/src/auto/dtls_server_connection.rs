// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, DatagramBased, DtlsConnection, TlsAuthenticationMode, TlsCertificate};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GDtlsServerConnection")]
    pub struct DtlsServerConnection(Interface<ffi::GDtlsServerConnection, ffi::GDtlsServerConnectionInterface>) @requires DatagramBased, DtlsConnection;

    match fn {
        type_ => || ffi::g_dtls_server_connection_get_type(),
    }
}

impl DtlsServerConnection {
    pub const NONE: Option<&'static DtlsServerConnection> = None;

    #[doc(alias = "g_dtls_server_connection_new")]
    pub fn new(
        base_socket: &impl IsA<DatagramBased>,
        certificate: Option<&impl IsA<TlsCertificate>>,
    ) -> Result<DtlsServerConnection, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_dtls_server_connection_new(
                base_socket.as_ref().to_glib_none().0,
                certificate.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DtlsServerConnection>> Sealed for T {}
}

pub trait DtlsServerConnectionExt: IsA<DtlsServerConnection> + sealed::Sealed + 'static {
    #[doc(alias = "authentication-mode")]
    fn authentication_mode(&self) -> TlsAuthenticationMode {
        ObjectExt::property(self.as_ref(), "authentication-mode")
    }

    #[doc(alias = "authentication-mode")]
    fn set_authentication_mode(&self, authentication_mode: TlsAuthenticationMode) {
        ObjectExt::set_property(self.as_ref(), "authentication-mode", authentication_mode)
    }

    #[doc(alias = "authentication-mode")]
    fn connect_authentication_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_authentication_mode_trampoline<
            P: IsA<DtlsServerConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsServerConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsServerConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::authentication-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_authentication_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DtlsServerConnection>> DtlsServerConnectionExt for O {}
