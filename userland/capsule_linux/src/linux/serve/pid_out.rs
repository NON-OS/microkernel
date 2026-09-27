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

//! The calls that hand a pid back, and the number the guest sees in it.

use crate::linux::abi::{nr, nr_path as np};

use super::pid_ns::PidNs;

/// A returned pid in the guest's terms; every other value passes unchanged.
pub fn value_out(ns: &mut PidNs, call: u64, v: u64) -> u64 {
    let returns_pid =
        matches!(
            call,
            nr::FORK
                | nr::VFORK
                | nr::CLONE
                | nr::GETPID
                | nr::GETTID
                | nr::SET_TID_ADDRESS
                | nr::WAIT4
        ) || matches!(call, np::GETPPID | np::GETPGRP | np::GETPGID | np::GETSID | np::SETSID);
    match u32::try_from(v) {
        Ok(k) if returns_pid && k > 0 => u64::from(ns.outward(k)),
        _ => v,
    }
}
