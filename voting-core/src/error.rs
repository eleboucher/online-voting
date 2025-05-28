use thiserror::Error;

#[derive(Debug, Error)]
pub enum VotingError {
    #[error("Voter not found")]
    VoterNotFound,

    #[error("Invalid choice")]
    InvalidChoice,

    #[error("Invalid ballot")]
    InvalidBallot,

    #[error("Ballot already cast")]
    AlreadyVoted,

    #[error("Ballot not found")]
    BallotNotFound,

    #[error("Invalid election")]
    InvalidElection,

    #[error("Election not found")]
    ElectionNotFound,

    // Add new crypto-specific errors
    #[error("Invalid cryptographic parameters")]
    InvalidParameters,

    #[error("Encryption failed: {0}")]
    EncryptionError(String),

    #[error("Decryption failed: {0}")]
    DecryptionError(String),

    #[error("Invalid zero-knowledge proof")]
    InvalidProof,

    #[error("Insufficient trustees: need {needed}, have {available}")]
    InsufficientTrustees { needed: usize, available: usize },

    #[error("Tally not allowed")]
    TallyNotAllowed,
}

pub type Result<T> = std::result::Result<T, VotingError>;
