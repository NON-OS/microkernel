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

//! Every driver the kernel spawns is held or gates its own medium.

use super::held::callers_of;

/// Every driver the kernel spawns is either held here or gates its medium in
/// its own driver (the disks, by StoreWrite; the camera and microphone, by
/// their capture bits), so a new driver cannot arrive open to every
/// capsule; and every service a list names is one a spawn registers.
#[test]
fn every_driver_the_kernel_spawns_is_classified() {
    const GATED_IN_DRIVER: [&str; 5] =
        ["driver.nvme0", "driver.ahci0", "driver.virtio_blk0", "driver.sdhci0", "driver.ufs0"];
    // The USB camera and microphone, gated the same way.
    const GATED_IN_USB: [&str; 2] = ["driver.uvc0", "driver.usb_mic0"];
    // A virtual machine's disk controllers, gated the same way ("driver." each).
    const GATED_IN_VM: [&str; 5] = ["piix_ide0", "virtio_scsi0", "pvscsi0", "buslogic0", "mpt0"];
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let mut registered = Vec::new();
    let mut stack = vec![root.join("hardware"), root.join("userspace")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("readable").flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.file_name().is_some_and(|n| n == "spawn.rs") {
                let text = std::fs::read_to_string(&path).expect("readable");
                if let Some(rest) = text.split("const SERVICE_NAME: &str = \"").nth(1) {
                    registered.push(rest.split('"').next().expect("closed").to_string());
                }
                // A later window registers under its InstanceEndpoint's name.
                for rest in text.split("InstanceEndpoint {").skip(1) {
                    if let Some(name) = rest.split("name: \"").nth(1) {
                        registered.push(name.split('"').next().expect("closed").to_string());
                    }
                }
            }
        }
    }
    let drivers: Vec<&String> = registered.iter().filter(|n| n.starts_with("driver.")).collect();
    assert!(drivers.len() >= 18, "found only {drivers:?}");
    for d in &drivers {
        assert!(
            callers_of(d).is_some()
                || GATED_IN_DRIVER.contains(&d.as_str())
                || GATED_IN_USB.contains(&d.as_str())
                || d.strip_prefix("driver.").is_some_and(|n| GATED_IN_VM.contains(&n)),
            "{d} is spawned by the kernel and open to every capsule"
        );
    }
    for (ep, _) in drivers.iter().filter_map(|d| callers_of(d).map(|c| (d, c))) {
        for caller in callers_of(ep).unwrap() {
            assert!(
                registered.iter().any(|r| r == caller),
                "{ep} names {caller}, which no spawn registers"
            );
        }
    }
}
