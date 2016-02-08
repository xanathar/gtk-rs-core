// This file was generated by gir (c9185c9) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use CellAreaContext;
use CellEditable;
use CellLayout;
use CellRenderer;
use CellRendererState;
use DirectionType;
use Orientation;
use Rectangle;
use SizeRequestMode;
use TreeIter;
use TreeModel;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct CellArea(Object<ffi::GtkCellArea>): Buildable, CellLayout;

    match fn {
        get_type => || ffi::gtk_cell_area_get_type(),
    }
}

pub trait CellAreaExt {
    fn activate<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cell_area: &Rectangle, flags: CellRendererState, edit_only: bool) -> bool;

    //fn activate_cell<T: IsA<Widget>, U: IsA<CellRenderer>>(&self, widget: &T, renderer: &U, event: /*Unknown conversion*//*Unimplemented*/Event, cell_area: &Rectangle, flags: CellRendererState) -> bool;

    fn add<T: IsA<CellRenderer>>(&self, renderer: &T);

    fn add_focus_sibling<T: IsA<CellRenderer>, U: IsA<CellRenderer>>(&self, renderer: &T, sibling: &U);

    //fn add_with_properties<T: IsA<CellRenderer>>(&self, renderer: &T, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn apply_attributes<T: IsA<TreeModel>>(&self, tree_model: &T, iter: &mut TreeIter, is_expander: bool, is_expanded: bool);

    fn attribute_connect<T: IsA<CellRenderer>>(&self, renderer: &T, attribute: &str, column: i32);

    fn attribute_disconnect<T: IsA<CellRenderer>>(&self, renderer: &T, attribute: &str);

    #[cfg(gtk_3_14)]
    fn attribute_get_column<T: IsA<CellRenderer>>(&self, renderer: &T, attribute: &str) -> i32;

    //fn cell_get<T: IsA<CellRenderer>>(&self, renderer: &T, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn cell_get_property<T: IsA<CellRenderer>>(&self, renderer: &T, property_name: &str, value: /*Ignored*/&mut gobject::Value);

    //fn cell_get_valist<T: IsA<CellRenderer>>(&self, renderer: &T, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn cell_set<T: IsA<CellRenderer>>(&self, renderer: &T, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn cell_set_property<T: IsA<CellRenderer>>(&self, renderer: &T, property_name: &str, value: /*Ignored*/&gobject::Value);

    //fn cell_set_valist<T: IsA<CellRenderer>>(&self, renderer: &T, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn copy_context(&self, context: &CellAreaContext) -> Option<CellAreaContext>;

    fn create_context(&self) -> Option<CellAreaContext>;

    //fn event<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, event: /*Unknown conversion*//*Unimplemented*/Event, cell_area: &Rectangle, flags: CellRendererState) -> i32;

    fn focus(&self, direction: DirectionType) -> bool;

    //fn foreach(&self, callback: /*Unknown conversion*//*Unimplemented*/CellCallback, callback_data: /*Unimplemented*/Fundamental: Pointer);

    //fn foreach_alloc<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cell_area: &Rectangle, background_area: &Rectangle, callback: /*Unknown conversion*//*Unimplemented*/CellAllocCallback, callback_data: /*Unimplemented*/Fundamental: Pointer);

    fn get_cell_allocation<T: IsA<Widget>, U: IsA<CellRenderer>>(&self, context: &CellAreaContext, widget: &T, renderer: &U, cell_area: &Rectangle) -> Rectangle;

    fn get_cell_at_position<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cell_area: &Rectangle, x: i32, y: i32) -> (CellRenderer, Rectangle);

    fn get_current_path_string(&self) -> Option<String>;

    fn get_edit_widget(&self) -> Option<CellEditable>;

    fn get_edited_cell(&self) -> Option<CellRenderer>;

    fn get_focus_cell(&self) -> Option<CellRenderer>;

    fn get_focus_from_sibling<T: IsA<CellRenderer>>(&self, renderer: &T) -> Option<CellRenderer>;

    fn get_focus_siblings<T: IsA<CellRenderer>>(&self, renderer: &T) -> Vec<CellRenderer>;

    fn get_preferred_height<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T) -> (i32, i32);

    fn get_preferred_height_for_width<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, width: i32) -> (i32, i32);

    fn get_preferred_width<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T) -> (i32, i32);

    fn get_preferred_width_for_height<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, height: i32) -> (i32, i32);

    fn get_request_mode(&self) -> SizeRequestMode;

    fn has_renderer<T: IsA<CellRenderer>>(&self, renderer: &T) -> bool;

    fn inner_cell_area<T: IsA<Widget>>(&self, widget: &T, cell_area: &Rectangle) -> Rectangle;

    fn is_activatable(&self) -> bool;

    fn is_focus_sibling<T: IsA<CellRenderer>, U: IsA<CellRenderer>>(&self, renderer: &T, sibling: &U) -> bool;

    fn remove<T: IsA<CellRenderer>>(&self, renderer: &T);

    fn remove_focus_sibling<T: IsA<CellRenderer>, U: IsA<CellRenderer>>(&self, renderer: &T, sibling: &U);

    //fn render<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cr: /*Ignored*/&mut cairo::Context, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState, paint_focus: bool);

    fn request_renderer<T: IsA<CellRenderer>, U: IsA<Widget>>(&self, renderer: &T, orientation: Orientation, widget: &U, for_size: i32) -> (i32, i32);

    fn set_focus_cell<T: IsA<CellRenderer>>(&self, renderer: &T);

    fn stop_editing(&self, canceled: bool);
}

