// Generated by gir (https://github.com/gtk-rs/gir @ 54e116a11822)
// from gir-files (https://github.com/gtk-rs/gir-files @ df20f22974b6)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ b73d2b6fbead)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GstWebRTCBundlePolicy = c_int;
pub const GST_WEBRTC_BUNDLE_POLICY_NONE: GstWebRTCBundlePolicy = 0;
pub const GST_WEBRTC_BUNDLE_POLICY_BALANCED: GstWebRTCBundlePolicy = 1;
pub const GST_WEBRTC_BUNDLE_POLICY_MAX_COMPAT: GstWebRTCBundlePolicy = 2;
pub const GST_WEBRTC_BUNDLE_POLICY_MAX_BUNDLE: GstWebRTCBundlePolicy = 3;

pub type GstWebRTCDTLSSetup = c_int;
pub const GST_WEBRTC_DTLS_SETUP_NONE: GstWebRTCDTLSSetup = 0;
pub const GST_WEBRTC_DTLS_SETUP_ACTPASS: GstWebRTCDTLSSetup = 1;
pub const GST_WEBRTC_DTLS_SETUP_ACTIVE: GstWebRTCDTLSSetup = 2;
pub const GST_WEBRTC_DTLS_SETUP_PASSIVE: GstWebRTCDTLSSetup = 3;

pub type GstWebRTCDTLSTransportState = c_int;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_NEW: GstWebRTCDTLSTransportState = 0;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_CLOSED: GstWebRTCDTLSTransportState = 1;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_FAILED: GstWebRTCDTLSTransportState = 2;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTING: GstWebRTCDTLSTransportState = 3;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTED: GstWebRTCDTLSTransportState = 4;

pub type GstWebRTCDataChannelState = c_int;
pub const GST_WEBRTC_DATA_CHANNEL_STATE_CONNECTING: GstWebRTCDataChannelState = 1;
pub const GST_WEBRTC_DATA_CHANNEL_STATE_OPEN: GstWebRTCDataChannelState = 2;
pub const GST_WEBRTC_DATA_CHANNEL_STATE_CLOSING: GstWebRTCDataChannelState = 3;
pub const GST_WEBRTC_DATA_CHANNEL_STATE_CLOSED: GstWebRTCDataChannelState = 4;

pub type GstWebRTCError = c_int;
pub const GST_WEBRTC_ERROR_DATA_CHANNEL_FAILURE: GstWebRTCError = 0;
pub const GST_WEBRTC_ERROR_DTLS_FAILURE: GstWebRTCError = 1;
pub const GST_WEBRTC_ERROR_FINGERPRINT_FAILURE: GstWebRTCError = 2;
pub const GST_WEBRTC_ERROR_SCTP_FAILURE: GstWebRTCError = 3;
pub const GST_WEBRTC_ERROR_SDP_SYNTAX_ERROR: GstWebRTCError = 4;
pub const GST_WEBRTC_ERROR_HARDWARE_ENCODER_NOT_AVAILABLE: GstWebRTCError = 5;
pub const GST_WEBRTC_ERROR_ENCODER_ERROR: GstWebRTCError = 6;
pub const GST_WEBRTC_ERROR_INVALID_STATE: GstWebRTCError = 7;
pub const GST_WEBRTC_ERROR_INTERNAL_FAILURE: GstWebRTCError = 8;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
pub const GST_WEBRTC_ERROR_INVALID_MODIFICATION: GstWebRTCError = 9;

pub type GstWebRTCFECType = c_int;
pub const GST_WEBRTC_FEC_TYPE_NONE: GstWebRTCFECType = 0;
pub const GST_WEBRTC_FEC_TYPE_ULP_RED: GstWebRTCFECType = 1;

pub type GstWebRTCICEComponent = c_int;
pub const GST_WEBRTC_ICE_COMPONENT_RTP: GstWebRTCICEComponent = 0;
pub const GST_WEBRTC_ICE_COMPONENT_RTCP: GstWebRTCICEComponent = 1;

