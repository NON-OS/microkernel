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
 * Whether a parsed catalogue's entries may be believed: every file one the
 * data volume can keep, then one the pins name, with the pin's tier, length
 * and SHA-256, and a mirror to fetch it from; and every tier's memory the
 * one `need.rs` works out, the figure a download is refused by and the
 * store shows. A name the volume cannot keep is refused by that reason
 * before its pin is looked for, so a catalogue built to another rule fails
 * at load rather than file by file in the middle of a download.
 * Pure, so model_fetch_proofs runs it on the host tool's own catalogue.
 */

use super::types::Catalogue;
use crate::need::memory;
use crate::pins::pin_of;

pub fn admit(cat: &Catalogue) -> Result<(), &'static str> {
    for tier in &cat.tiers {
        for f in &tier.files {
            if !f.keepable() {
                return Err("the model catalogue names a file the data volume cannot keep");
            }
            let pinned = pin_of(&f.volume_name())
                .is_some_and(|p| p.tier == tier.word && p.bytes == f.bytes && p.sha256 == f.sha256);
            if !pinned || f.mirrors.is_empty() {
                return Err("the model catalogue names a file the signed pins do not");
            }
        }
        /* Every file is its pin by now, so the tier's bytes are the pins'. */
        if memory(&tier.word, tier.bytes()) != Some(tier.memory) {
            return Err("the model catalogue gives a tier a memory need its shape does not");
        }
    }
    Ok(())
}
