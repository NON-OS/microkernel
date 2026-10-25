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

//! The receipt as a person reads it.
//!
//! Network reachability gets its own column instead of sitting inside a list
//! of twenty names. It is what nearly every reader is checking, and a property
//! you have to count out of a list is one nobody checks twice.

use crate::decode::{capability_names, reaches_network};
use crate::entries::Entry;
use crate::firmware::{firmware, programs, FIRMWARE_LABEL, FIRMWARE_WHEN};
use crate::hexcode::hex;
use crate::summary::summary;

/// Enough to compare against a published build by eye, not enough to mistake
/// for the whole measurement.
fn short(measurement: &[u8; 32]) -> String {
    hex(&measurement[..6])
}

/// Call only after the fold has matched. The firmware entry is folded like
/// the rest but is no program, so it is counted and listed apart.
pub fn render(entries: &[Entry]) -> String {
    let progs: Vec<&Entry> = programs(entries, |e| e.pid).collect();
    let fw = firmware(entries, |e| e.pid);
    let also = if fw.is_some() { " and the firmware tables" } else { "" };
    let mut out = format!("{} capsules{also}, all folded into the signed root\n\n", progs.len());
    out.push_str("  pid  measurement   network  capabilities\n");
    out.push_str("  ---  ------------  -------  ------------\n");
    for e in &progs {
        let net = if reaches_network(e.caps) { "YES" } else { "no" };
        out.push_str(&format!(
            "  {:>3}  {}  {:^7}  {}\n",
            e.pid,
            short(&e.measurement),
            net,
            capability_names(e.caps).join(", ")
        ));
    }
    if let Some(f) = fw {
        let (label, when) = (text(FIRMWARE_LABEL), text(FIRMWARE_WHEN));
        out.push_str(&format!("    -  {}     -     {label}, {when}\n", short(&f.measurement)));
        // Tables reach nothing; a mask here is the machine's fault, said aloud.
        if f.caps != 0 {
            let names = capability_names(f.caps).join(", ");
            out.push_str(&format!("  WARNING: the pid 0 entry carries capabilities: {names}\n"));
        }
    }
    out.push_str(&summary(&progs));
    out
}

fn text(b: &'static [u8]) -> &'static str {
    core::str::from_utf8(b).unwrap_or("?")
}