pub type GstWebRTCICEConnectionState = c_int;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_NEW: GstWebRTCICEConnectionState = 0;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_CHECKING: GstWebRTCICEConnectionState = 1;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_CONNECTED: GstWebRTCICEConnectionState = 2;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_COMPLETED: GstWebRTCICEConnectionState = 3;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_FAILED: GstWebRTCICEConnectionState = 4;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_DISCONNECTED: GstWebRTCICEConnectionState = 5;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_CLOSED: GstWebRTCICEConnectionState = 6;

pub type GstWebRTCICEGatheringState = c_int;
pub const GST_WEBRTC_ICE_GATHERING_STATE_NEW: GstWebRTCICEGatheringState = 0;
pub const GST_WEBRTC_ICE_GATHERING_STATE_GATHERING: GstWebRTCICEGatheringState = 1;
pub const GST_WEBRTC_ICE_GATHERING_STATE_COMPLETE: GstWebRTCICEGatheringState = 2;

pub type GstWebRTCICERole = c_int;
pub const GST_WEBRTC_ICE_ROLE_CONTROLLED: GstWebRTCICERole = 0;
pub const GST_WEBRTC_ICE_ROLE_CONTROLLING: GstWebRTCICERole = 1;

pub type GstWebRTCICETransportPolicy = c_int;
pub const GST_WEBRTC_ICE_TRANSPORT_POLICY_ALL: GstWebRTCICETransportPolicy = 0;
pub const GST_WEBRTC_ICE_TRANSPORT_POLICY_RELAY: GstWebRTCICETransportPolicy = 1;

pub type GstWebRTCKind = c_int;
pub const GST_WEBRTC_KIND_UNKNOWN: GstWebRTCKind = 0;
pub const GST_WEBRTC_KIND_AUDIO: GstWebRTCKind = 1;
pub const GST_WEBRTC_KIND_VIDEO: GstWebRTCKind = 2;

pub type GstWebRTCPeerConnectionState = c_int;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_NEW: GstWebRTCPeerConnectionState = 0;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTING: GstWebRTCPeerConnectionState = 1;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTED: GstWebRTCPeerConnectionState = 2;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_DISCONNECTED: GstWebRTCPeerConnectionState = 3;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_FAILED: GstWebRTCPeerConnectionState = 4;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_CLOSED: GstWebRTCPeerConnectionState = 5;

pub type GstWebRTCPriorityType = c_int;
pub const GST_WEBRTC_PRIORITY_TYPE_VERY_LOW: GstWebRTCPriorityType = 1;
pub const GST_WEBRTC_PRIORITY_TYPE_LOW: GstWebRTCPriorityType = 2;
pub const GST_WEBRTC_PRIORITY_TYPE_MEDIUM: GstWebRTCPriorityType = 3;
pub const GST_WEBRTC_PRIORITY_TYPE_HIGH: GstWebRTCPriorityType = 4;

pub type GstWebRTCRTPTransceiverDirection = c_int;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_NONE: GstWebRTCRTPTransceiverDirection = 0;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_INACTIVE: GstWebRTCRTPTransceiverDirection = 1;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDONLY: GstWebRTCRTPTransceiverDirection = 2;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_RECVONLY: GstWebRTCRTPTransceiverDirection = 3;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDRECV: GstWebRTCRTPTransceiverDirection = 4;

pub type GstWebRTCSCTPTransportState = c_int;
pub const GST_WEBRTC_SCTP_TRANSPORT_STATE_NEW: GstWebRTCSCTPTransportState = 0;
pub const GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTING: GstWebRTCSCTPTransportState = 1;
pub const GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTED: GstWebRTCSCTPTransportState = 2;
pub const GST_WEBRTC_SCTP_TRANSPORT_STATE_CLOSED: GstWebRTCSCTPTransportState = 3;

pub type GstWebRTCSDPType = c_int;
pub const GST_WEBRTC_SDP_TYPE_OFFER: GstWebRTCSDPType = 1;
pub const GST_WEBRTC_SDP_TYPE_PRANSWER: GstWebRTCSDPType = 2;
pub const GST_WEBRTC_SDP_TYPE_ANSWER: GstWebRTCSDPType = 3;
pub const GST_WEBRTC_SDP_TYPE_ROLLBACK: GstWebRTCSDPType = 4;

