// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use CellArea;
use CellLayout;
use TreeIter;
use TreeModel;
use Widget;
use ffi;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct EntryCompletion(Object<ffi::GtkEntryCompletion>): CellLayout;

    match fn {
        get_type => || ffi::gtk_entry_completion_get_type(),
    }
}

impl EntryCompletion {
    pub fn new() -> EntryCompletion {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new())
        }
    }

    pub fn new_with_area<T: IsA<CellArea>>(area: &T) -> EntryCompletion {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new_with_area(area.to_glib_none().0))
        }
    }

    pub fn complete(&self) {
        unsafe {
            ffi::gtk_entry_completion_complete(self.to_glib_none().0);
        }
    }

    pub fn compute_prefix(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_compute_prefix(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn delete_action(&self, index_: i32) {
        unsafe {
            ffi::gtk_entry_completion_delete_action(self.to_glib_none().0, index_);
        }
    }

    pub fn get_completion_prefix(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_completion_prefix(self.to_glib_none().0))
        }
    }

    pub fn get_entry(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_entry(self.to_glib_none().0))
        }
    }

    pub fn get_inline_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_completion(self.to_glib_none().0))
        }
    }

    pub fn get_inline_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_selection(self.to_glib_none().0))
        }
    }

    pub fn get_minimum_key_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_completion_get_minimum_key_length(self.to_glib_none().0)
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_model(self.to_glib_none().0))
        }
    }

    pub fn get_popup_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_completion(self.to_glib_none().0))
        }
    }

    pub fn get_popup_set_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_set_width(self.to_glib_none().0))
        }
    }

    pub fn get_popup_single_match(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_single_match(self.to_glib_none().0))
        }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_completion_get_text_column(self.to_glib_none().0)
        }
    }

    pub fn insert_action_markup(&self, index_: i32, markup: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_markup(self.to_glib_none().0, index_, markup.to_glib_none().0);
        }
    }

    pub fn insert_action_text(&self, index_: i32, text: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_text(self.to_glib_none().0, index_, text.to_glib_none().0);
        }
    }

    pub fn insert_prefix(&self) {
        unsafe {
            ffi::gtk_entry_completion_insert_prefix(self.to_glib_none().0);
        }
    }

    pub fn set_inline_completion(&self, inline_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_completion(self.to_glib_none().0, inline_completion.to_glib());
        }
    }

    pub fn set_inline_selection(&self, inline_selection: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_selection(self.to_glib_none().0, inline_selection.to_glib());
        }
    }

    //pub fn set_match_func(&self, func: /*Unknown conversion*//*Unimplemented*/EntryCompletionMatchFunc, func_data: /*Unimplemented*/Option<Fundamental: Pointer>, func_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_entry_completion_set_match_func() }
    //}

    pub fn set_minimum_key_length(&self, length: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_minimum_key_length(self.to_glib_none().0, length);
        }
    }

    pub fn set_model<T: IsA<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_entry_completion_set_model(self.to_glib_none().0, model.to_glib_none().0);
        }
    }

    pub fn set_popup_completion(&self, popup_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_completion(self.to_glib_none().0, popup_completion.to_glib());
        }
    }

    pub fn set_popup_set_width(&self, popup_set_width: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_set_width(self.to_glib_none().0, popup_set_width.to_glib());
        }
    }

    pub fn set_popup_single_match(&self, popup_single_match: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_single_match(self.to_glib_none().0, popup_single_match.to_glib());
        }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_text_column(self.to_glib_none().0, column);
        }
    }

    pub fn connect_action_activated<F: Fn(&EntryCompletion, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&EntryCompletion, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-activated",
                transmute(action_activated_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cursor_on_match<F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&EntryCompletion, &TreeModel, &TreeIter) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cursor-on-match",
                transmute(cursor_on_match_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_insert_prefix<F: Fn(&EntryCompletion, &str) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&EntryCompletion, &str) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-prefix",
                transmute(insert_prefix_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_match_selected<F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&EntryCompletion, &TreeModel, &TreeIter) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "match-selected",
                transmute(match_selected_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_no_matches<F: Fn(&EntryCompletion) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&EntryCompletion) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "no-matches",
                transmute(no_matches_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn action_activated_trampoline(this: *mut ffi::GtkEntryCompletion, index: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&EntryCompletion, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), index)
}

unsafe extern "C" fn cursor_on_match_trampoline(this: *mut ffi::GtkEntryCompletion, model: *mut ffi::GtkTreeModel, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&EntryCompletion, &TreeModel, &TreeIter) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(model), &from_glib_none(iter)).to_glib()
}

unsafe extern "C" fn insert_prefix_trampoline(this: *mut ffi::GtkEntryCompletion, prefix: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&EntryCompletion, &str) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(prefix)).to_glib()
}

unsafe extern "C" fn match_selected_trampoline(this: *mut ffi::GtkEntryCompletion, model: *mut ffi::GtkTreeModel, iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&EntryCompletion, &TreeModel, &TreeIter) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(model), &from_glib_none(iter)).to_glib()
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn no_matches_trampoline(this: *mut ffi::GtkEntryCompletion, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&EntryCompletion) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
