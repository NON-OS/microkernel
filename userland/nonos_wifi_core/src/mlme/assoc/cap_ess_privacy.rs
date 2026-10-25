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

//! Association. The request carries the SSID, the access point's own legacy
//! rates (so its basic rates are always among them), the station's RSNE for
//! the selection and, when SAE runs hash-to-element, the station's RSNXE;
//! message 2 repeats the same RSNE and RSNXE. A response counts only from the
//! chosen BSS to this station. Status 0 starts the four-way handshake with a
//! supplicant that holds the beacon's elements; status 30 ("try again later",
//! sent while the AP checks an earlier association with management frame
//! protection) keeps waiting so the retransmitted request can be answered;
//! any other status fails the join with that code.

/// Capability information: ESS, and Privacy (an RSN network is a private one).
pub(super) const CAP_ESS_PRIVACY: u16 = 0x0011;

/// Status 30: association request rejected temporarily; try again later.
pub(super) const STATUS_REJECTED_TEMPORARILY: u16 = 30;
