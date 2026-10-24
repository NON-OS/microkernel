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

//! What the tool prints for each outcome, and its exit code.

use crate::{RecordRefusal, Report, Verdict};
use alloc::format;
use alloc::string::String;

pub(super) const USAGE: &str = concat!(
    "usage: nsh-verify-record <file> --doc <doc> --fleet <32 hex> --authority <key> ",
    "[--ckpt <file>]... [--revoked <file>]... [--session <32 hex>] [--subject <128 hex>]"
);

const REGISTRY: &str =
    "registry entries not checked offline; the registry root is bound by the quote";

pub(super) fn show(r: Result<Report, RecordRefusal>) -> (i32, String) {
    let rep = match r {
        Ok(rep) => rep,
        Err(why) => return (1, format!("record refused: {why:?}")),
    };
    let whose = format!(
        "session {} subject {} start_ms {}",
        hex(&rep.session_id),
        hex(&rep.subject),
        rep.start_ms
    );
    let (code, verdict) = match rep.verdict {
        Verdict::Complete { entries, end_reason, boxed: false, .. } => {
            (0, format!("record complete: {entries} entries, end reason {end_reason}"))
        }
        Verdict::Complete { entries, end_reason, boxed: true, .. } => (
            4,
            format!(
                "record complete: {entries} entries, end reason {end_reason}; \
                 holds provisional BOX entries"
            ),
        ),
        Verdict::CutShort { seq, boxed } => {
            let note = if boxed { "; holds provisional BOX entries" } else { "" };
            (3, format!("cut short at seq {seq}{note}"))
        }
    };
    (code, format!("{whose}\n{verdict}\n{REGISTRY}"))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
