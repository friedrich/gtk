// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use CellArea;
use CellLayout;
use CellRenderer;
use Container;
use IconViewDropPosition;
use MovementStep;
use Orientation;
use Scrollable;
use SelectionMode;
use Tooltip;
use TreeIter;
use TreeModel;
use TreePath;
use Widget;
use cairo;
use ffi;
#[cfg(feature = "v3_6")]
use gdk;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct IconView(Object<ffi::GtkIconView>): Container, Widget, CellLayout, Scrollable;

    match fn {
        get_type => || ffi::gtk_icon_view_get_type(),
    }
}

impl IconView {
    pub fn new() -> IconView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_area<T: IsA<CellArea>>(area: &T) -> IconView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new_with_area(area.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_model<T: IsA<TreeModel>>(model: &T) -> IconView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new_with_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn convert_widget_to_bin_window_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        unsafe {
            let mut bx = mem::uninitialized();
            let mut by = mem::uninitialized();
            ffi::gtk_icon_view_convert_widget_to_bin_window_coords(self.to_glib_none().0, wx, wy, &mut bx, &mut by);
            (bx, by)
        }
    }

    pub fn create_drag_icon(&self, path: &TreePath) -> Option<cairo::Surface> {
        unsafe {
            from_glib_full(ffi::gtk_icon_view_create_drag_icon(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    //pub fn enable_model_drag_dest(&self, targets: /*Ignored*/&[&TargetEntry], n_targets: i32, actions: /*Ignored*/gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_icon_view_enable_model_drag_dest() }
    //}

    //pub fn enable_model_drag_source(&self, start_button_mask: gdk::ModifierType, targets: /*Ignored*/&[&TargetEntry], n_targets: i32, actions: /*Ignored*/gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_icon_view_enable_model_drag_source() }
    //}

    #[cfg(feature = "v3_8")]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_cell_rect<T: IsA<CellRenderer>>(&self, path: &TreePath, cell: Option<&T>) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_icon_view_get_cell_rect(self.to_glib_none().0, mut_override(path.to_glib_none().0), cell.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    pub fn get_column_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_column_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_columns(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_columns(self.to_glib_none().0)
        }
    }

    pub fn get_cursor(&self) -> Option<(TreePath, CellRenderer)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut cell = ptr::null_mut();
            let ret = from_glib(ffi::gtk_icon_view_get_cursor(self.to_glib_none().0, &mut path, &mut cell));
            if ret { Some((from_glib_full(path), from_glib_none(cell))) } else { None }
        }
    }

