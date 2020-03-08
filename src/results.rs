
pub enum Success {
    Quit,
    Retire,
    Help(String),
    Victory(String),
    ValidMove(String),
}

pub enum Failure {
    InvalidMove,
    InvalidCommand,
}
