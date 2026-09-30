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

//! Why a market call brought back no body, and what the store says for it
//! in place of the catalogue.

use alloc::format;
use alloc::vec::Vec;

/// A market call that brought back nothing to read.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Failure {
    /// No reply: no port, the call failed or timed out, or the reply was too
    /// short to carry a status.
    NoReply,
    /// A reply whose status was not zero.
    Status(i32),
    /// A zero status and a body that does not parse.
    Malformed,
}

/// The statuses the market replies with, as `capsule_market`'s
/// `protocol/errno.rs` defines them.
const NAMES: [(i32, &str); 5] = [
    (-22, "EINVAL"),
    (-61, "ENODATA"),
    (-90, "EMSGSIZE"),
    (-116, "ESTALE"),
    (-129, "EKEYREJECTED"),
];

/// ENODATA on a catalogue call: the market holds no accepted catalogue, as
/// on an image built without the marketplace operator's key.
const E_NODATA: i32 = -61;

impl Failure {
    /// What the catalogue pane says, one line per `\n`.
    pub fn catalogue_trouble(self) -> Vec<u8> {
        match self {
            Failure::NoReply => b"market did not answer".to_vec(),
            Failure::Status(E_NODATA) => b"this machine has no signed catalogue (ENODATA)\n\
                Qwen still runs from the Terminal: type qwen, or qwen window"
                .to_vec(),
            Failure::Status(status) => match NAMES.iter().find(|(n, _)| *n == status) {
                Some((_, name)) => format!("market refused the catalogue: {name} ({status})"),
                None => format!("market refused the catalogue with status {status}"),
            }
            .into_bytes(),
            Failure::Malformed => b"market sent a catalogue this store cannot read".to_vec(),
        }
    }
}
