// This file was generated by gir (c9185c9) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use SizeGroupMode;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct SizeGroup(Object<ffi::GtkSizeGroup>): Buildable;

    match fn {
        get_type => || ffi::gtk_size_group_get_type(),
    }
}

impl SizeGroup {
    pub fn new(mode: SizeGroupMode) -> SizeGroup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_size_group_new(mode))
        }
    }

    pub fn add_widget<T: IsA<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_size_group_add_widget(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    pub fn get_ignore_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_size_group_get_ignore_hidden(self.to_glib_none().0))
        }
    }

    pub fn get_mode(&self) -> SizeGroupMode {
        unsafe {
            ffi::gtk_size_group_get_mode(self.to_glib_none().0)
        }
    }

    pub fn get_widgets(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_size_group_get_widgets(self.to_glib_none().0))
        }
    }

    pub fn remove_widget<T: IsA<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_size_group_remove_widget(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    pub fn set_ignore_hidden(&self, ignore_hidden: bool) {
        unsafe {
            ffi::gtk_size_group_set_ignore_hidden(self.to_glib_none().0, ignore_hidden.to_glib());
        }
    }

    pub fn set_mode(&self, mode: SizeGroupMode) {
        unsafe {
            ffi::gtk_size_group_set_mode(self.to_glib_none().0, mode);
        }
    }
}
