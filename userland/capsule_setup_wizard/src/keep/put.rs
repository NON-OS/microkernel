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

/* Writing one kept file, and the answers record in whichever version fits. */

use nonos_app_skeleton::clients::vfs;
use nonos_policy_proto::setup_record::{Answers, ANSWERS_LEN, ANSWERS_PATH, ANSWERS_V1_LEN};

use crate::server::say::say;

/*
 * The store replaces a record only with one of the same length, so where an
 * earlier build left a version 1 record the current one would be refused.
 * Version 1 is kept there instead, without the name and the Qwen model, and
 * this boot's copy, which the installer carries to a new disk, holds them.
 */
pub(super) fn put_answers(pid: u32, answers: &Answers) -> Result<(), &'static str> {
    let old = vfs::read_file(pid, ANSWERS_PATH, ANSWERS_LEN as u32);
    if !old.is_ok_and(|raw| raw.len() == ANSWERS_V1_LEN) {
        return put(pid, ANSWERS_PATH, &answers.encode());
    }
    put(pid, ANSWERS_PATH, &answers.encode_v1())?;
    say(b"[SETUP] an older record is in the store: name and Qwen model not kept\n");
    let _ = vfs::unlink(pid, ANSWERS_PATH);
    vfs::write_file(pid, ANSWERS_PATH, &answers.encode())
}

/*
 * A record loaded from an earlier boot belongs to nobody, and only a file's
 * owner may persist it, so it is unlinked and written afresh first.
 */
pub(super) fn put(pid: u32, path: &[u8], bytes: &[u8]) -> Result<(), &'static str> {
    let _ = vfs::unlink(pid, path);
    vfs::write_file(pid, path, bytes)?;
    vfs::persist(pid, path)
}
