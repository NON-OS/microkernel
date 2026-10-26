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

//! The kernel reads a handle's MSI entry only through `msi_of`, keyed by the
//! handle's id, and a departed function's entry is cleared with its handle.

fn src(rel: &str) -> String {
    let at = format!("{}/../../src/hardware/broker/{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(at).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

#[test]
fn the_kernel_reads_and_clears_the_entry_only_through_these() {
    let readers = [
        ("irq/bind/msi.rs", "handle.as_ref().and_then(pci_index::msi_of)"),
        ("irq/bind/quiet.rs", "pci_index::msi_of(handle)"),
        ("irq/bind/teardown.rs", "pci_index::msi_of(&handle)"),
        ("irq/aarch64/message/bind.rs", "msi_of(h).is_some()"),
        ("irq/aarch64/message/program.rs", "msi_of(handle)"),
        ("irq/aarch64/message/silence.rs", "msi_of(h)"),
    ];
    for (file, read) in readers {
        assert!(src(file).contains(read), "{file} no longer reads the side table");
    }
    assert!(!src("pci_index.rs").contains("msi: Option<MsiInfo>"), "the handle carries MSI again");
    let msi = src("pci_index/msi.rs");
    assert!(
        msi.contains("table::of(&MSI.read(), handle.device_id)")
            && msi.contains("*MSI.write() = entries;")
    );
    let late = src("pci_index/late.rs");
    let take = &late[late.find("fn take(").expect("take")..];
    assert!(take.contains("set::<MsiInfo>(&mut MSI.write(), device_id, None);"), "depart keeps it");
    assert!(late.contains("set(&mut MSI.write(), handle.device_id, msi);"), "a late add drops it");
}
