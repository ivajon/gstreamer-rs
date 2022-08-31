// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::BinMonitor;
use crate::ElementMonitor;
use crate::Monitor;
use crate::Reporter;
use crate::Runner;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstValidatePipelineMonitor")]
    pub struct PipelineMonitor(Object<ffi::GstValidatePipelineMonitor, ffi::GstValidatePipelineMonitorClass>) @extends BinMonitor, ElementMonitor, Monitor, gst::Object, @implements Reporter;

    match fn {
        type_ => || ffi::gst_validate_pipeline_monitor_get_type(),
    }
}

impl PipelineMonitor {
    pub const NONE: Option<&'static PipelineMonitor> = None;

    #[doc(alias = "gst_validate_pipeline_monitor_new")]
    pub fn new(
        pipeline: &impl IsA<gst::Pipeline>,
        runner: &impl IsA<Runner>,
        parent: Option<&impl IsA<Monitor>>,
    ) -> PipelineMonitor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_validate_pipeline_monitor_new(
                pipeline.as_ref().to_glib_none().0,
                runner.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for PipelineMonitor {}
unsafe impl Sync for PipelineMonitor {}
