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

//! The SOCKS a guest speaks to 127.0.0.1:9050, read and answered: SOCKS5
//! with no authentication (RFC 1928), SOCKS4, and SOCKS4a, which Monero's
//! own `--proxy` client speaks. Only CONNECT, to an IPv4 address or a name,
//! is taken; the name stays a name. Every byte here comes from a guest, so
//! every length is checked before it is used, a short buffer means "more
//! to come", and a request is bounded at a few hundred bytes. Pure: the
//! proofs hold the very parser the capsule runs.

mod greet;
pub mod kinds;
pub mod parsed;
pub mod reply;
pub mod talk;
pub mod v4;
pub mod v5;

pub use kinds::{Connect, Heard, To, Ver};
pub use talk::Talk;
