// NONOS Operating System (AGPL-3.0-or-later)
//! HID over SPI: spawned after the SPI host it reaches its device through,
//! asking for IPC, memory and input posting and no hardware at all, and
//! reached by no capsule.

use super::common::owner;
use super::held::{callers_of, endpoint_admits};

const SPAWN: &str = include_str!("../../../../src/hardware/spi_hid_capsule/spawn.rs");

#[test]
fn the_spi_hid_driver_starts_after_its_host_and_holds_no_hardware() {
    let plan = include_str!("../../../../src/userspace/init/spawn_plan/drivers_spi.rs");
    let at = |s: &str| plan.find(s).expect(s);
    assert!(at("c::spawn_driver_spi_capsule") < at("spawn_spi_hid();"));
    for line in [
        "const SERVICE_NAME: &str = \"driver.spi_hid0\";",
        "const SERVICE_PORT: u32 = 4534;",
        "const REPLY_PORT: u32 = 4535;",
        "const REPLY_INBOX: &str = \"endpoint.4294967490\";",
    ] {
        assert!(SPAWN.contains(line), "{line}");
    }
    for asked in ["IPC", "Memory", "InputSource"] {
        assert!(SPAWN.contains(&format!("Capability::{asked}")), "it needs {asked}");
    }
    for absent in ["DeviceEnum", "Driver", "Mmio", "Irq", "Dma", "Pio"] {
        assert!(!SPAWN.contains(&format!("Capability::{absent}")), "it asks for {absent}");
    }
}

#[test]
fn nothing_reaches_the_spi_hid_driver() {
    assert_eq!(callers_of("driver.spi_hid0"), Some(&[][..]));
    let others = owner(&["driver.spi0", "app.terminal", "compositor", "input_router"]);
    assert!(!endpoint_admits("driver.spi_hid0", others), "a capsule reaches it");
}
