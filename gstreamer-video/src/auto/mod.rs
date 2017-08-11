// This file was generated by gir (cf27827) from gir-files (???)
// DO NOT EDIT

mod enums;
pub use self::enums::VideoColorMatrix;
pub use self::enums::VideoColorPrimaries;
#[cfg(feature = "v1_12")]
pub use self::enums::VideoFieldOrder;
pub use self::enums::VideoFormat;
pub use self::enums::VideoInterlaceMode;
pub use self::enums::VideoMultiviewMode;
pub use self::enums::VideoTileMode;
pub use self::enums::VideoTransferFunction;

mod flags;
pub use self::flags::VideoChromaSite;
pub use self::flags::VIDEO_CHROMA_SITE_UNKNOWN;
pub use self::flags::VIDEO_CHROMA_SITE_NONE;
pub use self::flags::VIDEO_CHROMA_SITE_H_COSITED;
pub use self::flags::VIDEO_CHROMA_SITE_V_COSITED;
pub use self::flags::VIDEO_CHROMA_SITE_ALT_LINE;
pub use self::flags::VIDEO_CHROMA_SITE_COSITED;
pub use self::flags::VIDEO_CHROMA_SITE_JPEG;
pub use self::flags::VIDEO_CHROMA_SITE_MPEG2;
pub use self::flags::VIDEO_CHROMA_SITE_DV;
pub use self::flags::VideoFlags;
pub use self::flags::VIDEO_FLAG_NONE;
pub use self::flags::VIDEO_FLAG_VARIABLE_FPS;
pub use self::flags::VIDEO_FLAG_PREMULTIPLIED_ALPHA;
pub use self::flags::VideoFormatFlags;
pub use self::flags::VIDEO_FORMAT_FLAG_YUV;
pub use self::flags::VIDEO_FORMAT_FLAG_RGB;
pub use self::flags::VIDEO_FORMAT_FLAG_GRAY;
pub use self::flags::VIDEO_FORMAT_FLAG_ALPHA;
pub use self::flags::VIDEO_FORMAT_FLAG_LE;
pub use self::flags::VIDEO_FORMAT_FLAG_PALETTE;
pub use self::flags::VIDEO_FORMAT_FLAG_COMPLEX;
pub use self::flags::VIDEO_FORMAT_FLAG_UNPACK;
pub use self::flags::VIDEO_FORMAT_FLAG_TILED;
pub use self::flags::VideoFrameFlags;
pub use self::flags::VIDEO_FRAME_FLAG_NONE;
pub use self::flags::VIDEO_FRAME_FLAG_INTERLACED;
pub use self::flags::VIDEO_FRAME_FLAG_TFF;
pub use self::flags::VIDEO_FRAME_FLAG_RFF;
pub use self::flags::VIDEO_FRAME_FLAG_ONEFIELD;
pub use self::flags::VIDEO_FRAME_FLAG_MULTIPLE_VIEW;
pub use self::flags::VIDEO_FRAME_FLAG_FIRST_IN_BUNDLE;
pub use self::flags::VideoMultiviewFlags;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_NONE;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT;
pub use self::flags::VIDEO_MULTIVIEW_FLAGS_MIXED_MONO;

#[doc(hidden)]
pub mod traits {
}
