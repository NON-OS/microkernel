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

//! Cookies, held in memory, one jar per network.
//!
//! A site that cannot keep a session cookie cannot keep anyone signed in,
//! and a consent banner that cannot remember its answer asks again on every
//! page. The jar is the RFC 6265 model with its size bounded.
//!
//! Each network has its own jar: a cookie set while browsing over Nym is
//! never sent over Direct or Anyone, or the cookie would link the reader's
//! anonymous visits to their direct ones. Nothing here is written anywhere:
//! the jars live in this capsule's memory and are gone when it exits.

pub mod absorb;
pub mod civil;
pub mod clock;
pub mod date;
pub mod jar;
pub mod jars;
pub mod matching;
pub mod parse;
pub mod store;
pub mod types;

pub use clock::unix_secs;
pub use store::{absorb, request_header, script_get, script_set, set_page};
