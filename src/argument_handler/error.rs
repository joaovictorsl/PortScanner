// Different kinds of ArgumentError
pub enum ArgumentError {
    InvalidNumberOfArguments(&'static str),
    InvalidFlag(String),
    InvalidNumberOfThreads(&'static str),
    InvalidIpAddr(&'static str),
}
// Implementing Display trait for ArgumentError
impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgumentError::InvalidNumberOfArguments(msg) => {
                write!(f, "{}", msg)
            }
            ArgumentError::InvalidFlag(msg) => write!(f, "{}", msg),
            ArgumentError::InvalidNumberOfThreads(msg) => {
                write!(f, "{}", msg)
            }
            ArgumentError::InvalidIpAddr(msg) => write!(f, "{}", msg),
        }
    }
}
