// NONOS Operating System (AGPL-3.0-or-later)
//! Finding the NCM function, and leaving what is not one.

use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS};

use crate::ncm::function::find_ncm;
use crate::spec::{config_ncm, config_vendor};

#[test]
fn the_ncm_function_its_data_alternate_mac_index_and_capabilities_are_read() {
    let f = find_ncm(&config_ncm(1, 0x31)).unwrap();
    assert_eq!((f.comm, f.data, f.data_alt, f.mac_index), (0, 1, 1, 4));
    assert_eq!((f.max_segment, f.caps), (1514, 0x31));
    assert_eq!((f.pipes.bulk_in, f.pipes.bulk_out), (0x82, 0x03));
    assert_eq!((f.pipes.max_packet_in, f.pipes.max_packet_out), (512, 512));
}

#[test]
fn ecm_rndis_and_vendor_functions_are_not_ncm() {
    assert_eq!(find_ncm(&CONFIG_ECM), None, "ECM, subclass 0x06, is the ECM driver's");
    assert_eq!(find_ncm(&CONFIG_RNDIS), None);
    assert_eq!(find_ncm(&config_vendor()), None);
}

/// Byte offsets in config_ncm: the comm interface's protocol, the Union's
/// length, the NCM descriptor's length and the data interfaces' class.
const PROTOCOL: usize = 9 + 7;
const UNION_TYPE: usize = 9 + 9 + 5 + 1;
const NCM_TYPE: usize = 9 + 9 + 5 + 5 + 13 + 1;
const DATA_ON_CLASS: usize = 9 + 9 + 5 + 5 + 13 + 6 + 7 + 9 + 5;

#[test]
fn a_function_missing_a_part_cdc_ncm_bind_common_requires_is_not_taken() {
    for (at, value, why) in [
        (PROTOCOL, 0x01, "a protocol cdc_devs[] does not match"),
        (UNION_TYPE, 0x25, "no Union"),
        (NCM_TYPE, 0x25, "no NCM functional descriptor"),
        (DATA_ON_CLASS, 0xFF, "no CDC data interface with bulk pipes"),
    ] {
        let mut c = config_ncm(1, 0);
        c[at] = value;
        assert_eq!(find_ncm(&c), None, "{why}");
    }
}

#[test]
fn a_cut_short_configuration_is_read_no_further_than_its_bytes() {
    let c = config_ncm(1, 0);
    for n in 0..c.len() {
        let _ = find_ncm(&c[..n]);
    }
    assert_eq!(find_ncm(&c[..c.len() - 7]), None, "the bulk OUT endpoint is cut");
}
