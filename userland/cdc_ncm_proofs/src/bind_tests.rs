// NONOS Operating System (AGPL-3.0-or-later)
//! Binding an NCM device in the order Linux cdc_ncm_bind_common,
//! cdc_ncm_init and cdc_ncm_setup take.

use nonos_usbnet::mock::Call;
use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS};
use nonos_usbnet::{Bind, Nic, Setup};

use crate::device::{calls, Dev};
use crate::ncm::bind;
use crate::spec::{config_ncm, config_vendor, MAC};

#[test]
fn a_plain_ncm_device_binds_with_every_request_in_linux_order() {
    let dev = Dev::ncm(0);
    let (found, bus) = (dev.found(), dev.bus());
    let Bind::Ours(nic) = bind(bus.clone(), &found) else { panic!("not bound") };
    assert_eq!(nic.mac(), MAC);
    assert!(nic.link_up());
    assert_eq!(nic.rx_max, 4096, "16 KiB offered, one bulk transfer asked");
    let c = calls(&bus);
    assert!(matches!(c[0], Call::Configure(p) if p.bulk_in == 0x82 && p.bulk_out == 0x03));
    assert_eq!(c[1], Call::Out(Setup::set_configuration(1), vec![]));
    assert_eq!(c[2], Call::Out(Setup::set_interface(1, 0), vec![]));
    assert_eq!(c[3], Call::In(Setup::new(0xA1, 0x80, 0, 0), 28), "GET_NTB_PARAMETERS");
    assert_eq!(c[4], Call::Out(Setup::set_interface(1, 1), vec![]));
    assert!(matches!(c[5], Call::In(s, _) if s.value == 0x0304), "the MAC string");
    let input_size = Call::Out(Setup::new(0x21, 0x86, 0, 0), 4096u32.to_le_bytes().to_vec());
    assert_eq!(c[6], input_size, "SET_NTB_INPUT_SIZE, 4-byte form");
    assert_eq!(c[7], Call::Out(Setup::new(0x21, 0x43, 0x0E, 0), vec![]), "packet filter");
    assert_eq!(c.len(), 8, "no CRC, format or datagram size request without the bits");
    assert!(nonos_libc::slept_ms() >= 10, "cdc_ncm_bind_common's pause before the data alternate");
}

#[test]
fn the_ncm_configuration_is_found_after_others() {
    let mut dev = Dev::ncm(0);
    dev.configs = vec![config_vendor(), config_ncm(3, 0)];
    let (found, bus) = (dev.found(), dev.bus());
    assert!(matches!(bind(bus.clone(), &found), Bind::Ours(_)));
    assert_eq!(calls(&bus)[1], Call::Out(Setup::set_configuration(3), vec![]));
}

#[test]
fn an_ecm_only_device_is_left_to_the_ecm_driver() {
    let mut dev = Dev::ncm(0);
    dev.configs = vec![CONFIG_RNDIS.to_vec(), CONFIG_ECM.to_vec()];
    let (found, bus) = (dev.found(), dev.bus());
    assert!(matches!(bind(bus.clone(), &found), Bind::NotOurs));
    assert!(calls(&bus).is_empty(), "nothing is sent to a device that is not ours");
}

#[test]
fn each_required_step_that_fails_is_named() {
    for (request, what) in [
        (0x09, "SET_CONFIGURATION refused"),
        (0x0B, "SET_INTERFACE 0 refused"),
        (0x80, "GET_NTB_PARAMETERS refused"),
    ] {
        let mut dev = Dev::ncm(0);
        dev.refuse = Some((request, -32));
        let r = bind(dev.bus(), &dev.found());
        assert!(matches!(r, Bind::Failed(w, -32) if w == what), "{what}");
    }
    let mut dev = Dev::ncm(0);
    dev.params.truncate(20);
    let r = bind(dev.bus(), &dev.found());
    assert!(matches!(r, Bind::Failed("NTB parameters short", -5)));
}
