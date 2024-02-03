// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gstreamer_gl_sys as gst_gl;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayWaylandClass {
    pub object_class: gst_gl::GstGLDisplayClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayWaylandClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayWaylandClass @ {self:p}"))
            .field("object_class", &self.object_class)
            .field("_padding", &self._padding)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayWayland {
    pub parent: gst_gl::GstGLDisplay,
    pub display: gpointer,
    pub registry: gpointer,
    pub compositor: gpointer,
    pub subcompositor: gpointer,
    pub shell: gpointer,
    pub foreign_display: gboolean,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayWayland {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayWayland @ {self:p}"))
            .field("parent", &self.parent)
            .field("display", &self.display)
            .field("registry", &self.registry)
            .field("compositor", &self.compositor)
            .field("subcompositor", &self.subcompositor)
            .field("shell", &self.shell)
            .finish()
    }
}

#[link(name = "gstgl-1.0")]
extern "C" {

    //=========================================================================
    // GstGLDisplayWayland
    //=========================================================================
    pub fn gst_gl_display_wayland_get_type() -> GType;
    pub fn gst_gl_display_wayland_new(name: *const c_char) -> *mut GstGLDisplayWayland;
    pub fn gst_gl_display_wayland_new_with_display(display: gpointer) -> *mut GstGLDisplayWayland;

}
