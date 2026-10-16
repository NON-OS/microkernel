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

//! WPA2-PSK (AKM 00-0F-AC:2, CCMP, key descriptor version 2) against known
//! answers that do not come from this crate. PBKDF2: IEEE Std 802.11-2020
//! J.4.2, as hostap's crypto module tests carry it. PRF-SHA1: the J.3.2
//! "prefix" vectors from the same tests. The four-way handshake: messages 1
//! and 3 as an authenticator frames them, and the messages 2 and 4 expected
//! back, byte for byte, built by an independent reference (Python hashlib and
//! the `cryptography` RFC 3394 key wrap, following hostap's
//! tests/hwsim/test_ap_psk.py `pmk_to_ptk`, `build_eapol` and
//! `eapol_key_mic`). The access point is a WPA2/WPA3 transition network (AKM
//! 2 and 8, MFPC=1, MFPR=0, an RSNXE) joined with PSK, and a PSK network that
//! requires management frame protection, whose message 3 adds the IGTK KDE.
//! Through the MLME, the association request's RSNE is the one message 2
//! repeats, and a message 3 repeated after a lost message 4 is answered.

mod igtk;
mod kdf;
mod mlme;
mod protected;
mod transition;
mod vectors;