    pub fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32) -> Option<(TreePath, IconViewDropPosition)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            let ret = from_glib(ffi::gtk_icon_view_get_dest_item_at_pos(self.to_glib_none().0, drag_x, drag_y, &mut path, &mut pos));
            if ret { Some((from_glib_full(path), from_glib(pos))) } else { None }
        }
    }

    pub fn get_drag_dest_item(&self) -> (TreePath, IconViewDropPosition) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            ffi::gtk_icon_view_get_drag_dest_item(self.to_glib_none().0, &mut path, &mut pos);
            (from_glib_full(path), from_glib(pos))
        }
    }

    pub fn get_item_at_pos(&self, x: i32, y: i32) -> Option<(TreePath, CellRenderer)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut cell = ptr::null_mut();
            let ret = from_glib(ffi::gtk_icon_view_get_item_at_pos(self.to_glib_none().0, x, y, &mut path, &mut cell));
            if ret { Some((from_glib_full(path), from_glib_full(cell))) } else { None }
        }
    }

    pub fn get_item_column(&self, path: &TreePath) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_column(self.to_glib_none().0, mut_override(path.to_glib_none().0))
        }
    }

    pub fn get_item_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_item_orientation(self.to_glib_none().0))
        }
    }

    pub fn get_item_padding(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_padding(self.to_glib_none().0)
        }
    }

    pub fn get_item_row(&self, path: &TreePath) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_row(self.to_glib_none().0, mut_override(path.to_glib_none().0))
        }
    }

    pub fn get_item_width(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_width(self.to_glib_none().0)
        }
    }

    pub fn get_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_margin(self.to_glib_none().0)
        }
    }

    pub fn get_markup_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_markup_column(self.to_glib_none().0)
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_icon_view_get_model(self.to_glib_none().0))
        }
    }

    pub fn get_path_at_pos(&self, x: i32, y: i32) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_icon_view_get_path_at_pos(self.to_glib_none().0, x, y))
        }
    }

    pub fn get_pixbuf_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_pixbuf_column(self.to_glib_none().0)
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_reorderable(self.to_glib_none().0))
        }
    }

    pub fn get_row_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_row_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_selected_items(&self) -> Vec<TreePath> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_icon_view_get_selected_items(self.to_glib_none().0))
        }
    }

    pub fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_selection_mode(self.to_glib_none().0))
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_text_column(self.to_glib_none().0)
        }
    }

    pub fn get_tooltip_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_tooltip_column(self.to_glib_none().0)
        }
    }

    pub fn get_tooltip_context(&self, x: &mut i32, y: &mut i32, keyboard_tip: bool) -> Option<(TreeModel, TreePath, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut path = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_icon_view_get_tooltip_context(self.to_glib_none().0, x, y, keyboard_tip.to_glib(), &mut model, &mut path, iter.to_glib_none_mut().0));
            if ret { Some((from_glib_none(model), from_glib_full(path), iter)) } else { None }
        }
    }

    pub fn get_visible_range(&self) -> Option<(TreePath, TreePath)> {
        unsafe {
            let mut start_path = ptr::null_mut();
            let mut end_path = ptr::null_mut();
            let ret = from_glib(ffi::gtk_icon_view_get_visible_range(self.to_glib_none().0, &mut start_path, &mut end_path));
            if ret { Some((from_glib_full(start_path), from_glib_full(end_path))) } else { None }
        }
    }

    pub fn item_activated(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_item_activated(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_path_is_selected(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    pub fn scroll_to_path(&self, path: &TreePath, use_align: bool, row_align: f32, col_align: f32) {
        unsafe {
            ffi::gtk_icon_view_scroll_to_path(self.to_glib_none().0, mut_override(path.to_glib_none().0), use_align.to_glib(), row_align, col_align);
        }
    }

    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_icon_view_select_all(self.to_glib_none().0);
        }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_select_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    //pub fn selected_foreach(&self, func: /*Unknown conversion*//*Unimplemented*/IconViewForeachFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_icon_view_selected_foreach() }
    //}

    #[cfg(feature = "v3_8")]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_icon_view_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    pub fn set_column_spacing(&self, column_spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_column_spacing(self.to_glib_none().0, column_spacing);
        }
    }

    pub fn set_columns(&self, columns: i32) {
        unsafe {
            ffi::gtk_icon_view_set_columns(self.to_glib_none().0, columns);
        }
    }

    pub fn set_cursor<T: IsA<CellRenderer>>(&self, path: &TreePath, cell: Option<&T>, start_editing: bool) {
        unsafe {
            ffi::gtk_icon_view_set_cursor(self.to_glib_none().0, mut_override(path.to_glib_none().0), cell.to_glib_none().0, start_editing.to_glib());
        }
    }

    pub fn set_drag_dest_item(&self, path: Option<&TreePath>, pos: IconViewDropPosition) {
        unsafe {
            ffi::gtk_icon_view_set_drag_dest_item(self.to_glib_none().0, mut_override(path.to_glib_none().0), pos.to_glib());
        }
    }

    pub fn set_item_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_icon_view_set_item_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    pub fn set_item_padding(&self, item_padding: i32) {
        unsafe {
            ffi::gtk_icon_view_set_item_padding(self.to_glib_none().0, item_padding);
        }
    }

    pub fn set_item_width(&self, item_width: i32) {
        unsafe {
            ffi::gtk_icon_view_set_item_width(self.to_glib_none().0, item_width);
        }
    }

    pub fn set_margin(&self, margin: i32) {
        unsafe {
            ffi::gtk_icon_view_set_margin(self.to_glib_none().0, margin);
        }
    }

    pub fn set_markup_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_markup_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_model<T: IsA<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_icon_view_set_model(self.to_glib_none().0, model.to_glib_none().0);
        }
    }

    pub fn set_pixbuf_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_pixbuf_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_icon_view_set_reorderable(self.to_glib_none().0, reorderable.to_glib());
        }
    }

    pub fn set_row_spacing(&self, row_spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_row_spacing(self.to_glib_none().0, row_spacing);
        }
    }

    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_icon_view_set_selection_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_text_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_tooltip_cell<T: IsA<CellRenderer>>(&self, tooltip: &Tooltip, path: &TreePath, cell: Option<&T>) {
        unsafe {
            ffi::gtk_icon_view_set_tooltip_cell(self.to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0), cell.to_glib_none().0);
        }
    }

    pub fn set_tooltip_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_tooltip_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_tooltip_item(&self, tooltip: &Tooltip, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_set_tooltip_item(self.to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_icon_view_unselect_all(self.to_glib_none().0);
        }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_unselect_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    pub fn unset_model_drag_dest(&self) {
        unsafe {
            ffi::gtk_icon_view_unset_model_drag_dest(self.to_glib_none().0);
        }
    }

    pub fn unset_model_drag_source(&self) {
        unsafe {
            ffi::gtk_icon_view_unset_model_drag_source(self.to_glib_none().0);
        }
    }

    pub fn connect_activate_cursor_item<F: Fn(&IconView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-cursor-item",
                transmute(activate_cursor_item_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_item_activated<F: Fn(&IconView, &TreePath) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView, &TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "item-activated",
                transmute(item_activated_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_cursor<F: Fn(&IconView, MovementStep, i32) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView, MovementStep, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_select_all<F: Fn(&IconView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-all",
                transmute(select_all_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_select_cursor_item<F: Fn(&IconView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-cursor-item",
                transmute(select_cursor_item_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_selection_changed<F: Fn(&IconView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_toggle_cursor_item<F: Fn(&IconView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-cursor-item",
                transmute(toggle_cursor_item_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_unselect_all<F: Fn(&IconView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&IconView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unselect-all",
                transmute(unselect_all_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_cursor_item_trampoline(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&IconView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn item_activated_trampoline(this: *mut ffi::GtkIconView, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&IconView, &TreePath) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(path))
}

unsafe extern "C" fn move_cursor_trampoline(this: *mut ffi::GtkIconView, step: ffi::GtkMovementStep, count: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&IconView, MovementStep, i32) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(step), count).to_glib()
}

unsafe extern "C" fn select_all_trampoline(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&IconView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn select_cursor_item_trampoline(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&IconView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn selection_changed_trampoline(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&IconView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn toggle_cursor_item_trampoline(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&IconView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn unselect_all_trampoline(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&IconView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
