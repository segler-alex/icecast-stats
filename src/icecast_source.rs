#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// A streaming source
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IcecastStatsSource {
    artist: Option<String>,
    title: Option<String>,
    audio_bitrate: Option<u32>,
    audio_channels: Option<u8>,
    audio_samplerate: Option<u32>,
    frame_rate: Option<f32>,
    frame_size: Option<String>,
    bitrate: Option<u32>,
    genre: Option<String>,
    #[serde(rename = "ice-bitrate")]
    ice_bitrate: Option<u32>,
    listener_peak: Option<u32>,
    listeners: Option<u32>,
    listenurl: Option<String>,
    server_description: Option<String>,
    server_name: Option<String>,
    server_type: Option<String>,
    subtype: Option<String>,
    #[cfg(feature = "chrono")]
    stream_start_iso8601: Option<DateTime<Utc>>,
    #[cfg(not(feature = "chrono"))]
    stream_start_iso8601: Option<String>,
    total_bytes_read: Option<u32>,
    total_bytes_sent: Option<u32>,
    user_agent: Option<String>,
    video_bitrate: Option<u32>,
    video_quality: Option<u32>,
}

impl IcecastStatsSource {
    /// Artist of the current song
    /// Metadata set by source client
    pub fn artist(&self) -> &Option<String> {
        &self.artist
    }

    /// Title of the current song
    /// Metadata set by source client
    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    /// Audio bitrate in bits/s
    /// Can be set by source client
    /// Example: 128000
    pub fn audio_bitrate(&self) -> &Option<u32> {
        &self.audio_bitrate
    }

    /// Audio bitrate in bits/s
    /// Can be set by source client
    /// Example: 1
    pub fn audio_channels(&self) -> &Option<u8> {
        &self.audio_channels
    }

    /// Information about the samplerate of the stream.
    /// Can be set by source client
    /// Example: 48000
    pub fn audio_samplerate(&self) -> &Option<u32> {
        &self.audio_samplerate
    }

    /// Example: 25.00
    pub fn frame_rate(&self) -> &Option<f32> {
        &self.frame_rate
    }

    /// Example: 720 x 576
    pub fn frame_size(&self) -> &Option<String> {
        &self.frame_size
    }

    /// Example: 128
    pub fn bitrate(&self) -> &Option<u32> {
        &self.bitrate
    }

    /// Example: jazz, classical
    pub fn genre(&self) -> &Option<String> {
        &self.genre
    }

    /// Example: 128
    pub fn ice_bitrate(&self) -> &Option<u32> {
        &self.ice_bitrate
    }

    /// Example: 234
    pub fn listener_peak(&self) -> &Option<u32> {
        &self.listener_peak
    }

    /// Currently Active Listeners.
    ///
    /// Example: 23
    pub fn listeners(&self) -> &Option<u32> {
        &self.listeners
    }

    /// Example: http://localhost:8000/test2
    pub fn listenurl(&self) -> &Option<String> {
        &self.listenurl
    }

    /// Example: Unspecified description
    pub fn server_description(&self) -> &Option<String> {
        &self.server_description
    }

    /// Example: Unspecified name
    pub fn server_name(&self) -> &Option<String> {
        &self.server_name
    }

    /// MIME-type for the stream currently active on this mountpoint.
    /// 
    /// Example: application/ogg
    pub fn server_type(&self) -> &Option<String> {
        &self.server_type
    }

    /// MIME-subtype, can be e.g. codecs like Opus, Vorbis, Theora.
    /// Separated with /.
    ///
    /// Example: application/ogg
    pub fn subtype(&self) -> &Option<String> {
        &self.subtype
    }

    /// Example: 2021-04-09T21:49:52+0200
    #[cfg(feature = "chrono")]
    pub fn stream_start_iso8601(&self) -> &Option<DateTime<Utc>> {
        &self.stream_start_iso8601
    }

    /// Example: 2021-04-09T21:49:52+0200
    #[cfg(not(feature = "chrono"))]
    pub fn stream_start_iso8601(&self) -> &Option<String> {
        &self.stream_start_iso8601
    }

    /// HTTP user agent string as sent by the source client.
    pub fn user_agent(&self) -> &Option<String> {
        &self.user_agent
    }
    /// Example: 200000
    pub fn video_bitrate(&self) -> &Option<u32> {
        &self.video_bitrate
    }

    /// Example: 0
    pub fn video_quality(&self) -> &Option<u32> {
        &self.video_quality
    }
}
