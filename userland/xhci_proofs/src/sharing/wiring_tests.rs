// NONOS Operating System (AGPL-3.0-or-later)
//! The server's side of the gate, read from its source, since the server
//! loop is not mounted here: Enable Slot records its sender as the slot's
//! holder, and the claim, the give-back and every other request reach the
//! gate before any handler or the slot's controller.

const FORWARD: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_entry.rs");
const ENABLE: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_enable.rs");
const CLAIMS: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_claims.rs");
const POLICE: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_police.rs");

#[test]
fn enable_slot_records_its_sender_before_anything_else_is_served() {
    let first = FORWARD.find("if req.op == OP_ENABLE_SLOT").expect("enable is taken first");
    assert!(first < FORWARD.find("self.claim_ops").expect("then the claims"));
    assert!(ENABLE.contains("self.share.holders.set(tx[at], sender())"));
}

#[test]
fn the_claim_the_give_back_and_the_rest_meet_the_gate() {
    assert_eq!(CLAIMS.matches("self.police(req, body, pid, tx)").count(), 3);
    assert!(CLAIMS.contains("self.share.holders.forget(slot)"));
    assert!(POLICE.contains("gate(&mut self.share, req.op, body, pid,"));
}
