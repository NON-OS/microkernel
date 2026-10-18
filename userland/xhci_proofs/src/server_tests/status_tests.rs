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

//! OP_CONTROLLER_STATUS tells a class driver how the controller is, never
//! where its DMA structures are: on a machine without an IOMMU those are
//! physical addresses a compromised class capsule could aim DMA at.

use super::bench::Bench;
use crate::protocol::{CONTROLLER_STATUS_PAYLOAD_LEN, OP_CONTROLLER_STATUS};

const HIGH_SPEED: u32 = 3;
const DCBAA_AT: usize = 36;
const SCRATCHPADS_AT: usize = 44;

#[test]
fn the_status_names_no_dma_address() {
    let mut b = Bench::new(HIGH_SPEED);
    let dcbaa = b.mux.ctxs[0].driver.dcbaa.phys();
    let r = b.call(OP_CONTROLLER_STATUS, &[]);
    assert_eq!(r.status, 0);
    assert_eq!(r.body.len(), CONTROLLER_STATUS_PAYLOAD_LEN);
    let field = |at: usize| u64::from_le_bytes(r.body[at..at + 8].try_into().expect("8 bytes"));
    assert_ne!(field(DCBAA_AT), dcbaa, "the DCBAA's bus address");
    assert_eq!(field(DCBAA_AT), 1, "a DCBAA is there");
    assert_eq!(field(SCRATCHPADS_AT), 0, "no scratchpad array on this controller");
    let bus = dcbaa.to_le_bytes();
    assert!(!r.body.windows(8).any(|w| w == bus), "nowhere in the reply");
}
