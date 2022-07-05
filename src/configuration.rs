use std::collections::HashMap;
use crate::errors::configuration_error::ConfigurationError;

// ServerID is a unique string identifying a server for all time.
pub type ServerID = String;

// ServerAddress is a network address for a server that a transport can contact.
pub type ServerAddress = String;

const VOTER: ServerSuffrage = 0;
const NONVOTER: ServerSuffrage = 1;
const STAGING: ServerSuffrage = 2;

#[derive(Debug, PartialEq, Clone)]
pub enum ServerSuffrage {
    Voter,
    Nonvoter,
    Staging
}

#[derive(Debug, PartialEq, Clone)]
pub struct Server {
    pub suffrage: ServerSuffrage,
    pub id: ServerID,
    pub addr: ServerAddress
}

#[derive(Debug, Clone, PartialEq)]
pub struct Configuration {
    pub servers: Vec<Server>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Configurations {
    pub committed: Configuration,
    pub committed_index: u32,
    pub latest: Configuration,
    pub latest_index: u32
}

// ConfigurationChangeCommand is the different ways to change the cluster configuration
#[derive(Clone)]
pub enum ConfigurationChangeCommand {
    AddStaging,
    AddNonvoter,
    DemoteVoter,
    RemoveServer,
    Promote
}

impl ConfigurationChangeCommand {
    pub fn configuration_change_command(self) -> String {
        let comm = match self {
            ConfigurationChangeCommand::AddStaging => "AddStaging".to_string(),
            ConfigurationChangeCommand::AddNonvoter => "AddNonvoter".to_string(),
            ConfigurationChangeCommand::DemoteVoter => "DemoteVoter".to_string(),
            ConfigurationChangeCommand::RemoveServer => "RemoveServer".to_string(),
            ConfigurationChangeCommand::Promote => "Promote".to_string(),
        };
        comm
    }
}

#[derive(Clone)]
pub struct ConfigurationChangeRequest {
    pub command: ConfigurationChangeCommand,
    pub server_id: ServerID,
    pub server_addr: ServerAddress,
    pub prev_index: Option<u64>,
}

impl Configuration {

    pub fn new() -> Configuration {
        Configuration {
            servers: Vec::new()
        }
    }

    pub fn has_vote(&self, id: ServerID) -> bool {
        for server in &self.servers {
            if server.id == id {
                return true;
            }
        }

        false
    }

    pub fn check_configuration(&self) -> Result<(), ConfigurationError> {
        let mut id_set: HashMap<ServerID, bool> = HashMap::new();
        let mut addr_set: HashMap<ServerAddress, bool> = HashMap::new();
        let mut voters:u32 = 0;

        let mut err = if self.servers.len() == 0 {
            return Err(ConfigurationError::EmptyConfiguration);
        };

        err = for server in &self.servers {
            id_set.insert(server.id.clone(), true);
            addr_set.insert(server.addr.clone(), true);
            
            if server.id == "" {
                return Err(ConfigurationError::EmptyIdInConfiguration);
            }
            if server.addr == "" {
                return Err(ConfigurationError::EmptyAddressInConfiguration);
            }
            if *id_set.get(&server.id).unwrap() {
                return Err(ConfigurationError::FoundDuplicateIdInConfiguration);
            }
            if let Some(x) = id_set.get_mut(&server.id) {
                *x = true
            }
            if *addr_set.get(&server.addr).unwrap() {
                return Err(ConfigurationError::FoundDuplicateAddressInConfiguration);
            }
            if let Some(x) = addr_set.get_mut(&server.addr) {
                *x = true
            }
            if server.suffrage == ServerSuffrage::Voter {
                voters += 1;
            }
        };

        err = if voters == 0 {
            return Err(ConfigurationError::NeedAtLeastOneVoterInConfiguration);
        };

        Ok(err)
    }

    pub fn next_configuration(
        &self,
        current_index: u64,
        change: ConfigurationChangeRequest
    ) -> Result<Self, ConfigurationError> {
        if change.prev_index > Some(0) && change.prev_index != Some(current_index) {
            return Ok(Configuration {
                servers: Vec::new()
            });
        }

        let mut configuration = self.clone();

        match &change.command {
            ConfigurationChangeCommand::AddStaging => {
                let new_server = Server {
                    suffrage: ServerSuffrage::Voter,
                    id: change.server_id.clone(),
                    addr: change.server_addr.clone()
                };

                let mut found = false;
                for i in 0..configuration.servers.len() {
                    for server in &configuration.servers {
                        if server.id == change.server_id {
                            if server.suffrage != ServerSuffrage::Voter {
                                configuration.servers[i].addr = change.server_addr.clone()
                            } else {
                                configuration.servers[i] = new_server.clone()
                            }
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    configuration.servers.push(new_server)
                }
            },
            ConfigurationChangeCommand::AddNonvoter => {
                let new_server = Server {
                    suffrage: ServerSuffrage::Nonvoter,
                    id: change.server_id.clone(),
                    addr: change.server_addr.clone()
                };

                let mut found = false;
                for i in 0..configuration.servers.len() {
                    for server in &configuration.servers {
                        if server.id == change.server_id {
                            if server.suffrage != ServerSuffrage::Nonvoter {
                                configuration.servers[i].addr = change.clone().server_addr
                            } else {
                                configuration.servers[i] = new_server.clone()
                            }
                            found = true;
                            break;
                        }
                    }
                }
                
                if !found {
                    configuration.servers.push(new_server)
                }
            },
            ConfigurationChangeCommand::DemoteVoter => {
                for i in 0..configuration.servers.len() {
                    for server in &configuration.servers {
                        if server.id == change.server_id {
                            configuration.servers[i].suffrage = ServerSuffrage::Nonvoter;
                            break;
                        }
                    }
                }
            },
            ConfigurationChangeCommand::RemoveServer => {
                for i in 0..configuration.servers.len() {
                    for server in &configuration.servers {
                        if server.id == change.server_id && configuration.servers.len() > i {
                            configuration.servers.remove(i);
                            break;
                        }
                    }
                }
            },
            ConfigurationChangeCommand::Promote => {
                for i in 0..configuration.servers.len() {
                    for server in &configuration.servers {
                        if server.id == change.server_id && server.suffrage == ServerSuffrage::Staging {
                            configuration.servers[i].suffrage = ServerSuffrage::Voter;
                            break;
                        }
                    }
                }
            },
        }
        
        Ok(configuration)
    }

    pub fn encode_configuration(self) -> &'static [u8] {
        todo!()
    }

    pub fn decode_configuration(bytes: &[u8]) -> Self {
        todo!()
    }
}