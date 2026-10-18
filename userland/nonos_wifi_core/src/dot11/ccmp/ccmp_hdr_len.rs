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

//! CCMP-128 on a received MPDU of any shape (IEEE Std 802.11-2020,
//! 12.5.3.3). Access points send QoS data, whose header is 26 octets (30 with
//! an HT Control field), and the CCM nonce and AAD depend on it: the nonce's
//! priority octet is the TID, the AAD masks the subtype bits and the Order
//! bit and appends the QoS Control field with only the TID kept. `data.rs`
//! frames the station's own transmissions, which are non-QoS; this reads
//! whatever the AP sent. The view refuses a frame too short for its header,
//! CCMP header and MIC, and a CCMP header without ExtIV.

/// The CCMP header and MIC lengths.
pub const CCMP_HDR_LEN: usize = 8;

pub const MIC_LEN: usize = 8;

/// Frame-control bits.
pub(super) const FC_RETRY: u16 = 0x0800;

pub(super) const FC_PWR_MGT: u16 = 0x1000;

pub(super) const FC_MORE_DATA: u16 = 0x2000;

pub(super) const FC_PROTECTED: u16 = 0x4000;

pub(super) const FC_ORDER: u16 = 0x8000;

/// The QoS subtype bit (bit 7 of the frame control's first octet).
pub(super) const SUBTYPE_QOS: u8 = 0x08;

/// The HT Control field a QoS frame with the Order bit carries.
pub(super) const HT_CONTROL_LEN: usize = 4;

/// The ExtIV bit of the CCMP header's key-id octet.
pub(super) const EXT_IV: u8 = 0x20;

/// The parts of a received frame CCMP needs, all bounds-checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameView {
    pub fc: u16,
    /// MAC header length, QoS Control, Address 4 and HT Control included.
    pub hdr_len: usize,
    /// QoS Control field offset, for a QoS data frame.
    pub qos_at: Option<usize>,
    /// The TID (QoS priority), zero for a non-QoS frame.
    pub tid: u8,
}
