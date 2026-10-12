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

//! VMBus's channel messages, its versions, the grant's layout and the
//! waits.

/// The SynIC message type every VMBus message travels as.
pub const MESSAGE_TYPE_CHANNEL: u32 = 1;
/// The connections the host takes channel messages on: 1 before 5.0, and
/// 4 for the contact from 5.0 (the host may then name another).
pub const CONNECTION_MESSAGE: u32 = 1;
pub const CONNECTION_MESSAGE_V5: u32 = 4;
/// The SINT the host is to send on, from 5.0.
pub const MESSAGE_SINT: u8 = 2;

pub const OFFERCHANNEL: u32 = 1;
pub const RESCIND_CHANNELOFFER: u32 = 2;
pub const REQUESTOFFERS: u32 = 3;
pub const ALLOFFERS_DELIVERED: u32 = 4;
pub const RELID_RELEASED: u32 = 12;
pub const INITIATE_CONTACT: u32 = 14;
pub const VERSION_RESPONSE: u32 = 15;
pub const UNLOAD: u32 = 16;
pub const UNLOAD_RESPONSE: u32 = 17;

/// The versions asked for, newest first: 5.3 down to 2.4 (Windows 8), as
/// Linux asks. 5.0 changed the contact; below 2.4 the host signals through
/// the interrupt page alone, which this core does not serve.
pub const VERSIONS: [u32; 8] =
    [0x5_0003, 0x5_0002, 0x5_0001, 0x5_0000, 0x4_0001, 0x4_0000, 0x3_0000, 0x2_0004];
pub const VERSION_V5: u32 = 0x5_0000;

/// One grant of three pages: the interrupt page the contact names before
/// 5.0, and the two monitor pages.
pub const PAGE: u64 = 4096;
pub const GRANT_BYTES: u64 = 3 * PAGE;
pub const INTERRUPT_PAGE: u64 = 0;
pub const MONITOR_PAGE_1: u64 = PAGE;
pub const MONITOR_PAGE_2: u64 = 2 * PAGE;

/// The channels kept: a Hyper-V guest has a few dozen at most.
pub const MAX_CHANNELS: usize = 64;

/// How long the host has to answer the contact, and to deliver its offers.
pub const REPLY_MS: u64 = 5_000;
pub const OFFERS_MS: u64 = 10_000;
/// How long a post may wait for room at the host.
pub const POST_MS: u64 = 1_000;
/// The serve loop's sleep with nothing waiting, and its bound per look.
pub const POLL_MS: u64 = 10;
pub const MESSAGES_PER_LOOK: usize = 16;
/// The first looks of a wait are a yield apart.
pub const QUICK_LOOKS: u32 = 64;
