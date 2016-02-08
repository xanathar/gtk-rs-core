// This file was generated by gir (c9185c9) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
#[cfg(gtk_3_10)]
use RevealerTransitionType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Revealer(Object<ffi::GtkRevealer>): Widget, Container, Bin, Buildable;

    match fn {
        get_type => || ffi::gtk_revealer_get_type(),
    }
}

impl Revealer {
    #[cfg(gtk_3_10)]
    pub fn new() -> Revealer {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_revealer_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_child_revealed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_revealer_get_child_revealed(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_reveal_child(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_revealer_get_reveal_child(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_revealer_get_transition_duration(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_transition_type(&self) -> RevealerTransitionType {
        unsafe {
            ffi::gtk_revealer_get_transition_type(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_reveal_child(&self, reveal_child: bool) {
        unsafe {
            ffi::gtk_revealer_set_reveal_child(self.to_glib_none().0, reveal_child.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_revealer_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_transition_type(&self, transition: RevealerTransitionType) {
        unsafe {
            ffi::gtk_revealer_set_transition_type(self.to_glib_none().0, transition);
        }
    }
}
