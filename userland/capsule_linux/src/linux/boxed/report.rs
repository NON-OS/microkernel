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

//! The BOX REPORT a boxed run ends with, for the person who ran it, and the
//! one `[BOX]` line with the same summary for the serial log. The counts are
//! capsule_linux's own enforcement; the kernel's sealed histogram counts a
//! refusal by its errno and is not what these are.

use alloc::string::String;
use core::fmt::Write;

use super::allowed::allowed;
pub use super::end::End;
use super::kind::ALL;
use super::preset::{BoxPolicy, Preset};
use super::tally::Tally;

/// The report's and the serial line's format: a reader refuses any other.
pub const VERSION: u32 = 1;

pub const SOURCE: &str =
    "counts are capsule_linux's enforcement, not the kernel's sealed histogram";

pub fn preset_name(p: Preset) -> &'static str {
    match p {
        Preset::Strict => "strict",
        Preset::Offline => "offline",
        Preset::Allow => "allow",
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| alloc::format!("{b:02x}")).collect()
}

/// `granted`: the services the program got in the box, None if it never started.
pub fn report(b: &BoxPolicy, hash: &[u8; 32], t: &Tally, granted: Option<u64>, e: &End) -> String {
    let mut s = alloc::format!("BOX REPORT v{VERSION}\n");
    let _ = writeln!(s, "  policy   {} {}", preset_name(b.preset), hex(hash));
    for line in allowed(b, granted) {
        let _ = writeln!(s, "  allowed  {line}");
    }
    let _ = writeln!(s, "  refused  {}", t.total());
    for kind in ALL.iter().filter(|k| t.count(**k) > 0) {
        let _ = write!(s, "    {:<9}{:>6}", kind.tag(), t.count(*kind));
        for (i, e) in t.examples(*kind).iter().enumerate() {
            let sep = if i == 0 { "  " } else { ", " };
            let _ = write!(s, "{sep}{}", String::from_utf8_lossy(e));
        }
        s.push('\n');
    }
    if let Some(why) = e.not_started {
        let _ = writeln!(s, "  started  no: {why}");
    }
    let _ = writeln!(s, "  exit     {}", e.exit);
    let _ = writeln!(s, "  {SOURCE}");
    s
}
