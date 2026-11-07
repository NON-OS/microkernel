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

//! The machine's own records, read for each connection and handed to
//! nsh.front: the enrolment certificate the operator signed for this
//! machine, the role table, the key id it is pinned under, and the
//! revocation list. They are read again for every connection, so a fresh
//! revocation list the operator puts in place applies to the next login
//! with no reboot, and the image itself never ages: only a list the
//! operator stops refreshing does. All four are signed or pinned data, so
//! nsh.front checks them; nsh.attest only reads and frames them, taking the
//! anchor in hex as `nsh-roles table` prints it, or raw (anchor.rs). A record
//! that is absent, over its cap or unreadable is sent empty, and nsh.front
//! turns the connection away as not enrolled.

use alloc::vec;
use alloc::vec::Vec;

use nsh_front_wire::{
    records_len, write_records, Records, ENROL_MAX, LIST_MAX, OP_RECORDS, ST_OK, TABLE_MAX,
};

use crate::anchor::{anchor, ANCHOR_FILE_MAX};

use crate::reply::{err, frame};
use crate::request::ENROL_PATH;
use crate::store::{self, Found};

pub const TABLE_PATH: &[u8] = b"/nonos/nsh/roles";
pub const ANCHOR_PATH: &[u8] = b"/nonos/nsh/roles.anchor";
pub const LIST_PATH: &[u8] = b"/nonos/nsh/revocations";

fn bytes(path: &[u8], cap: usize) -> Vec<u8> {
    match store::read(path, cap) {
        Found::Bytes(b) => b,
        Found::Absent | Found::Over | Found::Fault => Vec::new(),
    }
}

/// The `OP_RECORDS` answer for this connection.
pub fn answer() -> Vec<u8> {
    let enrol = bytes(ENROL_PATH, ENROL_MAX);
    let table = bytes(TABLE_PATH, TABLE_MAX);
    let anchor =
        anchor(&bytes(ANCHOR_PATH, ANCHOR_FILE_MAX)).map(|a| a.to_vec()).unwrap_or_default();
    let list = bytes(LIST_PATH, LIST_MAX);
    let r = Records { enrol: &enrol, table: &table, anchor: &anchor, list: &list };
    let Some(len) = records_len(&r) else {
        return err(OP_RECORDS, nsh_front_wire::ST_OVERFLOW);
    };
    let mut body = vec![0u8; len];
    match write_records(&mut body, &r) {
        Some(_) => frame(OP_RECORDS, ST_OK, &body),
        None => err(OP_RECORDS, nsh_front_wire::ST_OVERFLOW),
    }
}
