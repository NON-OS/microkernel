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

//! Link control messages (Vol 4, Part D, "Link Establishment" and "Low
//! Power"). Each is the payload of an unreliable packet of type 15: two
//! octets naming it and, for the two configuration messages, the
//! configuration field.

use alloc::vec::Vec;

pub const SYNC: [u8; 2] = [0x01, 0x7e];
pub const SYNC_RESPONSE: [u8; 2] = [0x02, 0x7d];
pub const CONFIG: [u8; 2] = [0x03, 0xfc];
pub const CONFIG_RESPONSE: [u8; 2] = [0x04, 0x7b];
pub const WAKEUP: [u8; 2] = [0x05, 0xfa];
pub const WOKEN: [u8; 2] = [0x06, 0xf9];
pub const SLEEP: [u8; 2] = [0x07, 0x78];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Message {
    Sync,
    SyncResponse,
    /// With its configuration field, if it carried one.
    Config(Option<u8>),
    ConfigResponse(Option<u8>),
    Wakeup,
    Woken,
    Sleep,
}

/// The message in `p`, or `None` for one this host does not know or one
/// with octets past its end.
pub fn parse(p: &[u8]) -> Option<Message> {
    let (name, rest) = p.split_first_chunk::<2>()?;
    let field = match rest {
        [] => None,
        [f] => Some(*f),
        _ => return None,
    };
    let bare = |m: Message| field.is_none().then_some(m);
    match *name {
        SYNC => bare(Message::Sync),
        SYNC_RESPONSE => bare(Message::SyncResponse),
        CONFIG => Some(Message::Config(field)),
        CONFIG_RESPONSE => Some(Message::ConfigResponse(field)),
        WAKEUP => bare(Message::Wakeup),
        WOKEN => bare(Message::Woken),
        SLEEP => bare(Message::Sleep),
        _ => None,
    }
}

/// A configuration message with its field.
pub fn with_field(name: [u8; 2], field: u8) -> Vec<u8> {
    [&name[..], &[field]].concat()
}
