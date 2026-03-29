use std::{path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TorrentId(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KBps(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MB(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScheduleDay(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Encryption {
    Required,
    Preferred,
    Tolerated,
}

// #[derive(Debug, Clone)]
// pub struct TrackerTierList(Vec<Vec<url::Url>>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Session {
    alt_speed_down: KBps,
    alt_speed_up: KBps,
    alt_speed_enabled: bool,

    #[serde(with = "minutes")]
    alt_speed_time_begin: Duration,

    #[serde(with = "minutes")]
    alt_speed_time_end: Duration,
    alt_speed_time_day: ScheduleDay,
    alt_speed_time_enabled: bool,
    anti_brute_force_enabled: bool,
    anti_brute_force_threshold: i32,
    blocklist_enabled: bool,
    blocklist_size: i32,
    #[serde(with = "opt_string")]
    blocklist_url: Option<String>,
    cache_size_mb: MB,
    config_dir: PathBuf,
    // default_trackers: Vec<String>,
    dht_enabled: bool,
    download_dir: PathBuf,
    download_queue_enabled: bool,
    download_queue_size: u32,
    encryption: Encryption,
    #[serde(with = "minutes")]
    idle_seeding_limit: Duration,
    idle_seeding_limit_enabled: bool,
    incomplete_dir: Option<PathBuf>,
    incomplete_dir_enabled: bool,
    lpd_enabled: bool,
    peer_limit_global: i32,
    peer_limit_per_torrent: i32,
    peer_port: u16,
    peer_port_random_on_start: bool,
    pex_enabled: bool,
    port_forwarding_enabled: bool,
    queue_stalled_enabled: bool,

    #[serde(with = "minutes")]
    queue_stalled_minutes: Duration,

    rename_partial_files: bool,

    rpc_version: i32,
    rpc_version_minimum: i32,
    rpc_version_semver: String,
    script_torrent_added_enabled: bool,
    script_torrent_added_filename: Option<PathBuf>,
    script_torrent_done_enabled: bool,
    script_torrent_done_filename: Option<PathBuf>,
    script_torrent_done_seeding_enabled: bool,
    script_torrent_done_seeding_filename: Option<PathBuf>,
    seed_queue_enabled: bool,
    seed_queue_size: i32,
    #[serde(rename = "seedRatioLimit")]
    seed_ratio_limit: i32,
    #[serde(rename = "seedRatioLimited")]
    seed_ratio_limited: bool,
    session_id: String,
    speed_limit_down: i32,
    speed_limit_down_enabled: bool,
    speed_limit_up: i32,
    speed_limit_up_enabled: bool,
    start_added_torrents: bool,
    tcp_enabled: bool,
    trash_original_torrent_files: bool,
    utp_enabled: bool,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    arguments: T,
}

mod minutes {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S: Serializer>(v: &Duration, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_u64(v.as_secs() / 60)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
        let minutes: u16 = Deserialize::deserialize(deserializer)?;
        Ok(Duration::from_secs((minutes as u64) * 60))
    }
}

mod opt_string {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<String>, ser: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(s) => ser.serialize_str(&s),
            None => ser.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<String>, D::Error> {
        let s: String = Deserialize::deserialize(deserializer)?;

        if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
    }
}
