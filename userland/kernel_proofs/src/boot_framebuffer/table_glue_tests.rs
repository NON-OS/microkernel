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
//! replaces it whole; a platform record, or a function found after boot,
//! only joins it at the end above every id a departed record had; a
//! departed record only leaves it, its id never reused; the NUMA facts and
//! an external-facing port change only `numa_node` and `flags`. So no BAR,
//! class or bus kind find has read ever changes, and what find stored stays
//! true for the rest of the boot with no lock taken to read it.

use super::glue_text::{body, code};

const DIR: &str = "hardware/broker/table";

const JOIN: &str = concat!(
    "letmuttable=TABLE.write();",
    "letmax=table.iter().map(|r|r.device_id).max();",
    "record.device_id=next_id(max,0x1_0000_0000);",
    "table.push(record);",
    "record.device_id",
);

const WRITERS: [&str; 6] = ["add.rs", "external.rs", "init.rs", "init.rs", "numa.rs", "remove.rs"];
const NUMA: &str = concat!(
    "letmuttable=TABLE.write();forrintable.iter_mut(){",
    "ifletSome((_,n))=nodes.iter().find(|(id,_)|*id==r.device_id){r.numa_node=*n;}}",
);
const EXTERNAL: &str = "{r.flags|=DEVICE_FLAG_EXTERNAL;}ids.len()";

fn file(name: &str) -> String {
    code(&format!("{DIR}/{name}"))
}

#[test]
fn the_table_is_replaced_by_the_scan_alone_and_otherwise_only_grows() {
    let mut writers = Vec::new();
    let path = format!("{}/../../src/{DIR}", env!("CARGO_MANIFEST_DIR"));
    for entry in std::fs::read_dir(path).expect("table/ readable") {
        let name = entry.expect("an entry").file_name().into_string().expect("a name");
        writers.extend(std::iter::repeat_n(name.clone(), file(&name).matches(".write()").count()));
    }
    writers.sort();
    assert_eq!(writers, WRITERS, "the table written elsewhere");
    let src = file("init.rs");
    let init = body(&src, "pubfninit_from_pci(devices:&[PciDevice]){");
    assert!(init.ends_with("*TABLE.write()=records;pci_index::install(handles);"));
    let head = "fnregister_platform_device(mutrecord:DeviceRecord)->u64{";
    assert_eq!(body(&src, head), JOIN, "a platform record does more than join the table");
    let add = file("add.rs");
    let add = body(&add, "fnadd_pci_function(dev:&PciDevice,disabled:bool)->u64{");
    assert!(add.contains("letid=next_id(table.iter().map(|r|r.device_id).max(),0);"));
    assert!(add.contains("table.push(record);") && !add.contains("iter_mut"), "a late one edits");
    let next = "max.map_or(empty,|m|m+1).max(RETIRED.load(Ordering::Acquire))";
    assert!(file("state.rs").contains(next), "an id below a departed one handed out");
    assert!(file("remove.rs").contains("retire(device_id);Some(table.remove(at))"));
    let numa = file("numa.rs");
    assert_eq!(body(&numa, "fnset_numa_nodes(nodes:&[(u64,u8)]){"), NUMA, "NUMA writes more");
    assert!(file("external.rs").contains(EXTERNAL), "a port's mark writes more than its flag");
}
