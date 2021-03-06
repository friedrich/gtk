// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use AppChooser;
use Box;
use Container;
use Menu;
use Orientable;
use Widget;
use ffi;
use gio;
use gio_ffi;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct AppChooserWidget(Object<ffi::GtkAppChooserWidget>): Box, Container, Widget, Orientable, AppChooser;

    match fn {
        get_type => || ffi::gtk_app_chooser_widget_get_type(),
    }
}

impl AppChooserWidget {
    pub fn new(content_type: &str) -> AppChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_widget_new(content_type.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_default_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_widget_get_default_text(self.to_glib_none().0))
        }
    }

    pub fn get_show_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_all(self.to_glib_none().0))
        }
    }

    pub fn get_show_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_default(self.to_glib_none().0))
        }
    }

    pub fn get_show_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_fallback(self.to_glib_none().0))
        }
    }

    pub fn get_show_other(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_other(self.to_glib_none().0))
        }
    }

    pub fn get_show_recommended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_recommended(self.to_glib_none().0))
        }
    }

    pub fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_show_all(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_all(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_default(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_fallback(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_fallback(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_other(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_other(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_show_recommended(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_recommended(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn connect_application_activated<F: Fn(&AppChooserWidget, &gio::AppInfo) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&AppChooserWidget, &gio::AppInfo) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "application-activated",
                transmute(application_activated_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_application_selected<F: Fn(&AppChooserWidget, &gio::AppInfo) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&AppChooserWidget, &gio::AppInfo) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "application-selected",
                transmute(application_selected_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_populate_popup<F: Fn(&AppChooserWidget, &Menu, &gio::AppInfo) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&AppChooserWidget, &Menu, &gio::AppInfo) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "populate-popup",
                transmute(populate_popup_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn application_activated_trampoline(this: *mut ffi::GtkAppChooserWidget, application: *mut gio_ffi::GAppInfo, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&AppChooserWidget, &gio::AppInfo) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(application))
}

unsafe extern "C" fn application_selected_trampoline(this: *mut ffi::GtkAppChooserWidget, application: *mut gio_ffi::GAppInfo, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&AppChooserWidget, &gio::AppInfo) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(application))
}

unsafe extern "C" fn populate_popup_trampoline(this: *mut ffi::GtkAppChooserWidget, menu: *mut ffi::GtkMenu, application: *mut gio_ffi::GAppInfo, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&AppChooserWidget, &Menu, &gio::AppInfo) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(menu), &from_glib_none(application))
}