pub type GstWebRTCSignalingState = c_int;
pub const GST_WEBRTC_SIGNALING_STATE_STABLE: GstWebRTCSignalingState = 0;
pub const GST_WEBRTC_SIGNALING_STATE_CLOSED: GstWebRTCSignalingState = 1;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_OFFER: GstWebRTCSignalingState = 2;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_OFFER: GstWebRTCSignalingState = 3;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_PRANSWER: GstWebRTCSignalingState = 4;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_PRANSWER: GstWebRTCSignalingState = 5;

pub type GstWebRTCStatsType = c_int;
pub const GST_WEBRTC_STATS_CODEC: GstWebRTCStatsType = 1;
pub const GST_WEBRTC_STATS_INBOUND_RTP: GstWebRTCStatsType = 2;
pub const GST_WEBRTC_STATS_OUTBOUND_RTP: GstWebRTCStatsType = 3;
pub const GST_WEBRTC_STATS_REMOTE_INBOUND_RTP: GstWebRTCStatsType = 4;
pub const GST_WEBRTC_STATS_REMOTE_OUTBOUND_RTP: GstWebRTCStatsType = 5;
pub const GST_WEBRTC_STATS_CSRC: GstWebRTCStatsType = 6;
pub const GST_WEBRTC_STATS_PEER_CONNECTION: GstWebRTCStatsType = 7;
pub const GST_WEBRTC_STATS_DATA_CHANNEL: GstWebRTCStatsType = 8;
pub const GST_WEBRTC_STATS_STREAM: GstWebRTCStatsType = 9;
pub const GST_WEBRTC_STATS_TRANSPORT: GstWebRTCStatsType = 10;
pub const GST_WEBRTC_STATS_CANDIDATE_PAIR: GstWebRTCStatsType = 11;
pub const GST_WEBRTC_STATS_LOCAL_CANDIDATE: GstWebRTCStatsType = 12;
pub const GST_WEBRTC_STATS_REMOTE_CANDIDATE: GstWebRTCStatsType = 13;
pub const GST_WEBRTC_STATS_CERTIFICATE: GstWebRTCStatsType = 14;

// Callbacks
pub type GstWebRTCICEOnCandidateFunc =
    Option<unsafe extern "C" fn(*mut GstWebRTCICE, c_uint, *mut c_char, gpointer)>;

// Records
#[repr(C)]
pub struct _GstWebRTCDTLSTransportClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstWebRTCDTLSTransportClass = *mut _GstWebRTCDTLSTransportClass;

#[repr(C)]
pub struct _GstWebRTCDataChannelClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstWebRTCDataChannelClass = *mut _GstWebRTCDataChannelClass;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCICECandidateStats {
    pub ipaddr: *mut c_char,
    pub port: c_uint,
    pub stream_id: c_uint,
    pub type_: *const c_char,
    pub proto: *const c_char,
    pub relay_proto: *const c_char,
    pub prio: c_uint,
    pub url: *mut c_char,
    pub _gst_reserved: [gpointer; 20],
}

