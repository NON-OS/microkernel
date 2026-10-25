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

//! What a holder of a PCI record may map, read as text (L79). A map is
//! refused a slot past the count and a BAR that is not memory, the very
//! BARs the cover rule leaves unread, so find reads every BAR a holder could
//! map and no other.

use super::glue_text::{body, code, in_order};

#[test]
fn a_map_is_refused_the_bars_the_rule_leaves_unread() {
    let src = code("hardware/broker/mmio/map_bar.rs");
    let head = "fnclaimed_bar(pid:u32,req:&MmioMapRequest)->Result<Claimed,MmioMapError>{";
    in_order(
        body(&src, head),
        &[
            "letdevice=table::list().into_iter().find(|r|r.device_id==req.device_id)",
            "letbar=mappable(&device.bars,device.bar_count,req.bar_index).map_err(|r|matchr{",
            "BarRefusal::BadIndex=>MmioMapError::BadBarIndex,",
            "BarRefusal::NotMmio=>MmioMapError::NotMmioBar,",
            "Ok(Claimed{",
        ],
    );
    // The slot rule the map asks, proved over every slot in hardware::broker.
    let sel = code("hardware/broker/mmio/bar_select.rs");
    in_order(
        body(&sel, "pubfnmappable(bars:&[Bar],bar_count:u8,index:u8)->Result<Bar,BarRefusal>{"),
        &[
            "ifi>=bars.len()||i>=bar_countasusize{returnErr(BarRefusal::BadIndex);}",
            "letbar=bars[i];ifbar.kind!=BAR_KIND_MMIO||bar.size==0{",
            "returnErr(BarRefusal::NotMmio);}Ok(bar)",
        ],
    );
}
