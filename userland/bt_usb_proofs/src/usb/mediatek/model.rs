/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A modelled MT7921/MT7922 behind its default endpoint, written from what
//! the chip takes rather than from the driver: its registers
//! (`model_port.rs`), WMT commands answered on the control pipe after some
//! empty reads (`model_wmt.rs`), and the package with a real SHA-256.
//! `broken` names the first thing the driver did the chip would not take.

use std::collections::BTreeMap;

use super::reg::{DEV_ID, FW_FLAVOR, FW_VERSION};

#[derive(Default)]
pub struct Model {
    pub regs: BTreeMap<u32, u32>,
    pub writes: Vec<(u32, u32)>,
    /// Looks at `BT_MISC` after a reset before it says done; `None` never.
    pub reset_takes: Option<usize>,
    pub misc_looks: usize,
    pub files: BTreeMap<u64, Vec<u8>>,
    pub clock_ms: u64,
    /// Empty answer reads before each WMT answer; `None` never.
    pub answer_after: Option<usize>,
    pub empty: usize,
    pub pending: Option<Vec<u8>>,
    /// "In progress" answers given to looks first; a status for every look.
    pub held: usize,
    pub odd: Option<u8>,
    /// The sections the chip holds, by load address; the one being sent.
    pub patched: BTreeMap<u32, Vec<u8>>,
    pub loading: Option<(u32, usize, Vec<u8>)>,
    /// Each block's flag and length, in order; the digests taken.
    pub blocks: Vec<(u8, usize)>,
    pub digests: usize,
    pub func_on: bool,
    pub broken: Option<&'static str>,
}

impl Model {
    pub fn chip(id: u32, version: u32, flavour: u32) -> Self {
        let regs = [(DEV_ID, id), (FW_VERSION, version), (FW_FLAVOR, flavour)].into();
        Model { regs, reset_takes: Some(3), answer_after: Some(2), ..Default::default() }
    }

    pub fn mt7922() -> Self {
        Model::chip(0x7922, 0x8a00, 0)
    }

    pub fn mt7921() -> Self {
        Model::chip(0x7961, 0x8a01, 0)
    }

    pub fn wrong(&mut self, why: &'static str) {
        self.broken.get_or_insert(why);
    }

    /// The answer pipe: empty until `answer_after` reads have passed.
    pub fn answer(&mut self, out: &mut [u8]) -> usize {
        match (self.answer_after, self.pending.take()) {
            (Some(after), Some(ev)) if self.empty >= after => {
                self.empty = 0;
                out[..ev.len()].copy_from_slice(&ev);
                ev.len()
            }
            (_, ev) => {
                self.empty += ev.is_some() as usize;
                self.pending = ev;
                0
            }
        }
    }
}
