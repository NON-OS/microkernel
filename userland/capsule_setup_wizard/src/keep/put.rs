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
use nonos_policy_proto::setup_record::{Record, ANSWERS_LEN, ANSWERS_PATH};
use nonos_policy_proto::setup_record::{ANSWERS_V1_LEN, ANSWERS_V2_LEN, ANSWERS_V3_LEN};
use nonos_policy_proto::setup_record::{ANSWERS_V4_LEN, ANSWERS_V5_LEN};

use crate::server::say::say;

/*
 * The store replaces a record only with one of the same length, so where an
 * earlier build left an older record the current one would be refused. That
 * version is kept there instead, holding what it can, and this boot's copy,
 * which the installer carries to a new disk, holds everything.
 */
pub(super) fn put_answers(pid: u32, record: &Record) -> Result<(), &'static str> {
    let old = vfs::read_file(pid, ANSWERS_PATH, ANSWERS_LEN as u32).map_or(0, |raw| raw.len());
    match old {
        ANSWERS_V1_LEN => put(pid, ANSWERS_PATH, &record.answers.encode_v1())?,
        ANSWERS_V2_LEN => put(pid, ANSWERS_PATH, &record.answers.encode())?,
        ANSWERS_V3_LEN => put(pid, ANSWERS_PATH, &record.encode_v3())?,
        ANSWERS_V4_LEN => put(pid, ANSWERS_PATH, &record.encode_v4())?,
        ANSWERS_V5_LEN => put(pid, ANSWERS_PATH, &record.encode_v5())?,
        _ => return put(pid, ANSWERS_PATH, &record.encode()),
    }
    say(b"[SETUP] an older record is in the store: what it cannot hold is not kept\n");
    let _ = vfs::unlink(pid, ANSWERS_PATH);
    vfs::write_file(pid, ANSWERS_PATH, &record.encode())
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
