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

//! The WPA2 data plane: convert an ethernet frame to an 802.11 data frame and
//! back, so once the link is up the IP stack rides the radio unchanged.
//!
//! Framing and encryption are split so a chip that does CCMP in hardware and a
//! chip that does it in software share the same code without one double
//! encrypting:
//!
//!   * `build_data` produces the plaintext 802.11 MPDU (MAC header, LLC/SNAP,
//!     ethertype, payload). A hardware-CCMP chip hands exactly this to the
//!     radio, which inserts the CCMP header and encrypts from its key cache.
//!   * `protect` is software CCMP: it takes that plaintext MPDU and returns the
//!     encrypted frame. A chip without crypto uses `build_data` then `protect`.
//!   * `unprotect` is the software receive half: decrypt and strip CCMP,
//!     yielding a plaintext MPDU; a hardware-CCMP chip skips it.
//!   * `parse_data` turns a plaintext MPDU back into an ethernet frame.
//!
//! `encap`/`decap` remain as the software round trip (`build_data` + `protect`,
//! `unprotect` + `parse_data`) so their byte output is unchanged. Encryption is
//! CCMP (AES-CCM with an 802.11-specific nonce and AAD built from the MAC
//! header) keyed by the pairwise temporal key. The AES-CCM core is proven
//! against RFC 3610; the nonce/AAD construction is asserted byte for byte in
//! `iwlwifi_proofs`.

// LLC/SNAP header that precedes the ethertype in an 802.11 data payload.
pub(super) const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];

pub(super) const CCMP_HDR_LEN: usize = 8;

pub(super) const MIC_LEN: usize = 8;

pub(super) const ETH_HEADER_LEN: usize = 14;
