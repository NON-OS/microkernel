// NONOS Operating System (AGPL-3.0-or-later)
//! A model ITS for the command-queue proofs (IHI 0069, 5.2): it reads
//! 32-byte slots from its read offset up to `GITS_CWRITER`, wrapping at the
//! end of the queue, and on an error command stops and sets
//! `CREADR.Stalled`. Each slot remembers whether it holds a command not yet
//! read, so a write over one is caught as it happens.

use crate::gic::its::ring_types::{BYTES, SLOT};

const SLOTS: usize = (BYTES / SLOT) as usize;

pub struct ModelIts {
    unread: [Option<u64>; SLOTS],
    read: u64,
    cwriter: u64,
    stalled: bool,
    /// The command that stalls the ITS when read.
    pub poison: Option<u64>,
    pub consumed: Vec<u64>,
    pub overwrites: usize,
}

impl ModelIts {
    pub fn new() -> Self {
        Self {
            unread: [None; SLOTS],
            read: 0,
            cwriter: 0,
            stalled: false,
            poison: None,
            consumed: Vec::new(),
            overwrites: 0,
        }
    }

    /// The driver writes command `id` into the slot at byte `offset`.
    pub fn store(&mut self, offset: u64, id: u64) {
        let slot = &mut self.unread[(offset / SLOT) as usize];
        if slot.is_some() {
            self.overwrites += 1;
        }
        *slot = Some(id);
    }

    pub fn post(&mut self, cwriter: u64) {
        self.cwriter = cwriter;
    }

    /// Read at most `budget` commands.
    pub fn step(&mut self, mut budget: usize) {
        while budget > 0 && !self.stalled && self.read != self.cwriter {
            let slot = &mut self.unread[(self.read / SLOT) as usize];
            let id = slot.expect("the ITS read a slot nothing wrote");
            // It stops on the command, which stays unread.
            if self.poison == Some(id) {
                self.stalled = true;
                return;
            }
            *slot = None;
            self.consumed.push(id);
            self.read = (self.read + SLOT) % BYTES;
            budget -= 1;
        }
    }

    pub fn creadr(&self) -> u64 {
        self.read | u64::from(self.stalled)
    }
}
