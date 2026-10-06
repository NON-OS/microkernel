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

/* /proc/<pid>/limits. */

use alloc::string::String;
use alloc::vec::Vec;

/* Linux's names and units, in its order of RLIMIT numbers 0 to 15. */
const NAMES: [(&str, &str); 16] = [
    ("Max cpu time", "seconds"),
    ("Max file size", "bytes"),
    ("Max data size", "bytes"),
    ("Max stack size", "bytes"),
    ("Max core file size", "bytes"),
    ("Max resident set", "bytes"),
    ("Max processes", "processes"),
    ("Max open files", "files"),
    ("Max locked memory", "bytes"),
    ("Max address space", "bytes"),
    ("Max file locks", "locks"),
    ("Max pending signals", "signals"),
    ("Max msgqueue size", "bytes"),
    ("Max nice priority", ""),
    ("Max realtime priority", ""),
    ("Max realtime timeout", "us"),
];

/* The same limits getrlimit answers. */
pub fn limits() -> Vec<u8> {
    let mut s = alloc::format!(
        "{:<25} {:<20} {:<20} {:<10}\n",
        "Limit",
        "Soft Limit",
        "Hard Limit",
        "Units"
    );
    let shown = |v: u64| match v {
        u64::MAX => String::from("unlimited"),
        n => alloc::format!("{n}"),
    };
    for (i, (name, unit)) in NAMES.iter().enumerate() {
        let (soft, hard) = crate::linux::call::limit_for(i as u64).unwrap_or((u64::MAX, u64::MAX));
        s += &alloc::format!("{:<25} {:<20} {:<20} ", name, shown(soft), shown(hard));
        s += &match unit.is_empty() {
            true => String::from("\n"),
            false => alloc::format!("{unit:<10}\n"),
        };
    }
    s.into_bytes()
}
