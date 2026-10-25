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

//! find's glue in bootfb/cover.rs, read as text (L79): each record asked
//! with its own BARs, through the rule the proofs mount, its memory BARs
//! only and only as many as it counts; a display the PCI bus's display
//! class; the any and the count of displays taken over the same snapshot
//! of the table as each record's own answer.

use super::glue_text::{body, code, in_order};

pub(super) const COVER: &str = "hardware/broker/bootfb/cover.rs";
const FIND: &str = "pub(super)fnfind(page:u64,len:u64){";

#[test]
fn find_asks_each_record_with_its_own_memory_bars() {
    let src = code(COVER);
    assert!(src.contains("usesuper::cover_rule::{coversasrule,holds,Span};"), "another rule");
    in_order(
        body(&src, FIND),
        &[
            "letall=table::list();",
            "letmine=|r:&DeviceRecord|{",
            "letspans:[Span;6]=r.bars.map(|b|(b.base,b.size,b.kind));",
            "holds((page,len),&spans,usize::from(r.bar_count),BarKind::Mmioasu8)",
        ],
    );
}

#[test]
fn the_fallback_counts_pci_displays_over_the_same_records() {
    let src = code(COVER);
    let display = body(&src, "fndisplay(r:&DeviceRecord)->bool{");
    assert_eq!(display, "r.bus_kind==BusKind::Pciasu8&&r.class==ids::DISPLAY");
    assert!(code("hardware/broker/class.rs").contains("(0x03,_)=>ids::DISPLAY,"));
    let find = body(&src, FIND);
    assert_eq!(find.matches("table::list()").count(), 1, "two snapshots of the table");
    in_order(
        find,
        &[
            "letall=table::list();",
            "letany=all.iter().any(mine);",
            "letsingle=all.iter().filter(|r|display(r)).count()==1;",
            "letby=|r:&&DeviceRecord|rule(mine(r),any,display(r),single);",
            "letids:Vec<u64>=all.iter().filter(by).map(|r|r.device_id).collect();",
            "COVER.call_once(||ids);",
        ],
    );
}
