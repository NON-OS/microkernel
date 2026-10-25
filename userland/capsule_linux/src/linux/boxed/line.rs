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

//! The `[BOX]` line a boxed run leaves in the serial log, for automation:
//! `v=1`, then key=value pairs, every kind's count always present.

use alloc::string::String;
use core::fmt::Write;

use super::kind::ALL;
use super::preset::BoxPolicy;
use super::report::{hex, preset_name, End, VERSION};
use super::tally::Tally;

/// The serial log's line: the policy, every kind's count, and the exit status.
pub fn box_line(policy: &BoxPolicy, hash: &[u8; 32], tally: &Tally, e: &End) -> String {
    let mut s = String::new();
    let _ = write!(s, "[BOX] v={VERSION} preset={}", preset_name(policy.preset));
    s.push_str(super::learn_report::enforced(policy));
    let _ = write!(s, " policy={}", hex(hash));
    let _ = write!(s, " refused={}", tally.total());
    for kind in ALL {
        let _ = write!(s, " {}={}", kind.tag(), tally.count(kind));
    }
    let started = if e.not_started.is_some() { "no" } else { "yes" };
    let _ = writeln!(s, " started={started} exit={} source=capsule_linux", e.exit);
    s
}
