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

//! What find reads of a PCI record, read as text (L79). Each present BAR
//! sits in its own slot, a memory BAR as memory at its base and size, and
//! the count reaches the last present slot; the bus is PCI and the class is
//! what the class code maps to. What a holder may map is `map_glue_tests`.

use super::glue_text::{body, code, in_order};

const RECORD: &str = "hardware/broker/table/pci_record.rs";
const HEAD: &str = "pub(super)fnrecord_from_pci(device_id:u64,dev:&PciDevice)->DeviceRecord{";

#[test]
fn a_pci_record_holds_every_present_bar_its_count_and_its_class() {
    let src = code(RECORD);
    let rec = body(&src, HEAD);
    let fill = concat!(
        "letmutbars=[Bar::empty();6];letmutbar_span=0u8;",
        "for(i,b)indev.bars.iter().enumerate(){",
        "ifb.is_present(){bars[i]=bar_from_pci(b);bar_span=(i+1)asu8;}",
        "}letmutrecord=DeviceRecord{device_id,bus_kind:BusKind::Pciasu8,",
    );
    assert!(rec.starts_with(fill), "a present BAR out of its slot, or past the count");
    in_order(
        rec,
        &[
            ",class:classify_pci(dev.class,dev.subclass,dev.progif).id(),",
            ",bar_count:bar_span,",
            ",bars,};legacy::attach(&mutrecord);record",
        ],
    );
    let from = body(&src, "fnbar_from_pci(b:&PciBar)->Bar{");
    assert!(from.starts_with("ifb.is_memory(){mem_bar(b)}"), "memory read as another kind");
    let mem = body(&src, "fnmem_bar(b:&PciBar)->Bar{");
    let tail = concat!(
        "letbase=b.address().map(|p|p.as_u64()).unwrap_or(0);",
        "Bar{base,size:b.size(),kind:BarKind::Mmioasu8,flags,aux:0,_pad:[0;2]}",
    );
    assert!(mem.ends_with(tail), "a memory BAR's base, size or kind changed");
}
