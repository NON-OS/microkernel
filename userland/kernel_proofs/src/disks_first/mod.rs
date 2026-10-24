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

//! The spawn order as a second guard among claimants that DMA (H-02, H-03):
//! the USB hosts before the Wi-Fi, so USB wins a CNVi card's group; the
//! SATA and NVMe controllers spawn before the bus drivers (smbus, spi,
//! i2c), the NICs and the rest, so a disk controller claims its isolation
//! group ahead of anything sharing it. HDA stays after the NICs: where HDA
//! and the I219 share a group, the network wins by default (the the developers'
//! ruling on H-02).

#[cfg(test)]
mod tests {
    const ORCHESTRATOR: &str =
        include_str!("../../../../src/userspace/init/spawn_plan/orchestrator.rs");
    const STORAGE: &str =
        include_str!("../../../../src/userspace/init/spawn_plan/drivers_storage.rs");
    const USB: &str = include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb.rs");

    fn at(text: &str, what: &str) -> usize {
        text.find(what).unwrap_or_else(|| panic!("{what} is gone"))
    }

    /// The body of `fn name(` to the closing brace at the line start.
    fn body(name: &str) -> &'static str {
        let from = &STORAGE[at(STORAGE, &format!("fn {name}() {{"))..];
        &from[..from.find("\n}").map_or(from.len(), |e| e + 2)]
    }

    #[test]
    fn the_disks_spawn_before_the_bus_drivers_and_the_nics() {
        let disks = at(ORCHESTRATOR, "super::drivers_storage::spawn_disks();");
        assert!(disks < at(ORCHESTRATOR, "super::drivers_bus::spawn();"), "after the bus");
        assert!(disks < at(ORCHESTRATOR, "super::drivers_nic::spawn();"), "after the NICs");
    }

    #[test]
    fn the_usb_hosts_spawn_before_the_wifi_so_usb_wins_a_cnvi_group() {
        let hosts = at(ORCHESTRATOR, "super::drivers_usb_hosts::spawn();");
        assert!(hosts < at(ORCHESTRATOR, "super::drivers_bus::spawn();"), "after the Wi-Fi");
        assert!(at(ORCHESTRATOR, "super::drivers_storage::spawn_disks();") < hosts, "disks first");
        assert!(!USB.contains("drivers_usb_hosts::spawn()"), "the hosts spawned twice");
    }

    #[test]
    fn hda_spawns_after_the_nics_so_the_network_wins_a_shared_group() {
        let nics = at(ORCHESTRATOR, "super::drivers_nic::spawn();");
        assert!(nics < at(ORCHESTRATOR, "super::drivers_storage::spawn();"), "HDA before the NICs");
    }

    #[test]
    fn the_disks_are_sata_and_nvme_and_spawn_once() {
        let disks = body("spawn_disks");
        assert!(disks.contains("spawn_ahci();") && disks.contains("spawn_nvme();"));
        let rest = body("spawn");
        assert!(!rest.contains("spawn_ahci();") && !rest.contains("spawn_nvme();"), "twice");
        assert!(rest.contains("spawn_hda();"), "HDA keeps its place");
    }
}
