// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Object;
use Plugin;
use PluginFeature;
use ffi;
use glib;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Registry(Object<ffi::GstRegistry, ffi::GstRegistryClass>): Object;

    match fn {
        get_type => || ffi::gst_registry_get_type(),
    }
}

impl Registry {
    pub fn add_feature<P: IsA<PluginFeature>>(&self, feature: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_registry_add_feature(self.to_glib_none().0, feature.to_glib_none().0), "Failed to add feature")
        }
    }

    pub fn add_plugin(&self, plugin: &Plugin) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_registry_add_plugin(self.to_glib_none().0, plugin.to_glib_none().0), "Failed to add plugin")
        }
    }

    pub fn check_feature_version(&self, feature_name: &str, min_major: u32, min_minor: u32, min_micro: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_registry_check_feature_version(self.to_glib_none().0, feature_name.to_glib_none().0, min_major, min_minor, min_micro))
        }
    }

    //pub fn feature_filter<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, filter: /*Unknown conversion*//*Unimplemented*/PluginFeatureFilter, first: bool, user_data: P) -> Vec<PluginFeature> {
    //    unsafe { TODO: call ffi::gst_registry_feature_filter() }
    //}

    pub fn find_feature(&self, name: &str, type_: glib::types::Type) -> Option<PluginFeature> {
        unsafe {
            from_glib_full(ffi::gst_registry_find_feature(self.to_glib_none().0, name.to_glib_none().0, type_.to_glib()))
        }
    }

    pub fn find_plugin(&self, name: &str) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_registry_find_plugin(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn get_feature_list(&self, type_: glib::types::Type) -> Vec<PluginFeature> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_get_feature_list(self.to_glib_none().0, type_.to_glib()))
        }
    }

    pub fn get_feature_list_by_plugin(&self, name: &str) -> Vec<PluginFeature> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_get_feature_list_by_plugin(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn get_feature_list_cookie(&self) -> u32 {
        unsafe {
            ffi::gst_registry_get_feature_list_cookie(self.to_glib_none().0)
        }
    }

    pub fn get_plugin_list(&self) -> Vec<Plugin> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_get_plugin_list(self.to_glib_none().0))
        }
    }

    pub fn lookup(&self, filename: &str) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_registry_lookup(self.to_glib_none().0, filename.to_glib_none().0))
        }
    }

    pub fn lookup_feature(&self, name: &str) -> Option<PluginFeature> {
        unsafe {
            from_glib_full(ffi::gst_registry_lookup_feature(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    //pub fn plugin_filter<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, filter: /*Unknown conversion*//*Unimplemented*/PluginFilter, first: bool, user_data: P) -> Vec<Plugin> {
    //    unsafe { TODO: call ffi::gst_registry_plugin_filter() }
    //}

    pub fn remove_feature<P: IsA<PluginFeature>>(&self, feature: &P) {
        unsafe {
            ffi::gst_registry_remove_feature(self.to_glib_none().0, feature.to_glib_none().0);
        }
    }

    pub fn remove_plugin(&self, plugin: &Plugin) {
        unsafe {
            ffi::gst_registry_remove_plugin(self.to_glib_none().0, plugin.to_glib_none().0);
        }
    }

    pub fn scan_path<P: AsRef<std::path::Path>>(&self, path: P) -> bool {
        unsafe {
            from_glib(ffi::gst_registry_scan_path(self.to_glib_none().0, path.as_ref().to_glib_none().0))
        }
    }

    pub fn get() -> Registry {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_registry_get())
        }
    }

    pub fn connect_feature_added<F: Fn(&Registry, &PluginFeature) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Registry, &PluginFeature) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "feature-added",
                transmute(feature_added_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_plugin_added<F: Fn(&Registry, &Plugin) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Registry, &Plugin) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "plugin-added",
                transmute(plugin_added_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for Registry {}
unsafe impl Sync for Registry {}

unsafe extern "C" fn feature_added_trampoline(this: *mut ffi::GstRegistry, feature: *mut ffi::GstPluginFeature, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Registry, &PluginFeature) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(feature))
}

unsafe extern "C" fn plugin_added_trampoline(this: *mut ffi::GstRegistry, plugin: *mut ffi::GstPlugin, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Registry, &Plugin) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(plugin))
}
