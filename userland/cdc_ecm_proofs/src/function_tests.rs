// NONOS Operating System (AGPL-3.0-or-later)
//! The function's descriptors as a hostile device could send them: a Union
//! naming the wrong master or a slave that is no data interface, a data
//! alternate 1 that is not exactly one bulk pair, a short Ethernet
//! descriptor. Each makes no function, so the configuration is not set.
//! Offsets are into QEMU's ECM configuration (usbnet_proofs).

use nonos_usbnet::qemu_usb_net::MAC_STRING;

use crate::device::bound;

fn refused(change: impl Fn(&mut Vec<u8>)) -> String {
    bound(change, &MAC_STRING).err().expect("bound")
}

#[test]
fn a_union_must_be_mastered_by_its_interface_and_name_a_data_one() {
    assert_eq!(refused(|c| c[26] = 1), "not ours", "master is the data interface");
    assert_eq!(refused(|c| c[27] = 0), "not ours", "slave is the master, no data interface");
    assert_eq!(refused(|c| c[27] = 5), "not ours", "slave not in the configuration");
}

#[test]
fn the_data_alternate_holds_one_bulk_pair_and_nothing_else() {
    assert_eq!(refused(|c| c[61] = 3), "not ours", "three declared");
    assert_eq!(refused(|c| c[75] = 0x82), "not ours", "two bulk IN");
    assert_eq!(refused(|c| c[69] = 0x03), "not ours", "IN is interrupt");
    assert_eq!(refused(|c| c[60] = 2), "not ours", "no alternate 1");
}

#[test]
fn a_short_ethernet_descriptor_makes_no_function() {
    let cut = |c: &mut Vec<u8>| {
        c[28] = 12;
        c.remove(40);
        c[2] -= 1;
    };
    assert_eq!(refused(cut), "not ours");
}
