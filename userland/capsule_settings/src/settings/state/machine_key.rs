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

//! What the kernel said when Security asked it for this machine's key.
//!
//! The key is the TPM's HMAC under the owner seed and the boot PCRs, the one
//! the saved Wi-Fi passphrases are sealed with. Its errnos are the kernel's
//! (nonos_libc `MACHINE_KEY_NO_TPM`, `MACHINE_KEY_WRONG_STATE`); the probe
//! that asks holds them to these at compile time (`machine_key_probe.rs`).

use crate::settings::schema::rows::Tone;

pub const NO_TPM: i64 = -19;
pub const WRONG_STATE: i64 = -13;
pub const TIMED_OUT: i64 = -110;
pub const REFUSED: i64 = -5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MachineKey {
    /// Not asked yet: the Security page has not been opened.
    Unasked,
    /// The TPM gave the key.
    Ready,
    /// There is no TPM to ask.
    NoTpm,
    /// The TPM refused: this boot's measured state is not the key's.
    BootChanged,
    /// The TPM did not finish a command in time.
    TimedOut,
    /// The TPM answered with an error code, or bytes that do not parse.
    Refused,
    /// Any other errno: the call itself failed.
    Failed,
}

/// The answer to one ask, from the call's return.
pub fn classify(rc: Result<(), i64>) -> MachineKey {
    match rc {
        Ok(()) => MachineKey::Ready,
        Err(NO_TPM) => MachineKey::NoTpm,
        Err(WRONG_STATE) => MachineKey::BootChanged,
        Err(TIMED_OUT) => MachineKey::TimedOut,
        Err(REFUSED) => MachineKey::Refused,
        Err(_) => MachineKey::Failed,
    }
}

/// The row's words and tone for what the kernel said.
pub fn said(key: MachineKey) -> (&'static str, Tone) {
    match key {
        MachineKey::Unasked => ("--", Tone::Idle),
        MachineKey::Ready => ("From the TPM, bound to this boot", Tone::Ok),
        MachineKey::NoTpm => ("No TPM on this machine", Tone::Warn),
        MachineKey::BootChanged => ("The TPM refused: the boot state changed", Tone::Warn),
        MachineKey::TimedOut => ("The TPM did not answer in time", Tone::Warn),
        MachineKey::Refused => ("The TPM refused it: log TPM in Terminal shows why", Tone::Warn),
        MachineKey::Failed => ("The machine key call failed", Tone::Warn),
    }
}
