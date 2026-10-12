// NONOS Operating System (AGPL-3.0-or-later)
//! Every step that fails stops the bind with its name and errno.

use nonos_usbnet::mock::{drawn, Call};
use nonos_usbnet::Bind;

use crate::ax::bind;
use crate::bound::found;
use crate::chip::Chip;
use crate::layout::CONFIG;

fn bind_with(
    chip: Chip,
) -> (Bind<crate::ax::link::Ax88179<nonos_usbnet::mock::MockBus>>, Vec<Call>) {
    let bus = chip.bus();
    let r = bind(bus.clone(), &found(0x0b95, 0x1790, 0xff, &[&CONFIG]), drawn());
    let calls = bus.0.borrow().calls.clone();
    (r, calls)
}

#[test]
fn each_refused_register_write_is_a_named_failure_with_its_errno() {
    let cases: [((u8, u16, u16), &str); 10] = [
        ((0x09, 1, 0), "SET_CONFIGURATION refused"),
        ((0x01, 0x26, 2), "AX_PHYPWR_RSTCTL power down refused"),
        ((0x01, 0x33, 1), "AX_CLK_SELECT write refused"),
        ((0x01, 0x10, 6), "AX_NODE_ID write refused"),
        ((0x01, 0x2e, 5), "AX_RX_BULKIN_QCTRL write refused"),
        ((0x01, 0x34, 1), "AX_RXCOE_CTL write refused"),
        ((0x01, 0x0b, 2), "AX_RX_CTL start refused"),
        ((0x01, 0x22, 2), "AX_MEDIUM_STATUS_MODE write refused"),
        ((0x02, 3, 0x1f), "PHY page 3 select refused"),
        ((0x02, 3, 0x00), "PHY BMCR unread"),
    ];
    for ((r, v, i), name) in cases {
        let (out, _) = bind_with(Chip { refuse: Some((r, v, i, -71)), ..Chip::default() });
        assert!(matches!(out, Bind::Failed(n, -71) if n == name), "{name}");
    }
}

#[test]
fn a_stalled_set_interface_is_ignored_as_linux_ignores_it() {
    let (r, _) = bind_with(Chip { refuse: Some((0x0b, 0, 0, -32)), ..Chip::default() });
    assert!(matches!(r, Bind::Ours(_)));
}
