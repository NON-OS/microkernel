// NONOS Operating System (AGPL-3.0-or-later)
//! A device whose answers come short or change between reads: only the
//! bytes it sent are taken, never what was asked for, and a configuration
//! is never read past the length its own head gave.

use std::cell::Cell;
use std::rc::Rc;

use crate::found::fetch;
use crate::mock::{Call, MockBus};
use crate::qemu_usb_net::{CONFIG_ECM, DEVICE};

/// The device descriptor, then each configuration read answered by `config`
/// with the read's number (0 the head, 1 the whole).
fn device(config: impl Fn(usize) -> Vec<u8> + 'static) -> (MockBus, Rc<Cell<usize>>) {
    let reads = Rc::new(Cell::new(0));
    let seen = reads.clone();
    let bus = MockBus::new(Box::new(move |call| match call {
        Call::In(s, _) if s.value >> 8 == 1 => Ok(DEVICE.to_vec()),
        Call::In(s, _) if s.value >> 8 == 2 => {
            seen.set(seen.get() + 1);
            Ok(config(seen.get() - 1))
        }
        _ => Err(-32),
    }));
    (bus, reads)
}

#[test]
fn a_device_descriptor_cut_short_is_none_though_it_names_its_length() {
    let mut bus = MockBus::new(Box::new(|_| Ok(DEVICE[..8].to_vec())));
    assert_eq!(fetch(&mut bus, 1).err(), Some(-5));
}

#[test]
fn a_configuration_shorter_than_its_head_is_kept_as_sent() {
    let (mut bus, _) = device(|_| CONFIG_ECM[..12].to_vec());
    let found = fetch(&mut bus, 1).unwrap();
    assert_eq!(found.configs[0], CONFIG_ECM[..12].to_vec(), "no padding");
}

#[test]
fn a_configuration_never_grows_past_its_heads_length() {
    let (mut bus, _) = device(|n| {
        let mut c = CONFIG_ECM.to_vec();
        if n % 2 == 0 {
            c[2..4].copy_from_slice(&9u16.to_le_bytes());
        }
        c
    });
    let found = fetch(&mut bus, 1).unwrap();
    assert_eq!(found.configs[0].len(), 9, "the head said 9; the rest is not read");
}

#[test]
fn a_device_naming_255_configurations_is_read_six_times() {
    let mut dev = DEVICE.to_vec();
    dev[17] = 255;
    let reads = Rc::new(Cell::new(0));
    let seen = reads.clone();
    let mut bus = MockBus::new(Box::new(move |call| match call {
        Call::In(s, _) if s.value >> 8 == 1 => Ok(dev.clone()),
        _ => {
            seen.set(seen.get() + 1);
            Ok(CONFIG_ECM.to_vec())
        }
    }));
    assert_eq!(fetch(&mut bus, 1).unwrap().configs.len(), 6);
    assert_eq!(reads.get(), 12, "a head and a whole each");
}
