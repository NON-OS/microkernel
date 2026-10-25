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

//! No capsule that takes keys or input, or holds a secret, may write to the
//! serial console, except on the dev profile (U3). A standard image grants
//! Debug to every capsule whose signed manifest asks for it
//! (`capsule-serial-debug`), and an input driver or the prover that holds
//! the device secret could then copy what it sees to a line anyone with a
//! cable can read. The manifest's optional Debug stays its ceiling; the
//! kernel withholds it here, where every verified capsule's caps are set,
//! so a capsule added later is held to the same rule. Pure: `kernel_proofs`
//! mounts it.

/// `Capability::Debug`, restated so the rule stays pure.
pub(crate) const DEBUG: u64 = 1 << 8;
/// `InputSource`, `Keyring`, `DeviceSecret` and `InputConsumer`.
pub(crate) const SECRETS: u64 = 1 << 21 | 1 << 28 | 1 << 35 | 1 << 36;

/// `caps` with Debug withheld when they also hold a secret, unless `dev`.
pub(crate) const fn unlogged(caps: u64, dev: bool) -> u64 {
    if dev || caps & SECRETS == 0 {
        caps
    } else {
        caps & !DEBUG
    }
}
