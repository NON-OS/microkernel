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

//! The connect's key slots and time budgets.

/// The pairwise key's index. The group key goes in the slot its own index
/// names: with the engine looking group-addressed frames up by the CCMP
/// KeyID, a fixed slot 1 went dark after the AP's first rekey moved it to 2.
pub(super) const PAIRWISE_KEY_ID: u8 = 0;
/// Milliseconds of uptime to spend joining before giving up. Long enough to
/// cover authentication (SAE adds a round trip), association and the four-way
/// handshake with the association driver's retransmits (an access point resends
/// message 1 about once a second, four times), while the hunt and the join
/// together still return within the panel's 30 s connect IPC timeout.
/// It was a count of receive passes, which shrank in wall time when every core
/// started running and the passes came faster.
pub(super) const JOIN_BUDGET_MS: u64 = 8_000;
/// The CAM entries reserved for group keys, by key index.
pub(super) const GROUP_KEY_SLOTS: u8 = 4;
