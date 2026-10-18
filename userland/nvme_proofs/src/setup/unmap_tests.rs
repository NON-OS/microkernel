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

//! Dropping a DMA region unmaps nothing: memory the controller may have
//! seen goes back only with the device's release. Only memory the
//! controller never took is unmapped, and only when asked.

use nonos_libc::calls;

use crate::dma::DmaRegion;

#[test]
fn a_dma_region_is_unmapped_only_when_asked() {
    // Mapped, and gone at the end of the statement: nothing is unmapped.
    assert!(DmaRegion::map(7, 1, 4096).is_ok(), "host memory");
    assert!(calls().iter().all(|c| c.0 != "dma_unmap"), "a drop unmapped");
    let Ok(given_back) = DmaRegion::map(7, 1, 4096) else { panic!("host memory") };
    given_back.unmap();
    assert_eq!(calls().last().map(|c| c.0), Some("dma_unmap"));
}
