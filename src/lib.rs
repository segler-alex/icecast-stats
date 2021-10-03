//! ## Icecast Server Status
//! Decode server status from an [icecast server](https://icecast.org/docs/icecast-trunk/server_stats/).
//!
//! ### Usage example
//! ```no_run
//! use icecast_stats::fetch;
//! 
//! let url_str = "https://stream.example.com:8000/somestream";
//! let stats = fetch(url_str).unwrap();
//! println!("{:#?}", stats);
//! ```
//! 
//! ### Example for a full icecast stats json file
//! ```json
//! {
//!     "icestats": {
//!         "admin": "icemaster@localhost",
//!         "host": "localhost",
//!         "location": "Earth",
//!         "server_id": "Icecast 2.4.4",
//!         "server_start": "Fri, 09 Apr 2021 21:49:50 +0200",
//!         "server_start_iso8601": "2021-04-09T21:49:50+0200",
//!         "source": {
//!             "audio_bitrate": 128000,
//!             "audio_channels": 1,
//!             "audio_samplerate": 48000,
//!             "bitrate": 128,
//!             "genre": "various",
//!             "ice-bitrate": 128,
//!             "listener_peak": 1,
//!             "listeners": 0,
//!             "listenurl": "http://localhost:8000/test2",
//!             "server_description": "Unspecified description",
//!             "server_name": "Unspecified name",
//!             "server_type": "application/ogg",
//!             "stream_start": "Fri, 09 Apr 2021 21:49:52 +0200",
//!             "stream_start_iso8601": "2021-04-09T21:49:52+0200",
//!             "subtype": "Vorbis",
//!             "dummy": null
//!         }
//!     }
//! }
//! ```

mod icecast_stats;
mod icecast_source;

use reqwest::blocking::get;
use std::error::Error;
pub use url::Url;

pub use crate::icecast_stats::IcecastStats;
pub use crate::icecast_stats::IcecastStatsRoot;
pub use crate::icecast_source::IcecastStatsSource;

/// Try to generate an icecast status url from any url to the same server
/// 
/// Example:
/// ```rust
/// use icecast_stats::Url;
/// use icecast_stats::generate_icecast_stats_url;
/// 
/// let url_str = "https://stream.example.com:8000/somestream";
/// let url = Url::parse(url_str).unwrap();
/// let stats_url = generate_icecast_stats_url(url);
/// assert!(stats_url.to_string().eq("https://stream.example.com:8000/status-json.xsl"));
/// ```
pub fn generate_icecast_stats_url(mut base: Url) -> Url {
    base.set_path("/status-json.xsl");
    base
}

/// Fetch icecast status information from server
/// 
/// This is a shorthand for
/// * generate_icecast_stats_url()
/// * downloading json
/// * parsing json
/// 
/// Example:
/// ```no_run
/// use icecast_stats::fetch;
/// 
/// let url_str = "https://stream.example.com:8000/somestream";
/// let stats = fetch(url_str).unwrap();
/// ```
pub fn fetch(url: &str) -> Result<IcecastStats, Box<dyn Error>> {
    let base_url = Url::parse(url)?;
    let url = generate_icecast_stats_url(base_url);
    let resp = get(url.to_string())?;
    let j: icecast_stats::IcecastStatsRoot = resp.json()?;
    Ok(j.icestats)
}
