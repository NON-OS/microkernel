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

//! What the device saw the driver do, and the faults a test can switch on.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// A command, with how many grants the driver had given back by then.
    Command {
        cmd: u32,
        grants_back: usize,
    },
    Shared(u64),
    Mac([u8; 6]),
    TxProd(u32),
    RxProd(u32),
    RxProd2(u32),
    Masked,
}

#[derive(Default, Clone, Copy)]
pub struct Faults {
    /// Offer only revision 2, which this driver does not know.
    pub no_rev1: bool,
    pub refuse_activation: bool,
    /// Stop both queues, as a device does on a descriptor it cannot use.
    pub stop_queues: bool,
}
