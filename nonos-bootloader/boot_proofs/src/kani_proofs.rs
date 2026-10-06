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

//! Kani harnesses: the rollback counter's read and the image footer, for every
//! input of the checked size, not just the sampled ones in the runnable tests.

use crate::image_format::parse::parse_image_footer;
use crate::security::tpm_nv::floor_cmd::{floor_read, succeeded, FloorRead};

// Any answer and any claimed length: the read never panics or reads past the
// buffer, and a value comes only from a success in the read's own shape.
#[kani::proof]
fn floor_read_is_total_and_a_value_only_on_success() {
    let resp: [u8; 32] = kani::any();
    let n: usize = kani::any();
    if let FloorRead::Value(v) = floor_read(&resp, n) {
        assert!(n >= 24 && n <= 32);
        assert!(resp[6..10] == [0, 0, 0, 0]);
        assert!(resp[10..16] == [0, 0, 0, 10, 0, 8]);
        assert!(v.to_be_bytes() == resp[16..24]);
    }
    let _ = succeeded(&resp, n);
}

// Uninitialized is that one response code and nothing else.
#[kani::proof]
fn only_nv_uninitialized_reads_as_uninitialized() {
    let resp: [u8; 16] = kani::any();
    let n: usize = kani::any();
    if floor_read(&resp, n) == FloorRead::Uninitialized {
        assert!(n >= 10 && n <= 16);
        assert!(resp[6..10] == [0, 0, 0x01, 0x4A]);
    }
}

// Parsing any attacker-controlled image footer is free of panics, out-of-bounds
// access and arithmetic overflow. Kani verifies these for every byte pattern of
// this size; the region-extraction slices cannot escape the buffer.
#[kani::proof]
#[kani::unwind(4)]
fn parse_footer_is_total_and_in_bounds() {
    let buf: [u8; 72] = kani::any();
    let _ = parse_image_footer(&buf);
}
