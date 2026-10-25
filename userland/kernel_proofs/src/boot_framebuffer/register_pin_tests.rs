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

//! register_boot_framebuffer pinned whole (L79), with its log lines' words
//! left out: find first, then every way out before publication returning
//! with nothing published, the sink made before the record exists, and the
//! id the scanout's takeover asks for the one that was published. Beside it,
//! the control that shows why find must run before the record exists.

use super::cover_find::{find, Rec, MIB, MMIO};
use super::glue_text::{bare, body};
use super::rule::window;

const REGISTER: &str = "hardware/broker/bootfb/register.rs";

const BODY: &str = concat!(
    "letSome(w)=phys_window()else{return;};",
    "super::cover::find(w.page,w.len);",
    "letSome((base,size))=window(w.page,w.len,w.offset)else{serial::println(b\"\");return;};",
    "if!super::sink::make(){serial::println(b\"\");return;}",
    "letmutbars=[Bar::empty();6];",
    "letbar=BarKind::Mmioasu8;",
    "bars[0]=Bar{base,size,aux:w.offset,kind:bar,flags:BAR_FLAG_PREFETCH,_pad:[0;2]};",
    "letid=register_platform_device(DeviceRecord{",
    "bus_kind:BusKind::Firmwareasu8,class:ids::DISPLAY,device:FIRMWARE_BOOT_FRAMEBUFFER,",
    "bar_count:1,bars,..DeviceRecord::empty()",
    "});",
    "BOOTFB.call_once(||id);",
    "letline=format!(\"\",base,size,id);",
    "serial::println(line.as_bytes());",
);

#[test]
fn registration_is_exactly_find_then_the_record_or_nothing() {
    let src = bare(REGISTER);
    assert_eq!(body(&src, "pubfnregister_boot_framebuffer(){"), BODY, "registration changed");
    assert_eq!(src.matches("BOOTFB.call_once(").count(), 1, "the id set twice");
    assert_eq!(src.matches("BOOTFB").count(), 4, "the id named elsewhere");
}

#[test]
fn the_takeover_asks_after_the_published_record() {
    let src = bare(REGISTER);
    let head = "pub(incrate::hardware::broker)fndevice()->Option<u64>{";
    assert_eq!(body(&src, head), "BOOTFB.get().copied()", "the takeover told of no record");
}

#[test]
fn were_the_record_published_first_find_would_name_it_and_lose_the_display() {
    // Illustrative: a frame in RAM and the one PCI display elsewhere. The
    // published record's one BAR is the very window find is asked about.
    let (page, len) = (0x7E00_0000, 0x30_0040);
    let (base, size) = window(page, len, 0x40).expect("a window a loader may hand over");
    let display = Rec::new(&[(0xFC00_0000, 16 * MIB, MMIO)], true);
    assert_eq!(find((page, len), &[display]), vec![0], "before: the lone display");
    let display = Rec::new(&[(0xFC00_0000, 16 * MIB, MMIO)], true);
    let published = Rec::new(&[(base, size, MMIO)], false);
    assert_eq!(find((page, len), &[display, published]), vec![1], "after: only itself");
}
