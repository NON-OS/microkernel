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

//! The exchange's stages and how it ends.

use alloc::vec::Vec;

use super::super::super::bounds::Bounds;
use super::super::super::read_tls_flight::FLIGHT_MAX;
use super::super::open::Opening;

/* A direct socket says nothing of a close, so its quiet window ends it. */
pub(super) const DIRECT: Bounds =
    Bounds { first_ms: 10_000, quiet_ms: 600, total_ms: 30_000, max: FLIGHT_MAX };

pub(super) enum Stage {
    Open(Opening),
    Hello,
    Send { next: Next },
    Flight,
    Check,
    Answer,
}

#[derive(Clone, Copy)]
pub(super) enum Next {
    Flight,
    Answer,
}

/// How a request ended.
pub enum Ended {
    /// The plaintext of the host's answer.
    Answer(Vec<u8>),
    /// Nothing of the request went: the reason, in the route's own words
    /// where it was the route.
    Failed(&'static str),
    /// The request had begun to go when the exchange broke, so the host may
    /// have acted on it, and why it broke. A broadcast that ends here must
    /// not be sent again.
    Unknown(&'static str),
}
