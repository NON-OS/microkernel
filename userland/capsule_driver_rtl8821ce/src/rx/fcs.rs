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

//! The frame check sequence the chip leaves on every received frame. The RX
//! descriptor's packet length counts the 4-byte FCS (rtw88 registers the
//! radio with mac80211's RX_INCLUDES_FCS for this reason), and the chip has
//! already checked it (a bad one sets the descriptor's CRC32 flag and the poll
//! drops the frame). Left on, it ran into the data path as four bytes of junk
//! after every payload, and into an SAE anti-clogging token, which is the rest
//! of its frame and so came back to the access point four bytes too long.

/// The FCS length.
pub const FCS_LEN: usize = 4;

/// The length of a received frame of `n` bytes without its FCS, or `None` for
/// a frame too short to carry one.
pub fn without_fcs(n: usize) -> Option<usize> {
    n.checked_sub(FCS_LEN)
}
