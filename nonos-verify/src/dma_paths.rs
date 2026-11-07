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

// Device DMA goes through the hardware broker alone (MkDmaMap, confined by
// the capsule's IOMMU domain when a unit is in service). The tree used to
// hold three other ways to make a device a bus master: the legacy DmaEngine,
// bus::pci::bind_driver and ConfigSpace::enable_bus_master. None had a
// caller, and they were removed. This fails if one of them comes back, or if
// code outside the broker sets the bus master bit, so a DMA path that skips
// the broker is a reviewed change and not a quiet one. a work area, 4.5.

use std::fs;
use std::path::{Path, PathBuf};

/// Names of the removed paths: not to be defined or called again.
const REMOVED: &[&str] =
    &["DmaEngine", "bind_driver(", "fn enable_bus_master(", ".enable_bus_master(", "enable_bus_mastering("];

/// Ways to OR the bus master bit into a command word.
const SETS_MASTER: &[&str] = &["| CMD_BUS_MASTER", "| command::BUS_MASTER", "|= command::BUS_MASTER", "| BUS_MASTER"];

/// Where setting it is the broker's own business: the claim path turns it on
/// for a confined device, and the command rule decides what a driver may set.
const BROKER: &str = "src/hardware/broker/";

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

#[test]
fn nothing_masters_a_device_outside_the_broker() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    assert!(files.len() > 1000, "the kernel source was not found");
    let mut found = Vec::new();
    for f in &files {
        let rel = f.strip_prefix(&root).unwrap_or(f).to_string_lossy().replace('\\', "/");
        let text = fs::read_to_string(f).unwrap_or_default();
        for (n, line) in text.lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") || code.starts_with('*') || code.starts_with("/*") {
                continue;
            }
            let removed = REMOVED.iter().any(|r| code.contains(r));
            let sets = !rel.starts_with(BROKER) && SETS_MASTER.iter().any(|p| code.contains(p));
            if removed || sets {
                found.push(format!("{rel}:{}: {}", n + 1, code));
            }
        }
    }
    assert!(found.is_empty(), "bus mastering outside the broker:\n{}", found.join("\n"));
}
