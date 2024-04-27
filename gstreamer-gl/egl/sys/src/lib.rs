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
    intptr_t, off_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Constants
pub const GST_GL_DISPLAY_EGL_NAME: &[u8] = b"gst.gl.display.egl\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayEGLClass {
    pub object_class: gst_gl::GstGLDisplayClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayEGLClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayEGLClass @ {self:p}"))
            .field("object_class", &self.object_class)
            .field("_padding", &self._padding)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayEGL {
    pub parent: gst_gl::GstGLDisplay,
    pub display: gpointer,
    pub foreign_display: gboolean,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayEGL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayEGL @ {self:p}"))
            .field("parent", &self.parent)
            .finish()
    }
}

#[link(name = "gstgl-1.0")]
extern "C" {

    //=========================================================================
    // GstGLDisplayEGL
    //=========================================================================
    pub fn gst_gl_display_egl_get_type() -> GType;
    pub fn gst_gl_display_egl_new() -> *mut GstGLDisplayEGL;
    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    pub fn gst_gl_display_egl_new_surfaceless() -> *mut GstGLDisplayEGL;
    pub fn gst_gl_display_egl_new_with_egl_display(display: gpointer) -> *mut GstGLDisplayEGL;
    pub fn gst_gl_display_egl_from_gl_display(
        display: *mut gst_gl::GstGLDisplay,
    ) -> *mut GstGLDisplayEGL;
    pub fn gst_gl_display_egl_get_from_native(
        type_: gst_gl::GstGLDisplayType,
        display: uintptr_t,
    ) -> gpointer;

}
