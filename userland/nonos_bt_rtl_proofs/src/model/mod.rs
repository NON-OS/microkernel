// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! A modelled RTL8821C on USB, as btrtl.c describes it: its ROM answers
//! Read Local Version as the RTL8821C's and Read ROM Version with its
//! version; it takes the patch by 0xfc20 fragments, checking each as it
//! comes (`answer`), and once the last one completes an image it accepts,
//! it runs that patch and reports its version instead (`wire`).

mod answer;
mod transport;
mod wire;

use std::collections::VecDeque;

pub struct Rtl {
    /// The image this ROM accepts: the patch for it, then the config.
    pub accepts: Vec<u8>,
    /// The patch's version, once one runs.
    pub running: Option<u32>,
    /// What Read ROM Version returns after its status.
    pub rom_reply: Vec<u8>,
    /// What each fragment is answered with after its status, if not its index.
    pub download_reply: Option<Vec<u8>>,
    /// The fragment, by its place in the download, this chip refuses.
    pub refuse_at: Option<usize>,
    /// Every command, opcode and parameters, in order.
    pub sent: Vec<(u16, Vec<u8>)>,
    /// The image as the fragments brought it.
    pub got: Vec<u8>,
    /// The next fragment's place, and the index the chip counts for it.
    pub(crate) place: usize,
    pub(crate) counted: u8,
    /// A fragment out of order, of the wrong size, or after the last.
    pub broken: Option<&'static str>,
    pub(crate) events: VecDeque<Vec<u8>>,
}

impl Rtl {
    pub fn new(rom: u8, accepts: Vec<u8>) -> Self {
        Self {
            accepts,
            running: None,
            rom_reply: vec![rom],
            download_reply: None,
            refuse_at: None,
            sent: Vec::new(),
            got: Vec::new(),
            place: 0,
            counted: 0,
            broken: None,
            events: VecDeque::new(),
        }
    }

    /// The opcodes sent, in order.
    pub fn opcodes(&self) -> Vec<u16> {
        self.sent.iter().map(|(o, _)| *o).collect()
    }

    /// The index octet of every fragment sent, in order.
    pub fn indices(&self) -> Vec<u8> {
        self.sent.iter().filter(|(o, _)| *o == 0xfc20).map(|(_, p)| p[0]).collect()
    }
}
