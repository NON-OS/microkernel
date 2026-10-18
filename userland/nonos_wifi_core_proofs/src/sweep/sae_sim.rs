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
//! The SAE sweeps' stations: hunting and pecking over one password, each
//! on its own draw of rand and mask.

use super::rng::Rng;
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::SaeStation;

pub fn station(rng: &mut Rng) -> SaeStation {
    let pwe = derive_pwe(b"pw", b"pw", &[2, 0, 0, 0, 0, 1], &[2, 0, 0, 0, 0, 2]).unwrap();
    let r: [u8; 32] = rng.bytes(32).try_into().unwrap();
    let m: [u8; 32] = rng.bytes(32).try_into().unwrap();
    SaeStation::start(pwe, false, &r, &m).expect("a usable draw")
}
