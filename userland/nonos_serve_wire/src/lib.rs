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

//! The wire between the capsule a box's SERVE_ONE names and the Linux
//! personality serving that box's one loopback port
//! (docs/design/serve-one.md). The capsule calls the endpoint
//! `linux.serve.<its name>` with one frame and gets one frame back.
//!
//! Every frame, asked or answered, is a 12-byte header and then `len` bytes:
//! `"NSO1" | op u8 | status u8 | conn u16 LE | len u32 LE`. A question's
//! status is 0. An answer carries the question's op and conn back, and its
//! status says how it went: OK, CLOSED (the guest closed its end; for READ,
//! after every byte it wrote was read), or a Linux errno.
//!
//! | Op | Question body | Answer body when OK |
//! |---|---|---|
//! | OPEN | none | none; conn is the new connection |
//! | WRITE | 1 to 65,536 bytes for the guest | u32 LE: how many it took, 0 when its buffer is full |
//! | READ | want u32 LE, wait_ms u32 LE (5,000 at most) | what the guest wrote, at most `want`; none when the wait ran out |
//! | CLOSE | none | none; the guest reads end of file |

#![cfg_attr(not(test), no_std)]

mod frame;
mod ops;
mod put;

pub use frame::{decode, request, Header, Refused};
pub use ops::*;
pub use put::put;

#[cfg(test)]
mod tests;
