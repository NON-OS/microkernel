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

use core::sync::atomic::Ordering;

use super::fadt::parse_fadt;
use super::madt::parse_madt;
use super::other::{parse_dmar, parse_hpet, parse_ivrs, parse_mcfg, parse_srat};
use super::rsdp::find_rsdp;
use super::state::{TableRegistry, INITIALIZED, STATS, TABLES};
use super::{parse_rsdt, parse_xsdt};
use crate::arch::x86_64::acpi::aml::power_devices::classify_blocks;
use crate::arch::x86_64::acpi::aml::sleep_obj::find_in_blocks;
use crate::arch::x86_64::acpi::error::{AcpiError, AcpiResult};

pub fn init() -> AcpiResult<()> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Err(AcpiError::AlreadyInitialized);
    }

    let rsdp = find_rsdp()?;

    let mut registry = TableRegistry::new();

    registry.data.revision = rsdp.base.revision;
    registry.data.oem_id = rsdp.base.oem_id;

    // ACPI 2.0+ firmware must be read through the XSDT (64-bit pointers,
    // tables above 4 GiB). The RSDT is the fallback only when there is no
    // XSDT or it cannot be read, as in ACPICA's acpi_tb_parse_root_table.
    let mut root = Err(AcpiError::NoRootTable);
    if rsdp.has_xsdt() {
        root = parse_xsdt(&mut registry, rsdp.xsdt_address);
        if root.is_err() {
            registry = TableRegistry::new();
            registry.data.revision = rsdp.base.revision;
            registry.data.oem_id = rsdp.base.oem_id;
        }
    }
    if root.is_err() && rsdp.rsdt_address() != 0 {
        root = parse_rsdt(&mut registry, rsdp.rsdt_address() as u64);
    }
    if let Err(e) = root {
        INITIALIZED.store(false, Ordering::SeqCst);
        return Err(e);
    }

    if let Err(e) = parse_fadt(&mut registry) {
        INITIALIZED.store(false, Ordering::SeqCst);
        return Err(e);
    }
    parse_madt(&mut registry);
    parse_hpet(&mut registry);
    parse_mcfg(&mut registry);
    parse_srat(&mut registry);
    parse_dmar(&mut registry);
    parse_ivrs(&mut registry);

    {
        let mut stats = STATS.write();
        stats.tables_found = registry.tables.len() as u32;
        stats.processors_found = registry.data.processors.len() as u32;
        stats.ioapics_found = registry.data.ioapics.len() as u32;
        stats.overrides_found = registry.data.overrides.len() as u32;
        stats.pcie_segments = registry.data.pcie_segments.len() as u32;

        let mut nodes: alloc::collections::BTreeSet<u32> = alloc::collections::BTreeSet::new();
        for region in &registry.data.numa_regions {
            nodes.insert(region.proximity_domain);
        }
        stats.numa_nodes = nodes.len() as u32;
    }

    *TABLES.write() = Some(registry);

    // \_S5 is read once here, while the heap and the firmware tables are
    // certainly intact: shutdown runs after the zerostate wipe and must not
    // need to allocate or walk AML.
    let (s5, power_devices) = {
        let blocks = crate::arch::x86_64::acpi::aml::tables::aml_blocks();
        (
            find_in_blocks(blocks.iter().map(|b| b.as_slice()), 5),
            classify_blocks(blocks.iter().map(|b| b.as_slice())),
        )
    };
    if s5.is_none() {
        crate::log_warn!("[ACPI] no constant \\_S5 package found; soft-off unavailable");
    }
    if let Some(reg) = TABLES.write().as_mut() {
        reg.data.s5 = s5;
        reg.data.power_devices = power_devices;
    }

    Ok(())
}
