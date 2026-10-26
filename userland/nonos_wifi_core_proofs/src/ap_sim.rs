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

//! The authenticator's half of the four-way and group key handshakes, built
//! the way IEEE Std 802.11-2020, 12.7.6 and 12.7.7 lay the messages out, so
//! the supplicant is driven by frames an access point would send.

// `iwlwifi_proofs` and `rtl8821ce_proofs` include this file by `#[path]`, so
// its parts are named from this file's directory, which resolves the same way
// in their crates as in this one.
#[path = "ap_sim/elements.rs"]
mod elements;
#[path = "ap_sim/frames.rs"]
mod frames;
// Key data wrapping, by its own path: the driver proofs that include this
// file build frames with `frames` and never wrap key data themselves.
#[path = "ap_sim/wrap.rs"]
pub mod wrap;

pub use elements::*;
pub use frames::*;
