// NONOS Operating System (AGPL-3.0-or-later)
//! What counts as binding a device, and the server's calls into the gate,
//! read from its source since the server loop is not mounted here: a bind
//! is weighed after the slot is made the real one and before it reaches a
//! controller, a re-enabled slot is judged afresh, and the gate's ops are
//! the kernel's alone.

use crate::protocol::*;
use crate::slots::binds::binds;

const FORWARD: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_entry.rs");
const ENABLE: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_enable.rs");
const AUTH: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_auth.rs");

#[test]
fn pipes_hubs_and_set_configuration_bind_and_reads_do_not() {
    for op in [OP_ALLOC_TRANSFER_RING, OP_CONFIGURE_BULK, OP_CONFIGURE_BULK2] {
        assert!(binds(op, &[4]));
    }
    for op in [OP_CONFIGURE_STREAMS, OP_CONFIGURE_ISOCH, OP_CONFIGURE_HUB] {
        assert!(binds(op, &[4]));
    }
    assert!(binds(OP_CONTROL_TRANSFER, &[4, 0, 0x00, 0x09, 1, 0, 0, 0, 0, 0]));
    assert!(!binds(OP_CONTROL_TRANSFER, &[4, 0, 0x80, 0x06, 0, 2, 0, 0, 9, 0]));
    for op in [OP_GET_CONFIG_DESCRIPTOR, OP_BULK_OUT, OP_INTERRUPT_IN, OP_CLAIM_INTERFACE] {
        assert!(!binds(op, &[4, 0, 0x00, 0x09]));
    }
}

#[test]
fn the_server_weighs_a_bind_before_any_controller_sees_it() {
    let real = FORWARD.find("self.virt.real(s)").expect("the real slot");
    let gate = FORWARD.find("if !self.bind_refused(req, body, tx)").expect("the gate");
    assert!(real < gate && gate < FORWARD.rfind("self.inner(req, body, tx)").unwrap());
    assert!(ENABLE.contains("self.share.auth.forget(tx[at])"));
    let bind = include_str!("../../../capsule_driver_xhci/src/server/mux_admit.rs");
    assert!(bind.contains("Some(v) => m.share.auth.admit_set(slot, v),"), "SET weighs its own");
    assert!(AUTH.contains("_ if sender() != 0 => E_PERM,"));
}
