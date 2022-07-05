pub type ProtocolVersion = u32;

const PROTOCOLVERSIONMIN: ProtocolVersion = 0;
const PROTOCOLVERSIONMAX: ProtocolVersion = 3;

pub type SnapshotVersion = u32;

const SNAPSHOTVERSIONMIN: SnapshotVersion = 0;
const SNAPSHOTVERSIONMAX: SnapshotVersion = 1;

pub struct Config {
    pub protocol_version: ProtocolVersion,
    pub heartbeat_timeout: std::time::Duration,
    pub election_timeout: std::time::Duration,
    pub commit_timeout: std::time::Duration,
    pub max_append_entries: u32,
    pub batch_apply_ch: bool,
    pub shutdown_on_remove: bool,
    pub trailing_logs: u64,
    pub snapshot_interval: std::time::Duration,
    pub snapshot_threshold: u64,
    pub leader_lease_timeout: std::time::Duration,
    // pub local_id: server_id,
}