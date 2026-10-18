// NONOS Operating System (AGPL-3.0-or-later)
//! The drive on the link: it takes one command at a time, runs it for as
//! long as its script says, and ends it as the script says. A command FIS
//! that arrives while it is busy is ignored. Only a COMRESET makes it drop
//! the command it holds; the driver giving up on it does not.

use std::time::{Duration, Instant};

use super::fetch::Slot;
use super::medium::Medium;
use super::script::{Cmd, Done, Outcome, Step};
use crate::constants::ata::{ATA_IDENTIFY, ATA_READ_DMA_EXT, ATA_WRITE_DMA_EXT, SECTOR_SIZE};

pub struct Drive {
    script: Vec<Step>,
    busy: Option<(Done, Instant)>,
    /// Every command the drive took, and how many it ignored.
    pub taken: Vec<Cmd>,
    pub ignored: u32,
    pub medium: Medium,
}

impl Drive {
    pub fn new(script: Vec<Step>) -> Self {
        Self { script, busy: None, taken: Vec::new(), ignored: 0, medium: Medium::default() }
    }

    pub fn take(&mut self, cmd: Cmd, now: Instant) {
        if self.busy.is_some() {
            self.ignored += 1;
            return;
        }
        let step = self.script.get(self.taken.len()).copied();
        let step = step.unwrap_or(Step { ms: 1, outcome: Outcome::Clean });
        self.taken.push(cmd);
        let done = Done { cmd, outcome: step.outcome };
        self.busy = Some((done, now + Duration::from_millis(step.ms)));
    }

    pub fn finished(&mut self, now: Instant) -> Option<Done> {
        match self.busy.take() {
            Some((done, at)) if now >= at => Some(done),
            held => {
                self.busy = held;
                None
            }
        }
    }

    pub fn reset(&mut self) {
        self.busy = None;
    }

    /// Holds no command.
    pub fn idle(&self) -> bool {
        self.busy.is_none()
    }

    /// The data phase of `done` through `slot`, as far as the slot's PRD
    /// reaches. Gives the bytes moved.
    pub fn move_data(&mut self, done: &Done, slot: &Slot) -> u32 {
        let asked = u64::from(done.cmd.count) * SECTOR_SIZE as u64;
        let less = if let Outcome::Short(n) = done.outcome { u64::from(n) } else { 0 };
        let bytes = asked.saturating_sub(less).min(slot.bytes);
        match done.cmd.op {
            ATA_READ_DMA_EXT => self.medium.read_into(done.cmd.lba, slot.data, bytes),
            ATA_WRITE_DMA_EXT => self.medium.write_from(done.cmd.lba, slot.data, bytes),
            ATA_IDENTIFY => super::identify::identify_into(slot.data, bytes),
            _ => return 0,
        }
        bytes as u32
    }
}