impl ::std::fmt::Debug for GstWebRTCICECandidateStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICECandidateStats @ {:p}", self))
            .field("ipaddr", &self.ipaddr)
            .field("port", &self.port)
            .field("stream_id", &self.stream_id)
            .field("type_", &self.type_)
            .field("proto", &self.proto)
            .field("relay_proto", &self.relay_proto)
            .field("prio", &self.prio)
            .field("url", &self.url)
            .field("_gst_reserved", &self._gst_reserved)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCICEClass {
    pub parent_class: gst::GstObjectClass,
    pub add_stream:
        Option<unsafe extern "C" fn(*mut GstWebRTCICE, c_uint) -> *mut GstWebRTCICEStream>,
    pub find_transport: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICE,
            *mut GstWebRTCICEStream,
            GstWebRTCICEComponent,
        ) -> *mut GstWebRTCICETransport,
    >,
    pub gather_candidates:
        Option<unsafe extern "C" fn(*mut GstWebRTCICE, *mut GstWebRTCICEStream) -> gboolean>,
    pub add_candidate:
        Option<unsafe extern "C" fn(*mut GstWebRTCICE, *mut GstWebRTCICEStream, *const c_char)>,
    pub set_local_credentials: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICE,
            *mut GstWebRTCICEStream,
            *mut c_char,
            *mut c_char,
        ) -> gboolean,
    >,
    pub set_remote_credentials: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICE,
            *mut GstWebRTCICEStream,
            *mut c_char,
            *mut c_char,
        ) -> gboolean,
    >,
    pub add_turn_server: Option<unsafe extern "C" fn(*mut GstWebRTCICE, *const c_char) -> gboolean>,
    pub set_is_controller: Option<unsafe extern "C" fn(*mut GstWebRTCICE, gboolean)>,
    pub get_is_controller: Option<unsafe extern "C" fn(*mut GstWebRTCICE) -> gboolean>,
    pub set_force_relay: Option<unsafe extern "C" fn(*mut GstWebRTCICE, gboolean)>,
    pub set_stun_server: Option<unsafe extern "C" fn(*mut GstWebRTCICE, *const c_char)>,
    pub get_stun_server: Option<unsafe extern "C" fn(*mut GstWebRTCICE) -> *mut c_char>,
    pub set_turn_server: Option<unsafe extern "C" fn(*mut GstWebRTCICE, *const c_char)>,
    pub get_turn_server: Option<unsafe extern "C" fn(*mut GstWebRTCICE) -> *mut c_char>,
    pub set_tos: Option<unsafe extern "C" fn(*mut GstWebRTCICE, *mut GstWebRTCICEStream, c_uint)>,
    pub set_on_ice_candidate: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICE,
            GstWebRTCICEOnCandidateFunc,
            gpointer,
            glib::GDestroyNotify,
        ),
    >,
    pub get_local_candidates: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICE,
            *mut GstWebRTCICEStream,
        ) -> *mut *mut GstWebRTCICECandidateStats,
    >,
    pub get_remote_candidates: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICE,
            *mut GstWebRTCICEStream,
        ) -> *mut *mut GstWebRTCICECandidateStats,
    >,
    pub get_selected_pair: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICE,
            *mut GstWebRTCICEStream,
            *mut *mut GstWebRTCICECandidateStats,
            *mut *mut GstWebRTCICECandidateStats,
        ) -> gboolean,
    >,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCICEClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICEClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("add_stream", &self.add_stream)
            .field("find_transport", &self.find_transport)
            .field("gather_candidates", &self.gather_candidates)
            .field("add_candidate", &self.add_candidate)
            .field("set_local_credentials", &self.set_local_credentials)
            .field("set_remote_credentials", &self.set_remote_credentials)
            .field("add_turn_server", &self.add_turn_server)
            .field("set_is_controller", &self.set_is_controller)
            .field("get_is_controller", &self.get_is_controller)
            .field("set_force_relay", &self.set_force_relay)
            .field("set_stun_server", &self.set_stun_server)
            .field("get_stun_server", &self.get_stun_server)
            .field("set_turn_server", &self.set_turn_server)
            .field("get_turn_server", &self.get_turn_server)
            .field("set_tos", &self.set_tos)
            .field("set_on_ice_candidate", &self.set_on_ice_candidate)
            .field("get_local_candidates", &self.get_local_candidates)
            .field("get_remote_candidates", &self.get_remote_candidates)
            .field("get_selected_pair", &self.get_selected_pair)
            .field("_gst_reserved", &self._gst_reserved)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCICEStreamClass {
    pub parent_class: gst::GstObjectClass,
    pub find_transport: Option<
        unsafe extern "C" fn(
            *mut GstWebRTCICEStream,
            GstWebRTCICEComponent,
        ) -> *mut GstWebRTCICETransport,
    >,
    pub gather_candidates: Option<unsafe extern "C" fn(*mut GstWebRTCICEStream) -> gboolean>,
}