impl<O: IsA<CellArea>> CellAreaExt for O {
    fn activate<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cell_area: &Rectangle, flags: CellRendererState, edit_only: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_activate(self.to_glib_none().0, context.to_glib_none().0, widget.to_glib_none().0, cell_area.to_glib_none().0, flags, edit_only.to_glib()))
        }
    }

    //fn activate_cell<T: IsA<Widget>, U: IsA<CellRenderer>>(&self, widget: &T, renderer: &U, event: /*Unknown conversion*//*Unimplemented*/Event, cell_area: &Rectangle, flags: CellRendererState) -> bool {
    //    unsafe { TODO: call ffi::gtk_cell_area_activate_cell() }
    //}

    fn add<T: IsA<CellRenderer>>(&self, renderer: &T) {
        unsafe {
            ffi::gtk_cell_area_add(self.to_glib_none().0, renderer.to_glib_none().0);
        }
    }

    fn add_focus_sibling<T: IsA<CellRenderer>, U: IsA<CellRenderer>>(&self, renderer: &T, sibling: &U) {
        unsafe {
            ffi::gtk_cell_area_add_focus_sibling(self.to_glib_none().0, renderer.to_glib_none().0, sibling.to_glib_none().0);
        }
    }

    //fn add_with_properties<T: IsA<CellRenderer>>(&self, renderer: &T, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_area_add_with_properties() }
    //}

    fn apply_attributes<T: IsA<TreeModel>>(&self, tree_model: &T, iter: &mut TreeIter, is_expander: bool, is_expanded: bool) {
        unsafe {
            ffi::gtk_cell_area_apply_attributes(self.to_glib_none().0, tree_model.to_glib_none().0, iter.to_glib_none_mut().0, is_expander.to_glib(), is_expanded.to_glib());
        }
    }

    fn attribute_connect<T: IsA<CellRenderer>>(&self, renderer: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_area_attribute_connect(self.to_glib_none().0, renderer.to_glib_none().0, attribute.to_glib_none().0, column);
        }
    }

    fn attribute_disconnect<T: IsA<CellRenderer>>(&self, renderer: &T, attribute: &str) {
        unsafe {
            ffi::gtk_cell_area_attribute_disconnect(self.to_glib_none().0, renderer.to_glib_none().0, attribute.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_14)]
    fn attribute_get_column<T: IsA<CellRenderer>>(&self, renderer: &T, attribute: &str) -> i32 {
        unsafe {
            ffi::gtk_cell_area_attribute_get_column(self.to_glib_none().0, renderer.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    //fn cell_get<T: IsA<CellRenderer>>(&self, renderer: &T, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_get() }
    //}

    //fn cell_get_property<T: IsA<CellRenderer>>(&self, renderer: &T, property_name: &str, value: /*Ignored*/&mut gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_get_property() }
    //}

    //fn cell_get_valist<T: IsA<CellRenderer>>(&self, renderer: &T, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_get_valist() }
    //}

    //fn cell_set<T: IsA<CellRenderer>>(&self, renderer: &T, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_set() }
    //}

    //fn cell_set_property<T: IsA<CellRenderer>>(&self, renderer: &T, property_name: &str, value: /*Ignored*/&gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_set_property() }
    //}

    //fn cell_set_valist<T: IsA<CellRenderer>>(&self, renderer: &T, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_cell_area_cell_set_valist() }
    //}

    fn copy_context(&self, context: &CellAreaContext) -> Option<CellAreaContext> {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_copy_context(self.to_glib_none().0, context.to_glib_none().0))
        }
    }

    fn create_context(&self) -> Option<CellAreaContext> {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_create_context(self.to_glib_none().0))
        }
    }

    //fn event<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, event: /*Unknown conversion*//*Unimplemented*/Event, cell_area: &Rectangle, flags: CellRendererState) -> i32 {
    //    unsafe { TODO: call ffi::gtk_cell_area_event() }
    //}

    fn focus(&self, direction: DirectionType) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_focus(self.to_glib_none().0, direction))
        }
    }

    //fn foreach(&self, callback: /*Unknown conversion*//*Unimplemented*/CellCallback, callback_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_cell_area_foreach() }
    //}

    //fn foreach_alloc<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cell_area: &Rectangle, background_area: &Rectangle, callback: /*Unknown conversion*//*Unimplemented*/CellAllocCallback, callback_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_cell_area_foreach_alloc() }
    //}

    fn get_cell_allocation<T: IsA<Widget>, U: IsA<CellRenderer>>(&self, context: &CellAreaContext, widget: &T, renderer: &U, cell_area: &Rectangle) -> Rectangle {
        unsafe {
            let mut allocation = Rectangle::uninitialized();
            ffi::gtk_cell_area_get_cell_allocation(self.to_glib_none().0, context.to_glib_none().0, widget.to_glib_none().0, renderer.to_glib_none().0, cell_area.to_glib_none().0, allocation.to_glib_none_mut().0);
            allocation
        }
    }

    fn get_cell_at_position<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cell_area: &Rectangle, x: i32, y: i32) -> (CellRenderer, Rectangle) {
        unsafe {
            let mut alloc_area = Rectangle::uninitialized();
            let ret = from_glib_none(ffi::gtk_cell_area_get_cell_at_position(self.to_glib_none().0, context.to_glib_none().0, widget.to_glib_none().0, cell_area.to_glib_none().0, x, y, alloc_area.to_glib_none_mut().0));
            (ret, alloc_area)
        }
    }

    fn get_current_path_string(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_current_path_string(self.to_glib_none().0))
        }
    }

    fn get_edit_widget(&self) -> Option<CellEditable> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edit_widget(self.to_glib_none().0))
        }
    }

    fn get_edited_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edited_cell(self.to_glib_none().0))
        }
    }

    fn get_focus_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_cell(self.to_glib_none().0))
        }
    }

    fn get_focus_from_sibling<T: IsA<CellRenderer>>(&self, renderer: &T) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_from_sibling(self.to_glib_none().0, renderer.to_glib_none().0))
        }
    }

    fn get_focus_siblings<T: IsA<CellRenderer>>(&self, renderer: &T) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_cell_area_get_focus_siblings(self.to_glib_none().0, renderer.to_glib_none().0))
        }
    }

    fn get_preferred_height<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_height(self.to_glib_none().0, context.to_glib_none().0, widget.to_glib_none().0, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    fn get_preferred_height_for_width<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, width: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_height_for_width(self.to_glib_none().0, context.to_glib_none().0, widget.to_glib_none().0, width, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    fn get_preferred_width<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_width(self.to_glib_none().0, context.to_glib_none().0, widget.to_glib_none().0, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    fn get_preferred_width_for_height<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, height: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_get_preferred_width_for_height(self.to_glib_none().0, context.to_glib_none().0, widget.to_glib_none().0, height, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    fn get_request_mode(&self) -> SizeRequestMode {
        unsafe {
            ffi::gtk_cell_area_get_request_mode(self.to_glib_none().0)
        }
    }

    fn has_renderer<T: IsA<CellRenderer>>(&self, renderer: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_has_renderer(self.to_glib_none().0, renderer.to_glib_none().0))
        }
    }

    fn inner_cell_area<T: IsA<Widget>>(&self, widget: &T, cell_area: &Rectangle) -> Rectangle {
        unsafe {
            let mut inner_area = Rectangle::uninitialized();
            ffi::gtk_cell_area_inner_cell_area(self.to_glib_none().0, widget.to_glib_none().0, cell_area.to_glib_none().0, inner_area.to_glib_none_mut().0);
            inner_area
        }
    }

    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_activatable(self.to_glib_none().0))
        }
    }

    fn is_focus_sibling<T: IsA<CellRenderer>, U: IsA<CellRenderer>>(&self, renderer: &T, sibling: &U) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_focus_sibling(self.to_glib_none().0, renderer.to_glib_none().0, sibling.to_glib_none().0))
        }
    }

    fn remove<T: IsA<CellRenderer>>(&self, renderer: &T) {
        unsafe {
            ffi::gtk_cell_area_remove(self.to_glib_none().0, renderer.to_glib_none().0);
        }
    }

    fn remove_focus_sibling<T: IsA<CellRenderer>, U: IsA<CellRenderer>>(&self, renderer: &T, sibling: &U) {
        unsafe {
            ffi::gtk_cell_area_remove_focus_sibling(self.to_glib_none().0, renderer.to_glib_none().0, sibling.to_glib_none().0);
        }
    }

    //fn render<T: IsA<Widget>>(&self, context: &CellAreaContext, widget: &T, cr: /*Ignored*/&mut cairo::Context, background_area: &Rectangle, cell_area: &Rectangle, flags: CellRendererState, paint_focus: bool) {
    //    unsafe { TODO: call ffi::gtk_cell_area_render() }
    //}

    fn request_renderer<T: IsA<CellRenderer>, U: IsA<Widget>>(&self, renderer: &T, orientation: Orientation, widget: &U, for_size: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::uninitialized();
            let mut natural_size = mem::uninitialized();
            ffi::gtk_cell_area_request_renderer(self.to_glib_none().0, renderer.to_glib_none().0, orientation, widget.to_glib_none().0, for_size, &mut minimum_size, &mut natural_size);
            (minimum_size, natural_size)
        }
    }

    fn set_focus_cell<T: IsA<CellRenderer>>(&self, renderer: &T) {
        unsafe {
            ffi::gtk_cell_area_set_focus_cell(self.to_glib_none().0, renderer.to_glib_none().0);
        }
    }

    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_area_stop_editing(self.to_glib_none().0, canceled.to_glib());
        }
    }
}
