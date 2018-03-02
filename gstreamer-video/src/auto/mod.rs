// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

mod video_filter;
pub use self::video_filter::VideoFilter;

mod video_overlay;
pub use self::video_overlay::VideoOverlay;
pub use self::video_overlay::VideoOverlayExt;

mod enums;
pub use self::enums::VideoColorMatrix;
pub use self::enums::VideoColorPrimaries;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use self::enums::VideoFieldOrder;
pub use self::enums::VideoFormat;
pub use self::enums::VideoInterlaceMode;
pub use self::enums::VideoMultiviewFramePacking;
pub use self::enums::VideoMultiviewMode;
pub use self::enums::VideoTileMode;
pub use self::enums::VideoTransferFunction;

mod flags;
pub use self::flags::VideoChromaSite;
pub use self::flags::VideoFlags;
pub use self::flags::VideoFormatFlags;
pub use self::flags::VideoFrameFlags;
pub use self::flags::VideoMultiviewFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::VideoOverlayExt;
}
