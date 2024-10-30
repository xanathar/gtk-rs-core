// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, AppInfo, File};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GAppLaunchContext")]
    pub struct AppLaunchContext(Object<ffi::GAppLaunchContext, ffi::GAppLaunchContextClass>);

    match fn {
        type_ => || ffi::g_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub const NONE: Option<&'static AppLaunchContext> = None;

    #[doc(alias = "g_app_launch_context_new")]
    pub fn new() -> AppLaunchContext {
        unsafe { from_glib_full(ffi::g_app_launch_context_new()) }
    }
}

impl Default for AppLaunchContext {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AppLaunchContext>> Sealed for T {}
}

pub trait AppLaunchContextExt: IsA<AppLaunchContext> + sealed::Sealed + 'static {
    #[doc(alias = "g_app_launch_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self, info: &impl IsA<AppInfo>, files: &[File]) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_get_display(
                self.as_ref().to_glib_none().0,
                info.as_ref().to_glib_none().0,
                files.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_app_launch_context_get_environment")]
    #[doc(alias = "get_environment")]
    fn environment(&self) -> Vec<std::ffi::OsString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_launch_context_get_environment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_app_launch_context_get_startup_notify_id")]
    #[doc(alias = "get_startup_notify_id")]
    fn startup_notify_id(
        &self,
        info: Option<&impl IsA<AppInfo>>,
        files: &[File],
    ) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_get_startup_notify_id(
                self.as_ref().to_glib_none().0,
                info.map(|p| p.as_ref()).to_glib_none().0,
                files.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_app_launch_context_launch_failed")]
    fn launch_failed(&self, startup_notify_id: &str) {
        unsafe {
            ffi::g_app_launch_context_launch_failed(
                self.as_ref().to_glib_none().0,
                startup_notify_id.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_app_launch_context_setenv")]
    fn setenv(&self, variable: impl AsRef<std::ffi::OsStr>, value: impl AsRef<std::ffi::OsStr>) {
        unsafe {
            ffi::g_app_launch_context_setenv(
                self.as_ref().to_glib_none().0,
                variable.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_app_launch_context_unsetenv")]
    fn unsetenv(&self, variable: impl AsRef<std::ffi::OsStr>) {
        unsafe {
            ffi::g_app_launch_context_unsetenv(
                self.as_ref().to_glib_none().0,
                variable.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "launch-failed")]
    fn connect_launch_failed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn launch_failed_trampoline<
            P: IsA<AppLaunchContext>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::GAppLaunchContext,
            startup_notify_id: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AppLaunchContext::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(startup_notify_id),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"launch-failed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    launch_failed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_72")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
    #[doc(alias = "launch-started")]
    fn connect_launch_started<F: Fn(&Self, &AppInfo, Option<&glib::Variant>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn launch_started_trampoline<
            P: IsA<AppLaunchContext>,
            F: Fn(&P, &AppInfo, Option<&glib::Variant>) + 'static,
        >(
            this: *mut ffi::GAppLaunchContext,
            info: *mut ffi::GAppInfo,
            platform_data: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AppLaunchContext::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(info),
                Option::<glib::Variant>::from_glib_borrow(platform_data)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"launch-started\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    launch_started_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "launched")]
    fn connect_launched<F: Fn(&Self, &AppInfo, &glib::Variant) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn launched_trampoline<
            P: IsA<AppLaunchContext>,
            F: Fn(&P, &AppInfo, &glib::Variant) + 'static,
        >(
            this: *mut ffi::GAppLaunchContext,
            info: *mut ffi::GAppInfo,
            platform_data: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AppLaunchContext::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(info),
                &from_glib_borrow(platform_data),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"launched\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    launched_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AppLaunchContext>> AppLaunchContextExt for O {}
