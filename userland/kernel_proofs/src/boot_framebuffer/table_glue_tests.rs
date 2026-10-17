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

//! The device table after find, read as text (L79). Only the PCI scan
//! replaces it whole; a platform record only joins it at the end; nothing
//! else in table/, where the table's static lives, writes it. So a record
//! find has read never changes, and what find stored stays true for the
//! rest of the boot with no lock taken to read it.

use super::glue_text::{body, code};

const DIR: &str = "hardware/broker/table";

const JOIN: &str = concat!(
    "letmuttable=TABLE.write();",
    "letnext=table.iter().map(|r|r.device_id).max().map(|m|m+1);",
    "record.device_id=next.unwrap_or(0x1_0000_0000);",
    "table.push(record);",
    "record.device_id",
);

#[test]
fn the_table_is_replaced_by_the_scan_alone_and_otherwise_only_grows() {
    let mut writers = Vec::new();
    let path = format!("{}/../../src/{DIR}", env!("CARGO_MANIFEST_DIR"));
    for entry in std::fs::read_dir(path).expect("table/ readable") {
        let name = entry.expect("an entry").file_name().into_string().expect("a name");
        let n = code(&format!("{DIR}/{name}")).matches(".write()").count();
        writers.extend(std::iter::repeat_n(name, n));
    }
    assert_eq!(writers, ["init.rs", "init.rs"], "the table written elsewhere");
    let src = code(&format!("{DIR}/init.rs"));
    let init = body(&src, "pubfninit_from_pci(devices:&[PciDevice]){");
    assert!(init.ends_with("*TABLE.write()=records;pci_index::install(handles);"));
    let head = "fnregister_platform_device(mutrecord:DeviceRecord)->u64{";
    assert_eq!(body(&src, head), JOIN, "a platform record does more than join the table");
}
