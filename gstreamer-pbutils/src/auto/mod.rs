// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod audio_visualizer;
pub use self::audio_visualizer::AudioVisualizer;

mod discoverer;
pub use self::discoverer::Discoverer;

mod discoverer_audio_info;
pub use self::discoverer_audio_info::DiscovererAudioInfo;

mod discoverer_container_info;
pub use self::discoverer_container_info::DiscovererContainerInfo;

mod discoverer_info;
pub use self::discoverer_info::DiscovererInfo;

mod discoverer_stream_info;
pub use self::discoverer_stream_info::DiscovererStreamInfo;

mod discoverer_subtitle_info;
pub use self::discoverer_subtitle_info::DiscovererSubtitleInfo;

mod discoverer_video_info;
pub use self::discoverer_video_info::DiscovererVideoInfo;

mod encoding_audio_profile;
pub use self::encoding_audio_profile::EncodingAudioProfile;

mod encoding_container_profile;
pub use self::encoding_container_profile::EncodingContainerProfile;

mod encoding_profile;
pub use self::encoding_profile::EncodingProfile;

mod encoding_target;
pub use self::encoding_target::EncodingTarget;

mod encoding_video_profile;
pub use self::encoding_video_profile::EncodingVideoProfile;

mod install_plugins_context;
pub use self::install_plugins_context::InstallPluginsContext;

mod enums;
pub use self::enums::AudioVisualizerShader;
pub use self::enums::DiscovererResult;
pub use self::enums::InstallPluginsReturn;

mod flags;
pub use self::flags::DiscovererSerializeFlags;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub use self::flags::PbUtilsCapsDescriptionFlags;

pub(crate) mod functions;

pub(crate) mod traits {
    pub use super::audio_visualizer::AudioVisualizerExt;
    pub use super::discoverer_stream_info::DiscovererStreamInfoExt;
    pub use super::encoding_profile::EncodingProfileExt;
}
