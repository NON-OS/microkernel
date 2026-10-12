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

//! How a link ends, and what a tick of the station's clock calls for.

use alloc::vec::Vec;

/// Why a link ended. The station is no longer associated after any of them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Loss {
    /// The access point deauthenticated or disassociated this station, with
    /// this reason code: in the clear without management frame protection,
    /// and protected (under the pairwise key or BIP) with it.
    Left(u16),
    /// With management frame protection, a deauthentication with this reason
    /// came in the clear and the SA Query it started went unanswered.
    Unanswered(u16),
    /// The access point went unheard, and the probes after it unanswered.
    Silent,
}

/// What a tick calls for.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tick {
    Quiet,
    /// Send this frame to the access point as it is.
    Send(Vec<u8>),
    /// The link ended.
    Lost(Loss),
}
