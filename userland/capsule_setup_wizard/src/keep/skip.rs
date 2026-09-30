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

//! Skipping setup on a boot after one that kept its answers.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_yield, Deadline};
use nonos_policy_proto::setup_record::{is_done, Record, ANSWERS_LEN, ANSWERS_PATH};
use nonos_policy_proto::setup_record::{DONE, DONE_PATH};
use nonos_policy_proto::Field;

use crate::server::say::say;

/// How long to wait for vfs to load the store. A boot with no NONOS disk
/// settles at once; this bounds only a vfs that never answers.
const STORE_WAIT_MS: u64 = 30_000;
/// How long to wait for the policy service to take the answers back.
const POLICY_WAIT_MS: u64 = 5_000;

/*
 * The apps turned off, when an earlier boot kept both the answers and the
 * marker; every app on for a record kept before setup asked.
 */
pub fn already_done() -> Option<u8> {
    let until = Deadline::after_ms(STORE_WAIT_MS);
    while vfs::store_settled() != Ok(true) {
        if until.expired() {
            say(b"[SETUP] the store did not load in time; asking again\n");
            return None;
        }
        let _ = mk_yield();
    }
    let pid = mk_getpid();
    let done = vfs::read_file(pid, DONE_PATH, DONE.len() as u32).is_ok_and(|raw| is_done(&raw));
    let raw = vfs::read_file(pid, ANSWERS_PATH, ANSWERS_LEN as u32);
    raw.ok().and_then(|raw| Record::decode(&raw)).filter(|_| done).map(|r| r.apps_off)
}

/// Wait until the policy service has restored the kept answers, so the
/// desktop setup starts on exit reads them rather than the defaults. It sets
/// Persistent back only when it restores, so that field is the signal.
pub fn wait_for_policy() {
    let until = Deadline::after_ms(POLICY_WAIT_MS);
    while !until.expired() {
        let port = nonos_policy_client::lookup();
        if port.and_then(|p| nonos_policy_client::get_bool(p, Field::Persistent)) == Some(true) {
            return;
        }
        let _ = mk_yield();
    }
    say(b"[SETUP] the policy service did not restore the answers in time\n");
}
