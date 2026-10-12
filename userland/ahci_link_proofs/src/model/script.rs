// NONOS Operating System (AGPL-3.0-or-later)
//! What the drive is given and how it answers: a command as the HBA sent
//! it, and a script of how long each command takes and how it ends.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd {
    pub op: u8,
    pub lba: u64,
    pub count: u32,
}

/// How a command ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Clean,
    /// Status 0x51 (DRDY, DSC, ERR) with this error register.
    Error(u8),
    /// A clean status, having moved this many bytes fewer than asked.
    Short(u32),
}

/// One command's turn: how long the drive takes, and how it ends. Past the
/// end of its script every command takes 1 ms and ends clean.
#[derive(Clone, Copy)]
pub struct Step {
    pub ms: u64,
    pub outcome: Outcome,
}

pub struct Done {
    pub cmd: Cmd,
    pub outcome: Outcome,
}
