// NONOS Operating System (AGPL-3.0-or-later)
//! A device that refuses SET_NTB_INPUT_SIZE keeps its own input size only
//! when that is one NCM 1.0, 6.2.7 allows: 2048 bytes at least.

use nonos_usbnet::mock::drawn;
use nonos_usbnet::Bind;

use crate::device::Dev;
use crate::ncm::bind;
use crate::spec::ntb_params;

#[test]
fn a_refused_input_size_under_the_spec_minimum_is_a_named_failure() {
    for in_max in [0, 1, 100, 1024, 2047] {
        let mut dev = Dev::ncm(0);
        dev.refuse = Some((0x86, -32));
        dev.params = ntb_params(1, in_max, 16384, (4, 0, 4));
        let r = bind(dev.bus(), &dev.found(), drawn());
        assert!(matches!(r, Bind::Failed("dwNtbInMaxSize under 2048", -32)), "{in_max}");
    }
}
