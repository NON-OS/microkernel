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

use crate::capabilities::{bits_to_caps, Capability, CapabilityToken};
use crate::syscall::numbers::SyscallNumber;

use super::super::cap_table::is_allowed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::syscall::contract) enum Verdict {
    /// EPERM, and the process runs on.
    Refuse,
    /// The process is ended.
    Kill,
}

/*
 * `envelope` is the manifest's required and optional bits within its
 * ceiling, before the boot profile trims them; zero for a process spawned
 * without a manifest. The Linux personality (the one holder of ForeignExec)
 * keeps EPERM: its guests' calls arrive through it. MkDebug is never a kill:
 * the profile withholds Debug from holders of secrets off the dev profile,
 * and a dropped log line is that call's documented refusal. The launcher
 * question is answered yes, so a launch call is never a kill on a fact the
 * capsule cannot see.
 */
pub(in crate::syscall::contract) fn verdict(envelope: u64, number: SyscallNumber) -> Verdict {
    if envelope == 0
        || envelope & Capability::ForeignExec.bit() != 0
        || number == SyscallNumber::MkDebug
    {
        return Verdict::Refuse;
    }
    let declared = CapabilityToken::with_caps(&bits_to_caps(envelope));
    if is_allowed(&declared, number, true) {
        Verdict::Refuse
    } else {
        Verdict::Kill
    }
}
