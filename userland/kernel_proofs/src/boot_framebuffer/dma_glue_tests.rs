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

//! No DMA against the boot framebuffer's record (L79), read from the DMA
//! map's validation: a takeover reclaims the record's MMIO from outside its
//! holder, and a DMA grant could not be reclaimed that way. The refusal asks
//! no length and stands before the only way to an Ok, so every length that
//! got that far is refused; and every DMA map is validated before anything
//! is allocated for it.

use super::glue_text::{body, code, in_order};

const HEAD: &str =
    "pub(super)fnvalidate(req:&DmaMapRequest,pid:u32)->Result<claim::InUse,DmaMapError>{";
const MAP: &str =
    "pubfnmap_for_caller(pid:u32,req:DmaMapRequest)->Result<DmaMapResult,DmaMapError>{";

#[test]
fn the_boot_framebuffer_record_is_refused_dma_at_any_length() {
    let src = code("hardware/broker/dma/map/validate.rs");
    let v = body(&src, HEAD);
    let tail = concat!(
        "letfb=crate::hardware::broker::bootfb::is_boot_framebuffer(req.device_id);",
        "iffb||pages>dma_page_limit_for_class(class){",
        "returnErr(DmaMapError::BadLengthForClass);",
        "}Ok(claim)",
    );
    assert!(v.ends_with(tail), "the refusal moved, narrowed or dropped");
    assert_eq!(v.matches("Ok(").count(), 1, "another way to an Ok");
    let tx = code("hardware/broker/dma/map/transaction.rs");
    let map = body(&tx, MAP);
    in_order(
        map,
        &[
            "letheld=matchvalidate::validate(&req,pid){Ok(held)=>held,",
            "Err(e)=>returnfail(\"validate\",e),",
            "alloc::alloc_and_zero(",
        ],
    );
}
