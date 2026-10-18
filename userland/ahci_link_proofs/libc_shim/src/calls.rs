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

//! What the log holds: a broker call, the device as the test's watch reads
//! it when the call is made, and whether the test had the broker refuse it.

/// A broker call; a PCI Command write carries the word written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Call {
    Claim,
    Command(u16),
    MmioMap,
    MmioUnmap,
    IrqBind,
    IrqUnbind,
    DmaMap,
    DmaUnmap,
    /// A DmaUnmap of a grant not live: given back twice, or never given.
    StaleUnmap,
    Release,
}

/// The device at one instant, as the test's model says: how many resets it
/// has finished (seen to end by the driver), and the engine bits it shows
/// running, none once its engines are stopped; for a device with several
/// ports, each of the first four ports' own bits too.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Engines {
    pub resets: u32,
    pub running: u32,
    pub ports: [u32; 4],
}

/// One call. A refused one changed nothing: a refused Command write left
/// the register as it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seen {
    pub call: Call,
    pub device: Engines,
    pub refused: bool,
}
