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

//! A PPTT of thousands of nodes in one parent chain: walked once.

use super::build::*;
use super::pptt_build::node;
use crate::arm_acpi::pptt::pptt;

#[test]
fn a_long_chain_is_walked_once() {
    let n = 4000u32;
    let parts: Vec<Vec<u8>> =
        (0..n).map(|i| node(0, if i == 0 { 0 } else { 36 + (i - 1) * 20 }, i, &[])).collect();
    let t = sdt(b"PPTT", 3, &parts.concat());
    let start = std::time::Instant::now();
    assert_eq!(pptt(&t).map(|p| p.nodes.len()), Ok(n as usize));
    assert!(start.elapsed().as_secs() < 2, "linear, not cubic");
}
