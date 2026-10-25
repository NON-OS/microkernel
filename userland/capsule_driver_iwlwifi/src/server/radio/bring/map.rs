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

//! The control, receive and firmware regions, mapped as broker-sized grants.

use alloc::vec::Vec;

use crate::driver::Driver;
use crate::firmware::gen3::plan::RB_REGION;

use super::super::grant::Grant;
use super::super::say::Line;

// Map the control region, the receive buffers and each firmware region, in
// that order; on any refusal give back what was mapped and say so.
pub(super) fn map_all(d: &Driver, ctrl_len: usize, fw: &[usize]) -> Option<Vec<Grant>> {
    let mut out: Vec<Grant> = Vec::with_capacity(fw.len() + 2);
    for &len in [ctrl_len, RB_REGION].iter().chain(fw) {
        match Grant::map(d.device_id, d.claim_epoch, len) {
            Some(g) => out.push(g),
            None => {
                Line::new().text(b"no DMA region of ").num(len as u32).text(b" bytes").send();
                out.into_iter().for_each(Grant::unmap);
                return None;
            }
        }
    }
    Some(out)
}
