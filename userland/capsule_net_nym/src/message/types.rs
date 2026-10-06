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

//! Constants the message layer is defined in terms of.

use crate::sphinx::constants::{HEADER_SIZE, NODE_ADDRESS_LENGTH};

/// A message that carries a sender tag and reply surbs, so the far end can
/// answer without ever learning who asked.
pub const TYPE_REPLIABLE: u8 = 1;

/// Content tag for a request with data attached, as opposed to one that only
/// tops up the far end's supply of reply surbs. This is the second form, whose
/// blocks carry a seed per hop and say how many hops they have; the first (0)
/// fixed every block at four full keys.
pub const TAG_DATA: u8 = 3;

/// Content tag for a message carrying nothing but reply blocks, sent when the
/// far end has asked for more. Second form, as above; the first was 1.
pub const TAG_ADDITIONAL_SURBS: u8 = 4;

/// The key rotation the blocks were built against, as the second form names
/// it. Unknown: a hop then tries its current key and, failing that, the one
/// before, so a block stays good across a rotation for as long as either is
/// held. Naming one would only shorten that.
pub const SURB_KEY_ROTATION_UNKNOWN: u8 = 0;

/// Bytes of a reply block before its per-hop seeds: its own key, the header,
/// and the address of the hop a reply enters by. What is left is the seeds,
/// which is how the second form counts a block's hops.
pub const SURB_BASE_BYTES: usize = 16 + HEADER_SIZE + NODE_ADDRESS_LENGTH;

/// Bytes of the tag a far end quotes to reach us again.
pub const SENDER_TAG_SIZE: usize = 16;
