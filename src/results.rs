
pub enum Success {
    Victory(String),
    ValidMove(String),
}

pub enum Failure {
    InvalidMove,
    InvalidCommand,
}
