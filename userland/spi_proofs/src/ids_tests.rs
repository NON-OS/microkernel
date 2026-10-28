// NONOS Operating System (AGPL-3.0-or-later)
//! Which functions the SPI host takes: only the LPSS SPI ids (Linux v6.12
//! intel-lpss-pci.c), never another function of the LPSS classes, and the
//! kernel spawns it for exactly the ids it takes (inventory/spi_host.rs).

#[path = "../../../src/hardware/inventory/spi_host.rs"]
mod kernel;

use crate::ids::spi_host;

const DISCOVER: &str = include_str!("../../capsule_driver_spi/src/discover.rs");

#[test]
fn the_kernel_spawns_for_exactly_the_ids_the_driver_takes() {
    let mut taken = 0;
    for id in 0u16..=0xFFFF {
        let ours = spi_host(0x8086, id);
        assert_eq!(ours, kernel::spi_host(1, 0x8086, id), "8086:{id:04x}: the lists differ");
        taken += ours as u32;
    }
    assert_eq!(taken, 86);
    assert!(!kernel::spi_host(2, 0x8086, 0x9d29), "an ACPI record is no PCI host");
}

#[test]
fn the_kernel_spawns_neither_driver_without_a_listed_host() {
    let plan = include_str!("../../../src/userspace/init/spawn_plan/drivers_spi.rs");
    let inv = include_str!("../../../src/hardware/inventory/spi_present.rs");
    assert!(inv.contains("list().iter().any(|r| spi_host(r.bus_kind, r.vendor, r.device))"));
    let at = |s: &str| plan.find(s).unwrap_or_else(|| panic!("{s} not in drivers_spi.rs"));
    let gate = at("if !crate::hardware::inventory::spi_host_present() {");
    assert!(gate < at("return;") && at("return;") < at("c::spawn_driver_spi_capsule"));
}

#[test]
fn an_unlisted_lpss_function_is_never_a_candidate() {
    // Sunrise Point-LP: SPI 0 and 1 taken; its thermal sensor (class 11/80),
    // I2C 0 and UART 0 share the classes and are not.
    assert!(spi_host(0x8086, 0x9d29) && spi_host(0x8086, 0x9d2a));
    for other in [0x9d31, 0x9d60, 0x9d27, 0xa0e8, 0x0c80, 0x1180] {
        assert!(!spi_host(0x8086, other), "8086:{other:04x} taken for SPI");
    }
    assert!(!spi_host(0x1022, 0x9d29), "an id is Intel's only with Intel's vendor");
    let check = DISCOVER.split("pub fn is_candidate").nth(1).expect("is_candidate");
    let first = check.lines().nth(2).expect("its first test").trim();
    assert_eq!(first, "crate::ids::spi_host(r.vendor, r.device)", "discovery asks the list");
}
