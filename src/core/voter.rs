use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voter {
    pub id: Uuid,
}

impl Voter {
    pub fn new() -> Self {
        Voter { id: Uuid::new_v4() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRegistryEntry {
    pub voter: Voter,
    pub has_voted: bool,
}