impl ::std::fmt::Debug for GstWebRTCICEStreamClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICEStreamClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("find_transport", &self.find_transport)
            .field("gather_candidates", &self.gather_candidates)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCICETransportClass {
    pub parent_class: gst::GstObjectClass,
    pub gather_candidates: Option<unsafe extern "C" fn(*mut GstWebRTCICETransport) -> gboolean>,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCICETransportClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICETransportClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("gather_candidates", &self.gather_candidates)
            .field("_padding", &self._padding)
            .finish()
    }
}

#[repr(C)]
pub struct _GstWebRTCRTPReceiverClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstWebRTCRTPReceiverClass = *mut _GstWebRTCRTPReceiverClass;

#[repr(C)]
pub struct _GstWebRTCRTPSenderClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstWebRTCRTPSenderClass = *mut _GstWebRTCRTPSenderClass;

#[repr(C)]
pub struct _GstWebRTCRTPTransceiverClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstWebRTCRTPTransceiverClass = *mut _GstWebRTCRTPTransceiverClass;

#[repr(C)]
pub struct _GstWebRTCSCTPTransportClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstWebRTCSCTPTransportClass = *mut _GstWebRTCSCTPTransportClass;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCSessionDescription {
    pub type_: GstWebRTCSDPType,
    pub sdp: *mut gst_sdp::GstSDPMessage,
}

impl ::std::fmt::Debug for GstWebRTCSessionDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCSessionDescription @ {:p}", self))
            .field("type_", &self.type_)
            .field("sdp", &self.sdp)
            .finish()
    }
}

