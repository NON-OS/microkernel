// NONOS Operating System (AGPL-3.0-or-later)
//! W4: Bus Master goes on only once a device whose DMA a predecessor left
//! armed read back its bus master stopped and was reset; a device that never
//! stops never gets it, and a refused command write is named.

use nonos_libc::{given_back, refuse_config, EPOCH};

use super::gen3_armed::{armed, Armed};
use crate::command::command;
use crate::gen3::outcome::master::MasterError;
use crate::gen3::outcome::{master_on, Failure, STAGE_NO_DMA, STEP_MASTER};
use crate::gen3_model::Polls;

fn run(dev: &Armed) -> (Result<(), Failure>, Vec<(&'static str, u64)>) {
    let before = given_back().len();
    let got = master_on(dev, &mut Polls { per_wait: 64, delays_us: 0 }, || {
        assert!(!dev.armed.get(), "the engines are reset before Bus Master");
        command(5, EPOCH, true)
    });
    (got, given_back()[before..].to_vec())
}

#[test]
fn bus_master_comes_on_only_after_the_armed_device_reads_back_stopped() {
    let (got, log) = run(&armed(Some(3)));
    assert_eq!(got, Ok(()));
    let order = [("stop_master", 0), ("master_stopped", 0), ("sw_reset", 0), ("pci_command", 6)];
    assert_eq!(log, order);
}

#[test]
fn a_device_that_never_stops_never_gets_bus_master() {
    let (got, log) = run(&armed(None));
    let f = got.unwrap_err();
    assert_eq!(f, Failure::Master(MasterError::NotStopped));
    assert_eq!(log, [("stop_master", 0), ("sw_reset", 0)], "reset, but no command write");
    assert_eq!((f.stage(), f.step(), f.detail()), (STAGE_NO_DMA, STEP_MASTER, 1));
    assert_eq!(f.text(), "the device's bus master never stopped");
}

#[test]
fn a_refused_bus_master_write_is_named() {
    refuse_config(true);
    let (got, log) = run(&armed(Some(1)));
    refuse_config(false);
    let f = got.unwrap_err();
    assert_eq!((f, f.detail()), (Failure::Master(MasterError::Refused), 2));
    assert_eq!(log.last(), Some(&("pci_command", 6)), "asked once, after the stop");
}
