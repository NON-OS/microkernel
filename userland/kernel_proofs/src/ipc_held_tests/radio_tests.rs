// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Bluetooth is a radio: Standard carries its LE host, and Hardened and
//! Air-Gapped take it out, as Air-Gapped takes out every network driver
//! (tools/nix/config.nix, radioFeatures). Its spawn asks for the one host
//! it can use, xHCI.

fn read(rel: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// The feature of the catalogue entry whose handle is `name`.
fn feature_of(catalogue: &str, name: &str) -> String {
    let entry = catalogue.split("\n {").find(|e| e.contains(&format!("\"handle\": \"{name}\"")));
    let rest = entry.expect("in the catalogue").split("\"feature\": \"").nth(1).expect("a feature");
    rest.split('"').next().expect("closed").to_string()
}

/// The `drop = ...;` line of profile `name` in config.nix.
fn drops(config: &str, name: &str) -> String {
    let body = config.split(&format!("    {name} = {{")).nth(1).expect("the profile");
    let line = body.lines().find(|l| l.trim_start().starts_with("drop =")).expect("its drop");
    line.to_string()
}

fn radio(config: &str) -> String {
    let list = config.split("radioFeatures = [").nth(1).expect("the radio list");
    list.split("];").next().expect("closed").to_string()
}

#[test]
fn hardened_and_air_gapped_drop_bluetooth_and_standard_keeps_it() {
    let (config, cargo) = (read("tools/nix/config.nix"), read("Cargo.toml"));
    let feature = feature_of(&read("tools/nix/capsules.json"), "driver.bt_usb0");
    assert!(radio(&config).contains(&format!("\"{feature}\"")), "{feature} is no radio");
    for profile in ["hardened", "airgapped"] {
        assert!(drops(&config, profile).contains("radioFeatures"), "{profile} keeps Bluetooth");
    }
    assert!(!drops(&config, "standard").contains("radioFeatures"), "standard drops Bluetooth");
    let gui = cargo.split("microkernel-full-gui = [").nth(1).expect("the full set");
    assert!(gui.split(']').next().unwrap().contains(&format!("\"{feature}\"")));
}

#[test]
fn the_rule_sees_a_profile_that_keeps_it() {
    let config = read("tools/nix/config.nix").replace(" ++ radioFeatures", "");
    assert!(!drops(&config, "hardened").contains("radioFeatures"));
}

#[test]
fn the_bluetooth_host_spawns_where_xhci_is() {
    let plan = read("src/userspace/init/spawn_plan/drivers_bt.rs");
    assert!(plan.contains("present(HardwareFamily::UsbXhci)"), "it asks for another host");
    for other in ["UsbEhci", "UsbOhci", "UsbUhci"] {
        assert!(!plan.contains(other), "it waits on {other}, which it cannot use");
    }
    let usb = read("src/userspace/init/spawn_plan/drivers_usb.rs");
    assert!(usb.contains("\n    drivers_bt::spawn();\n}"), "it is never spawned");
}
