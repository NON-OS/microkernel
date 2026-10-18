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

//! What the link needs from the capsule around it: the clock, the input
//! ring, the debug line, the battery, the user, the random source pairing
//! draws from, the bonds, and through `Desk` the user's part. The proofs
//! stand in for all of it, so every path the link takes runs on the host.

use super::desk::Desk;
use nonos_bt_bond::BondRecord;
use nonos_bt_hogp::host::Battery;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::pairing::Random;
use nonos_bt_wire::addr::{AddrType, BdAddr};

pub trait Env: Random + Desk {
    /// Milliseconds since some fixed point; never goes back.
    fn now_ms(&mut self) -> u64;
    /// One event for the input ring, stamped with the posting link's trust
    ///: `authenticated` is its bond's level, SOURCE_BT is implied.
    fn input(&mut self, input: Input, authenticated: bool);
    /// Once a turn of the link's loop, for whatever repeats on the clock.
    fn tick(&mut self, now_ms: u64);
    /// One line for whoever reads the capsule's log.
    fn say(&mut self, words: &[&[u8]]);
    /// A Battery Level the HID device gave (BAS 1.0, 3.1).
    fn battery(&mut self, battery: Battery);
    /// Six digits for the user to type on the keyboard being paired.
    fn show_passkey(&mut self, passkey: u32);
    /// The bond for a peer seen at this address: by identity, or by the
    /// IRK that resolves it (Vol 6, Part B, 1.3.2.3).
    fn bonded(&mut self, kind: AddrType, addr: &BdAddr) -> Option<BondRecord>;
    /// Whether any bond is kept.
    fn any_bonds(&mut self) -> bool;
    /// Keep a new bond, in place of any older one for the same identity.
    fn keep(&mut self, record: BondRecord);
    /// Drop the bond for this record's identity.
    fn forget(&mut self, record: &BondRecord);
}
