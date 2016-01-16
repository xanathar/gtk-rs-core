// This file was generated by gir (463de47) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct SeparatorMenuItem(Object<ffi::GtkSeparatorMenuItem>): Widget, Container, Bin, MenuItem, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_separator_menu_item_get_type(),
    }
}

impl SeparatorMenuItem {
    pub fn new() -> SeparatorMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_separator_menu_item_new()).downcast_unchecked()
        }
    }
}
