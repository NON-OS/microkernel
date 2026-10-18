// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What a host adapter driver gives the command layer: one command at a
//! time to one logical unit, its data through a buffer the adapter owns.

/// Where a command goes: a target on the adapter's bus, a logical unit on
/// the target.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr {
    pub target: u16,
    pub lun: u16,
}

/// Which way the data phase moves, seen from the host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    None,
    /// From the device into the buffer (a read).
    In,
    /// From the buffer to the device (a write).
    Out,
}

/// The sense bytes kept from one command: enough for fixed format (18) and
/// the descriptor format's header with its first descriptor.
pub const SENSE_MAX: usize = 32;

/// A command the target answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    /// The SCSI status byte (`status`).
    pub status: u8,
    pub sense: [u8; SENSE_MAX],
    pub sense_len: usize,
    /// Bytes of the data phase the target did not move.
    pub residual: u32,
}

impl Outcome {
    pub const fn good() -> Self {
        Self { status: crate::status::GOOD, sense: [0; SENSE_MAX], sense_len: 0, residual: 0 }
    }
}

/// A command the target never answered, as the adapter reports it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// Nothing at that target or logical unit (selection timeout, bad
    /// target).
    NoTarget,
    /// The bus or the target was reset under the command: try again.
    Reset,
    /// The adapter is out of room for now: try again.
    Busy,
    /// The adapter gave up waiting; it has recovered itself before
    /// returning.
    Timeout,
    /// Anything else the adapter reports.
    Failed,
}

pub trait Hba {
    /// The data buffer: what a write sends is put here first, what a read
    /// brings is found here after.
    fn buffer(&mut self) -> &mut [u8];

    /// Run `cdb` on `at`, moving `len` bytes of the buffer in `dir`.
    fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault>;
}
