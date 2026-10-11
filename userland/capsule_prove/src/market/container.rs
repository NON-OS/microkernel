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

//! The container a market proof travels in: NPRF v1, kind 1, with the plan's
//! six sections in tag order, the window the day of proving.

use alloc::vec::Vec;

use isolation_stark::Statement;
use nonos_proof_abi::{encode, tag, Kind};

use super::sealed::Sealed;

pub fn container(
    day: u64,
    s: &Sealed,
    device: &[u8],
    isolation: &[u8],
    st: &Statement,
    policy: &[u8; 256],
) -> Option<Vec<u8>> {
    let words: Vec<u8> = st.words().iter().flat_map(|w| w.to_le_bytes()).collect();
    let list = s.list.concat();
    let sections: [(u16, &[u8]); 6] = [
        (tag::RECORD, &s.raw),
        (tag::DEVICE, device),
        (tag::ISOLATION, isolation),
        (tag::PUBLICS, &words),
        (tag::MEASUREMENTS, &list),
        (tag::POLICY, policy),
    ];
    encode(Kind::Isolation, day, &sections)
}
