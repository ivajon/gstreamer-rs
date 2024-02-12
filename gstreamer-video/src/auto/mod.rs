// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod color_balance;
pub use self::color_balance::ColorBalance;

mod color_balance_channel;
pub use self::color_balance_channel::ColorBalanceChannel;

mod navigation;
pub use self::navigation::Navigation;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_aggregator;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use self::video_aggregator::VideoAggregator;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_aggregator_convert_pad;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use self::video_aggregator_convert_pad::VideoAggregatorConvertPad;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_aggregator_pad;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use self::video_aggregator_pad::VideoAggregatorPad;

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
mod video_aggregator_parallel_convert_pad;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub use self::video_aggregator_parallel_convert_pad::VideoAggregatorParallelConvertPad;

mod video_buffer_pool;
pub use self::video_buffer_pool::VideoBufferPool;

mod video_decoder;
pub use self::video_decoder::VideoDecoder;

mod video_encoder;
pub use self::video_encoder::VideoEncoder;

mod video_filter;
pub use self::video_filter::VideoFilter;

mod video_orientation;
pub use self::video_orientation::VideoOrientation;

mod video_overlay;
pub use self::video_overlay::VideoOverlay;

mod video_sink;
pub use self::video_sink::VideoSink;

mod enums;
#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
pub use self::enums::AncillaryMetaField;
pub use self::enums::ColorBalanceType;
pub use self::enums::NavigationCommand;
pub use self::enums::NavigationEventType;
pub use self::enums::NavigationMessageType;
pub use self::enums::NavigationQueryType;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
pub use self::enums::VideoAFDSpec;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
pub use self::enums::VideoAFDValue;
pub use self::enums::VideoAlphaMode;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use self::enums::VideoAncillaryDID;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use self::enums::VideoAncillaryDID16;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use self::enums::VideoCaptionType;
pub use self::enums::VideoChromaMode;
pub use self::enums::VideoColorMatrix;
pub use self::enums::VideoColorPrimaries;
pub use self::enums::VideoDitherMethod;
pub use self::enums::VideoFieldOrder;
pub use self::enums::VideoFormat;
pub use self::enums::VideoGammaMode;
pub use self::enums::VideoInterlaceMode;
pub use self::enums::VideoMatrixMode;
pub use self::enums::VideoMultiviewFramePacking;
pub use self::enums::VideoMultiviewMode;
pub use self::enums::VideoOrientationMethod;
pub use self::enums::VideoPrimariesMode;
pub use self::enums::VideoResamplerMethod;
pub use self::enums::VideoTileMode;
pub use self::enums::VideoTransferFunction;

mod flags;
#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
pub use self::flags::NavigationModifierType;
pub use self::flags::VideoBufferFlags;
pub use self::flags::VideoChromaSite;
pub use self::flags::VideoCodecFrameFlags;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub use self::flags::VideoDecoderRequestSyncPointFlags;
pub use self::flags::VideoFlags;
pub use self::flags::VideoFormatFlags;
pub use self::flags::VideoFrameFlags;
pub use self::flags::VideoMultiviewFlags;
pub use self::flags::VideoOverlayFormatFlags;
pub use self::flags::VideoPackFlags;
pub use self::flags::VideoTimeCodeFlags;

pub(crate) mod traits {
    pub use super::color_balance::ColorBalanceExt;
    pub use super::color_balance_channel::ColorBalanceChannelExt;
    pub use super::navigation::NavigationExt;
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub use super::video_aggregator::VideoAggregatorExt;
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub use super::video_aggregator_convert_pad::VideoAggregatorConvertPadExt;
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub use super::video_aggregator_pad::VideoAggregatorPadExt;
    pub use super::video_decoder::VideoDecoderExt;
    pub use super::video_encoder::VideoEncoderExt;
    pub use super::video_orientation::VideoOrientationExt;
    pub use super::video_overlay::VideoOverlayExt;
    pub use super::video_sink::VideoSinkExt;
}
