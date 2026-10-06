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

//! Why a join ended without a link, kept so the driver can tell the panel
//! more than "it failed": the refusal's own status or reason code where the
//! access point gave one, and which step refused otherwise.

use crate::rsn::SelectError;
use crate::sae::SaeFailure;
use crate::wpa::supplicant::Failure;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MlmeFailure {
    /// The network advertises no RSN element: an open network, which this
    /// station does not join.
    OpenNetwork,
    /// The beacon's RSN element is malformed.
    MalformedRsne,
    /// The security the network offers cannot be used (or, for a network
    /// saved as WPA3, would be a downgrade).
    Select(SelectError),
    /// The access point admits only 802.11n stations.
    NeedsHt,
    /// The passphrase is not 8 to 63 characters or 64 hex digits (PSK), or is
    /// longer than SAE is given room for.
    BadPassphrase,
    /// SAE was selected but the caller supplied no randomness for it.
    NoEntropy,
    /// Open System authentication was refused with this status code.
    AuthRejected(u16),
    /// The SAE exchange failed.
    Sae(SaeFailure),
    /// Association was refused with this status code.
    AssocRejected(u16),
    /// The access point deauthenticated or disassociated the station during
    /// the join, with this reason code (15, a four-way handshake timeout, is
    /// what a wrong WPA2 passphrase looks like from the station).
    Left(u16),
    /// The four-way handshake failed on an authentic message.
    Handshake(Failure),
}
