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

//! The knobs of the undo checks (plan section 6): each puts one broken design
//! back into the seeded model, which must then break the invariant named.
//! `Built` is the kernel as built.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) enum Knob {
    #[default]
    Built,
    /// 1: unconfined frames freed at teardown, no record kept (I2).
    FreeAtTeardown,
    /// 4: kept frames settled by their device alone, not its epoch (I2).
    SettleNoEpoch,
    /// 5: the Vendor ID read 99 ms after an FLR (I3).
    VendorAt99,
    /// 6: a display function's clean reset trusted for any service (I1).
    DisplayTrusted,
    /// 8: frames freed though the function reaches past Bus Master (I2).
    FreeWhenBinds,
    /// 9: any answer after a reset taken as its proof, no marker read (I1).
    NoMarker,
    /// 10: no busy mark, so two steppers may act on one record (I4).
    NoToken,
    /// 13: a lost function refused for good, never asked again (I8).
    NoRearm,
    /// 14: the reset written over Transactions Pending (I4).
    NoTpWait,
    /// Not a mutant: the kernel as built, with a step stalled between its
    /// clock read and its act, as a sibling function's settle can stall it.
    Stall,
    /// That stall, with the clock read again for the act and the put.
    StallFresh,
}
