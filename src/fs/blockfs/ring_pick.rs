// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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
//! Which header the volume mounts from, given what each slot of the header
//! ring gave. Only a slot that holds no header for this key is passed over:
//! never written, written under another key, torn, or not a header. A slot
//! the device failed to read fails the mount. Passed over, an unread newest
//! header would mount an older generation without a word, and that header's
//! free space starts below blocks the newer generations wrote, so the next
//! write would go over them. A header in a slot its generation does not map
//! to (`header_lba`) was not put there by a commit, and is passed over too.
//! Pure, for the proofs.

/// What reading one slot of the ring came to.
pub(super) enum SlotRead<H, E> {
    /// A header that opened under the key and parsed.
    Header { at: u64, generation: u64, header: H },
    /// No header for this key in the slot.
    NotOurs,
    /// The device failed the read.
    Failed(E),
}

/// The newest header, None when no slot holds one, or the first device
/// failure met.
pub(super) fn pick<H, E>(
    slots: impl IntoIterator<Item = SlotRead<H, E>>,
    ring: u64,
) -> Result<Option<H>, E> {
    let mut best: Option<(u64, H)> = None;
    for slot in slots {
        match slot {
            SlotRead::Failed(e) => return Err(e),
            SlotRead::NotOurs => {}
            SlotRead::Header { at, generation, .. } if ring == 0 || generation % ring != at => {}
            SlotRead::Header { generation, header, .. } => {
                if best.as_ref().is_none_or(|(newest, _)| generation > *newest) {
                    best = Some((generation, header));
                }
            }
        }
    }
    Ok(best.map(|(_, header)| header))
}
