pub struct RPCHeader {
    pub protocol_version: ProtocolVersion,
    pub id: Box<[u8]>,
    pub addr: Box<[u8]>
}

pub trait WithRPCHeader {
    fn getRPCHeader(self) -> RPCHeader;
}

pub struct AppendEntriesRequest {
    pub rpc_header: RPCHeader,
    pub term: u64,
    pub leader: Box<[u8]>,
    pub prev_log_entry: u64,
    pub prev_log_term: u64,
    pub entries: Vec<Log>,
    pub leader_commit_index: u64
}

impl WithRPCHeader for AppendEntriesRequest {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}

pub struct AppendEntriesResponse {
    pub rpc_header: RPCHeader,
    pub term: u64,
    pub last_log: u64,
    pub success: bool,
    pub no_retry_backoff: bool
}

impl WithRPCHeader for AppendEntriesResponse {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}

pub struct RequestVoteRequest {
    pub rpc_header: RPCHeader,
    pub term: u64,
    pub candidate: Box<[u8]>,
    pub last_log_index: u64,
    pub last_log_term: u64,
    pub leadership_transfer: bool
}

impl WithRPCHeader for RequestVoteRequest {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}

pub struct RequestVoteResponse {
    pub rpc_header: RPCHeader,
    pub term: u64,
    pub peers: Box<[u8]>,
    pub granted: bool
}

impl WithRPCHeader for RequestVoteResponse {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}

pub struct InstallSnapshotRequest {
    pub rpc_header: RPCHeader,
    pub snapshot_version: SnapshotVersion,
    pub term: u64,
    pub leader: Box<[u8]>,
    pub last_log_index: u64,
    pub last_log_term: u64,
    pub peers: Box<[u8]>,
    pub configuration: Box<[u8]>,
    pub configuration_index: u64,
    pub size: i64
}

impl WithRPCHeader for InstallSnapshotRequest {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}

pub struct InstallSnapshotResponse {
    pub rpc_header: RPCHeader,
    pub term: u64,
    pub success: bool
}

impl WithRPCHeader for InstallSnapshotResponse {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}

pub struct TimeoutNowRequest {
    pub rpc_header: RPCHeader
}

impl WithRPCHeader for TimeoutNowRequest {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}

pub struct TimeoutNowResponse {
    pub rpc_header: RPCHeader
}

impl WithRPCHeader for TimeoutNowResponse {
    fn getRPCHeader(self) -> RPCHeader {
        self.rpc_header
    }
}