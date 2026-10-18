// NONOS Operating System (AGPL-3.0-or-later)
//! A device whose dwNtbOutMaxSize cannot carry one full Ethernet frame in
//! one NTB16 is not bound: every frame the stack sends must go out.

use nonos_usbnet::mock::drawn;
use nonos_usbnet::Bind;

use crate::device::Dev;
use crate::ncm::bind;
use crate::spec::ntb_params;

#[test]
fn a_device_whose_largest_block_cannot_carry_a_frame_is_refused() {
    for out_max in [0, 1000, 1543] {
        let mut dev = Dev::ncm(0);
        dev.params = ntb_params(1, 16384, out_max, (4, 0, 4));
        let r = bind(dev.bus(), &dev.found(), drawn());
        assert!(matches!(r, Bind::Failed("NTB out size under one frame", -5)), "{out_max}");
    }
    let mut dev = Dev::ncm(0);
    dev.params = ntb_params(1, 16384, 1544, (4, 0, 4));
    let Bind::Ours(nic) = bind(dev.bus(), &dev.found(), drawn()) else {
        panic!("1544 carries one")
    };
    assert_eq!((nic.shape.max, nic.shape.out_max), (1544, 1544));
}
