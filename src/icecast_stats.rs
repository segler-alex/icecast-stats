use crate::IcecastStatsSource;
#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Root element of the icecast information
#[derive(Serialize, Deserialize, Debug)]
pub struct IcecastStatsRoot {
    pub icestats: IcecastStats,
}

/// Basic server information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IcecastStats {
    admin: String,
    client_connections: Option<u32>,
    clients: Option<u32>,
    connections: Option<u32>,
    file_connections: Option<u32>,
    host: String,
    listener_connections: Option<u32>,
    listeners: Option<u32>,
    location: String,
    server_id: String,
    source_client_connections: Option<u32>,
    source_relay_connections: Option<u32>,
    source_total_connections: Option<u32>,
    sources: Option<u32>,
    stats: Option<u32>,
    stats_connections: Option<u32>,
    #[cfg(feature = "chrono")]
    server_start_iso8601: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    server_start_iso8601: String,

    /// List of active streaming sources
    pub source: Option<IcecastStatsSourceEnum>,
}

impl IcecastStats {
    /// As set in the server config, this should contain contact details for getting in touch
    /// with the server administrator. Usually this will be an email address, but as
    /// this can be an arbitrary string it could also be a phone number.
    ///
    /// Example: icemaster@localhost
    pub fn admin(&self) -> &String {
        &self.admin
    }

    /// Client connections are basically anything that is not a source connection. These include listeners (not concurrent, but cumulative), any admin function accesses, and any static content (file serving) accesses.
    /// This is an accumulating counter.
    pub fn client_connections(&self) -> &Option<u32> {
        &self.client_connections
    }

    /// Number of currently active client connections.
    pub fn clients(&self) -> &Option<u32> {
        &self.clients
    }

    /// The total of all inbound TCP connections since start-up.
    /// This is an accumulating counter.
    pub fn connections(&self) -> &Option<u32> {
        &self.connections
    }

    /// This is an accumulating counter.
    pub fn file_connections(&self) -> &Option<u32> {
        &self.file_connections
    }

    /// As set in the server config, this should be the full DNS resolveable name or FQDN for the host on which this Icecast instance is running.
    ///
    /// Example: localhost
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Number of listener connections to mount points.
    /// This is an accumulating counter.
    pub fn listener_connections(&self) -> &Option<u32> {
        &self.listener_connections
    }

    /// Number of currently active listener connections.
    pub fn listeners(&self) -> &Option<u32> {
        &self.listeners
    }

    /// As set in the server config, this is a free form field that should describe e.g. the physical location of this server.
    ///
    /// Example: Earth
    pub fn location(&self) -> &String {
        &self.location
    }

    /// Defaults to the version string of the currently running Icecast server. While not recommended it can be overriden in the server config.
    ///
    /// Example: Icecast 2.4.4
    pub fn server_id(&self) -> &String {
        &self.server_id
    }

    /// Source client connections are the number of times (cumulative since start-up, not just currently connected) a source client has connected to Icecast.
    /// This is an accumulating counter.
    pub fn source_client_connections(&self) -> &Option<u32> {
        &self.source_client_connections
    }

    /// Number of outbound relay connections to (master) icecast servers.
    /// This is an accumulating counter.
    pub fn source_relay_connections(&self) -> &Option<u32> {
        &self.source_relay_connections
    }

    /// Both clients and relays.
    /// This is an accumulating counter.
    pub fn source_total_connections(&self) -> &Option<u32> {
        &self.source_total_connections
    }

    /// The total of currently connected sources.
    pub fn sources(&self) -> &Option<u32> {
        &self.sources
    }

    /// The total of currently connected STATS clients.
    pub fn stats(&self) -> &Option<u32> {
        &self.stats
    }

    /// Number of times a stats client has connected to Icecast.
    /// This is an accumulating counter.
    pub fn stats_connections(&self) -> &Option<u32> {
        &self.stats_connections
    }

    /// Timestamp of server startup in ISO 8601 date format.
    ///
    /// Example: 2021-04-09T21:49:50+0200
    #[cfg(feature = "chrono")]
    pub fn server_start_iso8601(&self) -> &DateTime<Utc> {
        &self.server_start_iso8601
    }

    /// Timestamp of server startup in ISO 8601 date format.
    ///
    /// Example: 2021-04-09T21:49:50+0200
    #[cfg(not(feature = "chrono"))]
    pub fn server_start_iso8601(&self) -> &String {
        &self.server_start_iso8601
    }
}

/// Multiple variants of sources
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum IcecastStatsSourceEnum {
    Single(IcecastStatsSource),
    Multiple(Vec<IcecastStatsSource>),
}
