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

//! The message layer of net.nym, the parts that stand alone: fragment
//! headers, padding, and the repliable message shapes.

#[path = "../../capsule_net_nym/src/message/fragment.rs"]
mod fragment;
#[path = "../../capsule_net_nym/src/message/fragment_parse.rs"]
mod fragment_parse;
#[path = "../../capsule_net_nym/src/message/repliable.rs"]
mod repliable;
#[path = "../../capsule_net_nym/src/message/types.rs"]
mod types;

pub use fragment::{Fragment, MAX_FRAGMENTS, UNLINKED_HEADER_LEN};
pub use fragment_parse::parse;
pub use repliable::{pad_to_packets, repliable_additional_surbs, repliable_data, unpad};
pub use types::{SENDER_TAG_SIZE, TAG_ADDITIONAL_SURBS, TAG_DATA, TYPE_REPLIABLE};
