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
//! Host proofs for the software AES.

//! net.nym's acknowledgement sealing and opening, compiled in unchanged, with
//! the sizes it is defined in.

#[path = "../../../capsule_net_nym/src/ack/open.rs"]
pub mod open;
#[path = "../../../capsule_net_nym/src/ack/plaintext.rs"]
pub mod plaintext;

pub mod types {
    pub const ACK_IV_BYTES: usize = 16;
    pub const FRAG_ID_BYTES: usize = 5;
    pub const ACK_PLAINTEXT_BYTES: usize = ACK_IV_BYTES + FRAG_ID_BYTES;
}
