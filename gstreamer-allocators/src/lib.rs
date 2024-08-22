// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;

macro_rules! assert_initialized_main_thread {
    () => {
        let initiated = gst::INITIALIZED.lock().expect("Initiated lock is poisoned");
        if !*initiated {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

mod caps_features;
pub use crate::caps_features::CAPS_FEATURES_MEMORY_DMABUF;

mod fd_allocator;
pub use fd_allocator::*;

#[cfg(any(target_os = "linux", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
mod dma_buf_allocator;
#[cfg(any(target_os = "linux", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
pub use dma_buf_allocator::*;

#[cfg(any(all(feature = "v1_24", target_os = "linux"), docsrs))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v1_24", target_os = "linux"))))]
mod drm_dumb_allocator;
#[cfg(any(all(feature = "v1_24", target_os = "linux"), docsrs))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v1_24", target_os = "linux"))))]
pub use drm_dumb_allocator::*;

#[cfg(any(all(feature = "v1_24", unix), docsrs))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v1_24", unix))))]
mod shm_allocator;

mod phys_memory;
pub use phys_memory::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_base::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;
}

pub mod subclass;
