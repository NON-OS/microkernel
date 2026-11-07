// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The four lock operations (`nonos_lock::wire`). Only the shell checks,
//! loads and reads the standing; the shell and first-boot setup set. The
//! pid in each body must be the sender's. The blob to keep is made once for
//! each change of the record and handed back with every answer that
//! changed it.

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;
use nonos_lock::wire::{
    parse_check, parse_load, parse_set, parse_state, standing, E_ACCES, E_EXIST, E_INVAL, E_NOENT,
    E_NOMEM, E_WAIT, E_WRONG, OK, OP_LOCK_CHECK, OP_LOCK_LOAD, OP_LOCK_SET, OP_LOCK_STATE,
};
use nonos_lock::SALT_LEN;

use super::blob::{self, ReadError};
use super::keeper::{Keeper, Outcome};
use crate::protocol::{encode_response, Request};

/// The verifier, and the blob made for it last.
#[derive(Default)]
pub struct Lock {
    pub keeper: Keeper,
    pub blob: Vec<u8>,
}

impl Lock {
    fn remake(&mut self) {
        self.blob = self.keeper.record().map(blob::make).unwrap_or_default();
    }
}

pub fn handle(lock: &mut Lock, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let now = mk_uptime_ms();
    let (status, body) = match req.op {
        OP_LOCK_SET => set(lock, req.payload, sender_pid, now),
        OP_LOCK_CHECK => check(lock, req.payload, sender_pid, now),
        OP_LOCK_LOAD => load(lock, req.payload, sender_pid, now),
        OP_LOCK_STATE => state(lock, req.payload, sender_pid, now),
        _ => (E_INVAL, Vec::new()),
    };
    encode_response(req.seq, status, &body)
}

fn own(payload_pid: u32, sender_pid: u32) -> bool {
    crate::server::resolve_caller(payload_pid, sender_pid).is_some()
}

fn set(lock: &mut Lock, payload: &[u8], sender: u32, now: i64) -> (i32, Vec<u8>) {
    let Some((pid, current, new)) = parse_set(payload) else {
        return (E_INVAL, Vec::new());
    };
    if !own(pid, sender) {
        return (E_ACCES, Vec::new());
    }
    let mut draw = [0u8; 32];
    if !crate::entropy::gather_secret(&mut draw) {
        return (E_NOMEM, Vec::new());
    }
    let mut salt = [0u8; SALT_LEN];
    salt.copy_from_slice(&draw[..SALT_LEN]);
    nonos_lock::wipe::secure_zero(&mut draw);
    let outcome = lock.keeper.set(current, new, &salt, now);
    nonos_lock::wipe::secure_zero(&mut salt);
    answer(lock, outcome, now)
}

fn check(lock: &mut Lock, payload: &[u8], sender: u32, now: i64) -> (i32, Vec<u8>) {
    let Some((pid, passphrase)) = parse_check(payload) else {
        return (E_INVAL, Vec::new());
    };
    if !own(pid, sender) {
        return (E_ACCES, Vec::new());
    }
    let outcome = lock.keeper.check(passphrase, now);
    answer(lock, outcome, now)
}

fn load(lock: &mut Lock, payload: &[u8], sender: u32, now: i64) -> (i32, Vec<u8>) {
    let Some((pid, bytes)) = parse_load(payload) else {
        return (E_INVAL, Vec::new());
    };
    if !own(pid, sender) {
        return (E_ACCES, Vec::new());
    }
    if lock.keeper.held() {
        return (E_EXIST, Vec::new());
    }
    match blob::read(bytes) {
        Ok(record) => {
            let _ = lock.keeper.load(record, now);
            lock.blob = bytes.to_vec();
            (OK, Vec::new())
        }
        Err(ReadError::NoKey) => (E_NOENT, Vec::new()),
        Err(ReadError::Malformed) => (E_INVAL, Vec::new()),
    }
}

fn state(lock: &mut Lock, payload: &[u8], sender: u32, now: i64) -> (i32, Vec<u8>) {
    let Some(pid) = parse_state(payload) else {
        return (E_INVAL, Vec::new());
    };
    if !own(pid, sender) {
        return (E_ACCES, Vec::new());
    }
    (OK, standing(lock.keeper.held(), lock.keeper.count(now), &lock.blob))
}

/// The status and body for an outcome; a changed record is sealed again.
fn answer(lock: &mut Lock, outcome: Outcome, now: i64) -> (i32, Vec<u8>) {
    let count = |l: &Lock| -> Vec<u8> { l.keeper.count(now).encode().to_vec() };
    match outcome {
        Outcome::Taken => {
            lock.remake();
            (OK, lock.blob.clone())
        }
        Outcome::Wrong => {
            lock.remake();
            let mut body = count(lock);
            body.extend_from_slice(&lock.blob);
            (E_WRONG, body)
        }
        Outcome::Wait => (E_WAIT, count(lock)),
        Outcome::NoRecord => (E_NOENT, Vec::new()),
        Outcome::Held => (E_EXIST, Vec::new()),
        Outcome::Invalid => (E_INVAL, Vec::new()),
        Outcome::NoMemory => (E_NOMEM, Vec::new()),
    }
}
