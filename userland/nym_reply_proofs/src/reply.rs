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

//! The reply layer of net.nym, the parts that stand alone: reassembly and
//! reading what a reassembled reply says. Opening a reply needs the key
//! store and the cipher, so it is not taken here.

#[path = "../../capsule_net_nym/src/reply/assembly.rs"]
mod assembly;
#[path = "../../capsule_net_nym/src/reply/message.rs"]
mod message;
#[path = "../../capsule_net_nym/src/reply/reassemble.rs"]
mod reassemble;
#[path = "../../capsule_net_nym/src/reply/types.rs"]
mod types;

pub use message::{reply_body, reply_message, Reply};
pub use reassemble::{
    Collected, Reassembly, MAX_PENDING, MESSAGE_MAX, PENDING_BYTES_MAX, RECENT_DONE, STALE_MS,
};
pub use types::{
    DIGEST_BYTES, RECIPIENT_BYTES, TAG_REPLY_DATA, TAG_REPLY_SURB_REQUEST, TYPE_REPLY,
};
