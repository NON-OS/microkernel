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

//! Turning the tree's idle states on, once PSCI and the broadcast frame are
//! known.

use super::allow::allow;
use super::psci_format::suspend_format;
use super::select::allowed;
use super::table::{EXTENDED, TREE};
use crate::arch::aarch64::timer::broadcast;
use crate::sys::serial::Line;

/// Turn the states on once PSCI says CPU_SUSPEND is there and in which
/// format it reads power_state. Any doubt leaves every CPU on a plain WFI.
pub fn prepare() {
    let Some(tree) = TREE.get().filter(|t| t.count > 0) else {
        return;
    };
    let mut line = Line::new();
    line.str(b"[IDLE] ");
    // The frame first: whether timer-stop states are usable depends on it.
    let frame = broadcast::start();
    let extended = suspend_format();
    match extended {
        Ok(extended) => {
            EXTENDED.call_once(|| extended);
            let allow = allow();
            let usable = tree.states().iter().filter(|s| allowed(s, extended, allow)).count();
            line.str(b"states=").dec(tree.count as u64).str(b" usable=").dec(usable as u64);
            line.str(if extended { b" format=extended" } else { b" format=original" });
            line.str(if allow.powerdown { b" powerdown=resumable" } else { b" powerdown=no" });
            match frame {
                Ok(number) => _ = line.str(b" broadcast=frame").dec(u64::from(number)),
                Err(reason) => _ = line.str(b" broadcast=none (").str(reason).str(b")"),
            }
        }
        Err(reason) => _ = line.str(reason).str(b"; WFI only"),
    }
    line.end();
}
