// NONOS Operating System (AGPL-3.0-or-later)
//! On Intel, config 90h to 97h logged as the firmware left them, beside the
//! CAP and PI line; on any other vendor, not read.

use nonos_libc::{config_reads, said, set_config};

use super::open::open;
use super::open_tests::AHCI;
use crate::discover::Found;
use crate::model::HbaRig;

fn port_map_line() -> Option<String> {
    said().into_iter().find(|l| l.contains("cfg 90h"))
}

#[test]
fn an_intel_controller_logs_its_port_map_and_control_dwords() {
    let _rig = HbaRig::left_armed();
    set_config(0x90, 0x0003_0000);
    set_config(0x94, 0x0001_0003);
    let opened = open(AHCI).expect("the controller opens");
    assert_eq!(port_map_line().as_deref(), Some("[ahci] cfg 90h 0x00030000 94h 0x00010003"));
    assert!(config_reads().contains(&(0x90, 4)) && config_reads().contains(&(0x94, 4)));
    drop(opened);
}

#[test]
fn a_read_the_broker_refuses_is_named_unread() {
    let _rig = HbaRig::left_armed();
    set_config(0x94, -1);
    let opened = open(AHCI).expect("the controller opens");
    assert!(port_map_line().is_some_and(|l| l.ends_with("94h unread")));
    drop(opened);
}

#[test]
fn another_vendors_controller_is_not_read_there() {
    let _rig = HbaRig::left_armed();
    let opened = open(Found { vendor: 0x1022, ..AHCI }).expect("the controller opens");
    assert_eq!(port_map_line(), None);
    assert!(!config_reads().iter().any(|&(off, _)| (0x90..0x98).contains(&off)));
    drop(opened);
}
