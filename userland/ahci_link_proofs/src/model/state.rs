// NONOS Operating System (AGPL-3.0-or-later)
//! What the model port holds: its registers, what the driver did to it,
//! and the times its pending events fall due.

use std::time::{Duration, Instant};

use crate::constants::regs::{CMD_CR, CMD_FR, CMD_FRE, CMD_ST};

/// PxSSTS with the link up: DET 3, Gen 2, interface active.
pub const LINK_UP: u32 = 0x123;
/// PxSERR.DIAG.N (PhyRdy changed) and DIAG.X (COMINIT came in).
pub const DIAG_N: u32 = 1 << 16;
pub const DIAG_X: u32 = 1 << 26;

pub struct PortRegs {
    pub cmd: u32,
    pub tfd: u32,
    pub is: u32,
    pub ci: u32,
    pub sact: u32,
    pub sctl: u32,
    pub ssts: u32,
    pub serr: u32,
}

impl PortRegs {
    /// Link up, engine and FIS receive on, the drive idle.
    pub fn idle() -> Self {
        let cmd = CMD_ST | CMD_CR | CMD_FRE | CMD_FR;
        Self { cmd, tfd: 0x50, is: 0, ci: 0, sact: 0, sctl: 0x300, ssts: LINK_UP, serr: 0 }
    }

    /// PxIS as read: PRCS (22) and PCS (6) mirror DIAG.N and DIAG.X, and
    /// clear only with them (AHCI 1.3.1, 3.3.5).
    pub fn is_read(&self) -> u32 {
        let mirror = |diag, bit| if self.serr & diag != 0 { bit } else { 0 };
        self.is | mirror(DIAG_N, 1 << 22) | mirror(DIAG_X, 1 << 6)
    }
}

/// What the driver did to the port, counted.
#[derive(Default)]
pub struct Count {
    /// PxCI writes, and those made with the engine off.
    pub issued: u32,
    pub issued_stopped: u32,
    /// COMRESETs the drive saw, and DET holds too short to be one.
    pub comresets: u32,
    pub short_holds: u32,
    pub clo: u32,
    /// PxSERR and PxIS as read each time ST was set.
    pub started_with: Vec<(u32, u32)>,
    /// Command FISes the HBA sent to a drive still busy or without a link,
    /// which it never took, and slots its ready status ended with nothing
    /// run.
    pub lost: u32,
    pub unrun: u32,
    /// For every command dropped by stopping the engine, how long it had
    /// been given since PxCI was set.
    pub given: Vec<Duration>,
}

/// When PxCI was last set, DET last went to COMRESET, the link comes back,
/// and the drive, busy or resetting, posts its ready status.
#[derive(Default)]
pub struct Times {
    pub ci: Option<Instant>,
    pub reset_from: Option<Instant>,
    pub link: Option<Instant>,
    pub ready: Option<Instant>,
    /// When the drive last took a command.
    pub taken: Option<Instant>,
}
