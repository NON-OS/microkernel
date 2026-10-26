// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The key gate's bounds: the ops it leaves to any owner, the store's room
//! for every name it admits, and its place in dispatch before any handler.

use crate::server::protocol::callers::{callers, may_ask, CREATORS};
use crate::server::protocol::types::*;
use crate::store::{MAX_KEYS, MAX_KEYS_PER_OWNER};

const DISPATCH: &str = include_str!("../../capsule_keyring/src/server/dispatch.rs");

#[test]
fn every_other_op_stays_any_owners() {
    let open = [OP_RETRIEVE, OP_DELETE, OP_LOCK, OP_UNLOCK, OP_METADATA, OP_COUNT, OP_LIST];
    for op in open.into_iter().chain([OP_WALLET_ADDRESS, OP_WALLET_EXPORT, OP_LIST_WALLET_RAILS]) {
        assert_eq!(callers(op), None, "op {op}");
        assert!(may_ask(op, 61, |_| None) && may_ask(op, 0, |_| Some(0)), "op {op}");
    }
}

#[test]
fn the_creators_fit_the_store_and_name_every_caller() {
    assert!(CREATORS.len() * MAX_KEYS_PER_OWNER <= MAX_KEYS);
    for names in (0..64).filter_map(callers) {
        assert!(names.iter().all(|n| CREATORS.contains(n)));
    }
    let guard = "    if !super::key_gate::allowed(req.op, sender_pid) {\n        \
                 return encode_response(req.seq, EACCES, &[]);";
    let gate = DISPATCH.find(guard).expect("refused with EACCES");
    assert!(gate < DISPATCH.find("match req.op").unwrap(), "before any handler");
}
