pub enum Phase {
    NewRound,
    PrePrepared,
    Prepared,
    Committed,
    FinalCommitted,
    RoundChange,
}

pub struct State {
    view: u128,
    phase: Phase,
    is_primary: bool
}