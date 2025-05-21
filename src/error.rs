use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VotingError {
    VoterNotFound,
    InvalidChoice,
    InvalidBallot,
    InvalidElection,
    InvalidVote,
    InvalidReceipt,
}

impl fmt::Display for VotingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VotingError::VoterNotFound => write!(f, "Voter not found"),
            VotingError::InvalidChoice => write!(f, "Invalid choice"),
            VotingError::InvalidBallot => write!(f, "Invalid ballot"),
            VotingError::InvalidElection => write!(f, "Invalid election"),
            VotingError::InvalidVote => write!(f, "Invalid vote"),
            VotingError::InvalidReceipt => write!(f, "Invalid receipt"),
        }
    }
}

impl Error for VotingError {}

pub type Result<T> = std::result::Result<T, VotingError>;
