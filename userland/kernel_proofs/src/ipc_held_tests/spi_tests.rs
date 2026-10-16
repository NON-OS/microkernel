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
//! The Intel LPSS SPI host: spawned after the I2C host, asking for its
//! own function and MMIO but no IRQ, DMA or PIO, and held to the one
//! driver above it; its raw transfers reach whatever device is on the bus.

use super::common::owner;
use super::held::{callers_of, endpoint_admits};

const SPAWN: &str = include_str!("../../../../src/hardware/spi_capsule/spawn.rs");

#[test]
fn the_spi_host_starts_after_the_i2c_host_with_no_irq_dma_or_pio() {
    let bus = include_str!("../../../../src/userspace/init/spawn_plan/drivers_bus.rs");
    let at = |s: &str| bus.find(s).expect(s);
    assert!(at("spawn_i2c_pci();") < at("drivers_spi::spawn();"));
    for line in [
        "const SERVICE_NAME: &str = \"driver.spi0\";",
        "const SERVICE_PORT: u32 = 4528;",
        "const REPLY_PORT: u32 = 4529;",
        "const REPLY_INBOX: &str = \"endpoint.4294967485\";",
    ] {
        assert!(SPAWN.contains(line), "{line}");
    }
    for asked in ["DeviceEnum", "Driver", "Mmio"] {
        assert!(SPAWN.contains(&format!("Capability::{asked}")), "it needs {asked}");
    }
    for absent in ["Irq", "Dma", "Pio"] {
        assert!(!SPAWN.contains(&format!("Capability::{absent}")), "it asks for {absent}");
    }
}

#[test]
fn only_the_spi_hid_driver_reaches_the_spi_host() {
    assert_eq!(callers_of("driver.spi0"), Some(&["driver.spi_hid0"][..]));
    assert!(endpoint_admits("driver.spi0", owner(&["driver.spi_hid0"])));
    let others = owner(&["app.terminal", "app.browser", "driver.i2c_hid0", "compositor"]);
    assert!(!endpoint_admits("driver.spi0", others), "another capsule reaches it");
}
