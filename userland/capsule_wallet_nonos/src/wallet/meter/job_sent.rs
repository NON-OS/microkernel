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

//! What a finished meter job sent, in words, and the address a sent close
//! makes due (v1.4.14, v1.4.60).

use alloc::format;
use alloc::string::String;

use super::{JobKind, MeterJob, Outcome};

fn short(h: &[u8]) -> String {
    let hex: String = h.iter().take(4).map(|b| format!("{b:02x}")).collect();
    format!("0x{hex}…")
}

pub(super) fn sent(kind: JobKind, b: &[u8]) -> Option<String> {
    Some(match (kind, b.len()) {
        (JobKind::TopUp, 64) => {
            format!("top-up sent: approve {}, topUp {}", short(&b[..32]), short(&b[32..]))
        }
        (JobKind::Close, 32) => format!("close sent: {}", short(b)),
        (JobKind::Claim, 32) => format!("claim sent: {}", short(b)),
        _ => return None,
    })
}

/// The address a finished job makes due: a close that was sent, to its
/// channel's reclaimTo, never zero.
pub fn closed_due(job: &MeterJob, o: &Outcome) -> Option<[u8; 20]> {
    let sent = job.kind == JobKind::Close && matches!(o, Outcome::Sent(_));
    job.reclaim_to.filter(|a| sent && *a != [0; 20])
}
