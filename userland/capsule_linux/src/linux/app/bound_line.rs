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

//! The line a store program's binding leaves in the serial log: who vouched
//! for its policy, the listing it was published under, and the services the
//! policy names and the ones it is served. No process is named, so two runs
//! of one program leave the same line.

use alloc::string::String;
use core::fmt::Write;

use nonos_linux_policy::mask::NAMES;

/// Who vouched for the policy a program is bound to (choose.rs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vouched {
    Vendor,
    Market,
    Default,
}

impl Vouched {
    /// The vendor's row wins over a file, as in choose.rs.
    pub fn of(vendor_row: bool, file: bool) -> Self {
        match (vendor_row, file) {
            (true, _) => Self::Vendor,
            (false, true) => Self::Market,
            (false, false) => Self::Default,
        }
    }
}

pub fn bound_line(from: Vouched, listing: &[u8; 32], policy: u64, served: u64) -> String {
    let from = match from {
        Vouched::Vendor => "vendor",
        Vouched::Market => "market",
        Vouched::Default => "default",
    };
    let mut s = String::new();
    let _ = write!(s, "[LINUX] policy bound: from={from} listing=");
    for b in listing {
        let _ = write!(s, "{b:02x}");
    }
    let _ = write!(s, " mask={policy:#x} served={served:#x}");
    for (name, _) in NAMES.iter().filter(|(_, bit)| served & bit != 0) {
        let _ = write!(s, " {name}");
    }
    s.push('\n');
    s
}