// Classes
#[repr(C)]
pub struct GstWebRTCDTLSTransport {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstWebRTCDTLSTransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCDTLSTransport @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstWebRTCDataChannel {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstWebRTCDataChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCDataChannel @ {:p}", self))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCICE {
    pub parent: gst::GstObject,
    pub ice_gathering_state: GstWebRTCICEGatheringState,
    pub ice_connection_state: GstWebRTCICEConnectionState,
    pub min_rtp_port: c_uint,
    pub max_rtp_port: c_uint,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCICE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICE @ {:p}", self))
            .field("parent", &self.parent)
            .field("ice_gathering_state", &self.ice_gathering_state)
            .field("ice_connection_state", &self.ice_connection_state)
            .field("min_rtp_port", &self.min_rtp_port)
            .field("max_rtp_port", &self.max_rtp_port)
            .field("_gst_reserved", &self._gst_reserved)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCICEStream {
    pub parent: gst::GstObject,
    pub stream_id: c_uint,
}

impl ::std::fmt::Debug for GstWebRTCICEStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICEStream @ {:p}", self))
            .field("parent", &self.parent)
            .field("stream_id", &self.stream_id)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstWebRTCICETransport {
    pub parent: gst::GstObject,
    pub role: GstWebRTCICERole,
    pub component: GstWebRTCICEComponent,
    pub state: GstWebRTCICEConnectionState,
    pub gathering_state: GstWebRTCICEGatheringState,
    pub src: *mut gst::GstElement,
    pub sink: *mut gst::GstElement,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCICETransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICETransport @ {:p}", self))
            .field("parent", &self.parent)
            .field("role", &self.role)
            .field("component", &self.component)
            .field("state", &self.state)
            .field("gathering_state", &self.gathering_state)
            .field("src", &self.src)
            .field("sink", &self.sink)
            .field("_padding", &self._padding)
            .finish()
    }
}

#[repr(C)]
pub struct GstWebRTCRTPReceiver {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstWebRTCRTPReceiver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPReceiver @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstWebRTCRTPSender {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstWebRTCRTPSender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPSender @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstWebRTCRTPTransceiver {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstWebRTCRTPTransceiver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPTransceiver @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstWebRTCSCTPTransport {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstWebRTCSCTPTransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCSCTPTransport @ {:p}", self))
            .finish()
    }
}

#[link(name = "gstwebrtc-1.0")]
extern "C" {

    //=========================================================================
    // GstWebRTCBundlePolicy
    //=========================================================================
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_webrtc_bundle_policy_get_type() -> GType;

    //=========================================================================
    // GstWebRTCDTLSSetup
    //=========================================================================
    pub fn gst_webrtc_dtls_setup_get_type() -> GType;

    //=========================================================================
    // GstWebRTCDTLSTransportState
    //=========================================================================
    pub fn gst_webrtc_dtls_transport_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCDataChannelState
    //=========================================================================
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_webrtc_data_channel_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCError
    //=========================================================================
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_webrtc_error_get_type() -> GType;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_webrtc_error_quark() -> glib::GQuark;

    //=========================================================================
    // GstWebRTCFECType
    //=========================================================================
    #[cfg(any(feature = "v1_14_1", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14_1")))]
    pub fn gst_webrtc_fec_type_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICEComponent
    //=========================================================================
    pub fn gst_webrtc_ice_component_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICEConnectionState
    //=========================================================================
    pub fn gst_webrtc_ice_connection_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICEGatheringState
    //=========================================================================
    pub fn gst_webrtc_ice_gathering_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICERole
    //=========================================================================
    pub fn gst_webrtc_ice_role_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICETransportPolicy
    //=========================================================================
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_webrtc_ice_transport_policy_get_type() -> GType;

    //=========================================================================
    // GstWebRTCKind
    //=========================================================================
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_webrtc_kind_get_type() -> GType;

    //=========================================================================
    // GstWebRTCPeerConnectionState
    //=========================================================================
    pub fn gst_webrtc_peer_connection_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCPriorityType
    //=========================================================================
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_webrtc_priority_type_get_type() -> GType;

    //=========================================================================
    // GstWebRTCRTPTransceiverDirection
    //=========================================================================
    pub fn gst_webrtc_rtp_transceiver_direction_get_type() -> GType;

    //=========================================================================
    // GstWebRTCSCTPTransportState
    //=========================================================================
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_webrtc_sctp_transport_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCSDPType
    //=========================================================================
    pub fn gst_webrtc_sdp_type_get_type() -> GType;
    pub fn gst_webrtc_sdp_type_to_string(type_: GstWebRTCSDPType) -> *const c_char;

    //=========================================================================
    // GstWebRTCSignalingState
    //=========================================================================
    pub fn gst_webrtc_signaling_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCStatsType
    //=========================================================================
    pub fn gst_webrtc_stats_type_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICECandidateStats
    //=========================================================================
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_candidate_stats_get_type() -> GType;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_candidate_stats_copy(
        stats: *mut GstWebRTCICECandidateStats,
    ) -> *mut GstWebRTCICECandidateStats;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_candidate_stats_free(stats: *mut GstWebRTCICECandidateStats);

    //=========================================================================
    // GstWebRTCSessionDescription
    //=========================================================================
    pub fn gst_webrtc_session_description_get_type() -> GType;
    pub fn gst_webrtc_session_description_new(
        type_: GstWebRTCSDPType,
        sdp: *mut gst_sdp::GstSDPMessage,
    ) -> *mut GstWebRTCSessionDescription;
    pub fn gst_webrtc_session_description_copy(
        src: *const GstWebRTCSessionDescription,
    ) -> *mut GstWebRTCSessionDescription;
    pub fn gst_webrtc_session_description_free(desc: *mut GstWebRTCSessionDescription);

    //=========================================================================
    // GstWebRTCDTLSTransport
    //=========================================================================
    pub fn gst_webrtc_dtls_transport_get_type() -> GType;

    //=========================================================================
    // GstWebRTCDataChannel
    //=========================================================================
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn gst_webrtc_data_channel_get_type() -> GType;
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn gst_webrtc_data_channel_close(channel: *mut GstWebRTCDataChannel);
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn gst_webrtc_data_channel_send_data(
        channel: *mut GstWebRTCDataChannel,
        data: *mut glib::GBytes,
    );
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn gst_webrtc_data_channel_send_string(
        channel: *mut GstWebRTCDataChannel,
        str: *const c_char,
    );

    //=========================================================================
    // GstWebRTCICE
    //=========================================================================
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_get_type() -> GType;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_add_candidate(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
        candidate: *const c_char,
    );
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_add_stream(
        ice: *mut GstWebRTCICE,
        session_id: c_uint,
    ) -> *mut GstWebRTCICEStream;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_add_turn_server(ice: *mut GstWebRTCICE, uri: *const c_char) -> gboolean;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_find_transport(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
        component: GstWebRTCICEComponent,
    ) -> *mut GstWebRTCICETransport;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_gather_candidates(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
    ) -> gboolean;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_get_is_controller(ice: *mut GstWebRTCICE) -> gboolean;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_get_local_candidates(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
    ) -> *mut *mut GstWebRTCICECandidateStats;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_get_remote_candidates(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
    ) -> *mut *mut GstWebRTCICECandidateStats;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_get_selected_pair(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
        local_stats: *mut *mut GstWebRTCICECandidateStats,
        remote_stats: *mut *mut GstWebRTCICECandidateStats,
    ) -> gboolean;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_get_stun_server(ice: *mut GstWebRTCICE) -> *mut c_char;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_get_turn_server(ice: *mut GstWebRTCICE) -> *mut c_char;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_force_relay(ice: *mut GstWebRTCICE, force_relay: gboolean);
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_is_controller(ice: *mut GstWebRTCICE, controller: gboolean);
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_local_credentials(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
        ufrag: *mut c_char,
        pwd: *mut c_char,
    ) -> gboolean;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_on_ice_candidate(
        ice: *mut GstWebRTCICE,
        func: GstWebRTCICEOnCandidateFunc,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    );
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_remote_credentials(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
        ufrag: *mut c_char,
        pwd: *mut c_char,
    ) -> gboolean;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_stun_server(ice: *mut GstWebRTCICE, uri: *const c_char);
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_tos(
        ice: *mut GstWebRTCICE,
        stream: *mut GstWebRTCICEStream,
        tos: c_uint,
    );
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_set_turn_server(ice: *mut GstWebRTCICE, uri: *const c_char);

    //=========================================================================
    // GstWebRTCICEStream
    //=========================================================================
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_stream_get_type() -> GType;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_stream_find_transport(
        stream: *mut GstWebRTCICEStream,
        component: GstWebRTCICEComponent,
    ) -> *mut GstWebRTCICETransport;
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn gst_webrtc_ice_stream_gather_candidates(ice: *mut GstWebRTCICEStream) -> gboolean;

    //=========================================================================
    // GstWebRTCICETransport
    //=========================================================================
    pub fn gst_webrtc_ice_transport_get_type() -> GType;
    pub fn gst_webrtc_ice_transport_connection_state_change(
        ice: *mut GstWebRTCICETransport,
        new_state: GstWebRTCICEConnectionState,
    );
    pub fn gst_webrtc_ice_transport_gathering_state_change(
        ice: *mut GstWebRTCICETransport,
        new_state: GstWebRTCICEGatheringState,
    );
    pub fn gst_webrtc_ice_transport_new_candidate(
        ice: *mut GstWebRTCICETransport,
        stream_id: c_uint,
        component: GstWebRTCICEComponent,
        attr: *mut c_char,
    );
    pub fn gst_webrtc_ice_transport_selected_pair_change(ice: *mut GstWebRTCICETransport);

    //=========================================================================
    // GstWebRTCRTPReceiver
    //=========================================================================
    pub fn gst_webrtc_rtp_receiver_get_type() -> GType;

    //=========================================================================
    // GstWebRTCRTPSender
    //=========================================================================
    pub fn gst_webrtc_rtp_sender_get_type() -> GType;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_webrtc_rtp_sender_set_priority(
        sender: *mut GstWebRTCRTPSender,
        priority: GstWebRTCPriorityType,
    );

    //=========================================================================
    // GstWebRTCRTPTransceiver
    //=========================================================================
    pub fn gst_webrtc_rtp_transceiver_get_type() -> GType;

    //=========================================================================
    // GstWebRTCSCTPTransport
    //=========================================================================
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn gst_webrtc_sctp_transport_get_type() -> GType;

}
