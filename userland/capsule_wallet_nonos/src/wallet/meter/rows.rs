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

/*
 * The meter's ACCOUNT reply as lines a person reads (B8). Pure: the NOX
 * figures come through `nox`, so wallet_proofs reads real replies with it.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_payment_proto::{Account, KEPT, READY};

const DAY: u64 = 86_400;

/// How the meter stands: answering, its key open, kept past power off.
pub fn state_line(a: Option<&Account>) -> &'static str {
    match a {
        None => "did not answer",
        Some(a) if a.flags & READY == 0 => {
            "no key open: a boot that keeps nothing holds no meter key, and pays nothing"
        }
        Some(a) if a.flags & KEPT == 0 => "open for this session only",
        Some(_) => "open, kept on this machine",
    }
}

fn ends(expiry: u64, now: u64) -> String {
    match expiry {
        0 => String::from("not set"),
        e if e <= now => String::from("ended: nothing is paid until it is set again"),
        e if e - now < DAY => format!("in {} hours", (e - now) / 3600),
        e => format!("in {} days", (e - now) / DAY),
    }
}

/// The budget, as (label, value) rows. A cap of zero pays nothing.
pub fn budget_rows(a: &Account, now: u64, nox: fn(u128) -> String) -> Vec<(&'static str, String)> {
    if a.cap == 0 {
        return Vec::from([("budget", String::from("none set: the meter pays nothing"))]);
    }
    Vec::from([
        ("budget", format!("{} NOX", nox(a.cap))),
        ("spent", format!("{} NOX", nox(a.spent))),
        ("left", format!("{} NOX", nox(a.left))),
        ("budget ends", ends(a.expiry, now)),
        ("channels", format!("{}", a.total)),
    ])
}
