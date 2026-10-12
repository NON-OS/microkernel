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

//! WPA3-SAE against published known answers, then the exchange end to end.
//!
//! Vector source: IEEE Std 802.11-2020, Annex J.10 (SAE test vectors), as
//! transcribed in hostapd's `src/common/common_module_tests.c`, function
//! `sae_tests()`, fetched from https://w1.fi/cgit/hostap/plain/src/common/
//! common_module_tests.c (sha256 a75c0bc7e9069a2c443cfd225e52c912493c0255448
//! 606948483d3e9e3bcd9e3). The hunting-and-pecking part derives the PWE for
//! addresses 4d:3f:2f:ff:e3:87 / a5:d8:aa:95:8e:3c and password
//! "mekmitasdigoat", builds the commit from the given rand and mask, and from
//! the peer's commit derives the KCK, PMK and PMKID. The hash-to-element part
//! derives PT for SSID "byteme", that password and identifier "psk4internet",
//! and from it the PWE for 00:09:5b:66:ec:1e / 00:0b:6b:d9:02:46.

mod annex;
mod discard;
mod exchange;
mod frames;
mod hostile;
mod known;
mod malformed;
