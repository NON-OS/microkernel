// NONOS Operating System (AGPL-3.0-or-later)

//! One vfs ask for the stage the restore stands at, its answer applied.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_policy_proto::kept_values::VALUES_ASK;
use nonos_policy_proto::settings_record::VALUES_PATH;
use nonos_policy_proto::setup_record::{check_record, is_done, ANSWERS_LEN, ANSWERS_PATH};
use nonos_policy_proto::setup_record::{DONE, DONE_PATH};

use super::super::stage::{Got, Stage};
use super::super::{apply, values};
use crate::progress::VFS_DEADLINE_MS;

/// A file read: its bytes, absent or refused, or no answer in time.
enum Read {
    Bytes(Vec<u8>),
    Absent,
    Late,
}

/* One byte past each record, so a longer file fails is_done or check_record
 * rather than being cut to a marker or a record that passes. */
pub(super) fn ask(stage: Stage, pid: u32) -> Got {
    match (stage, stage_read(stage, pid)) {
        (_, Some(Read::Late)) => Got::Late,
        (Stage::Marker, Some(Read::Bytes(raw))) if is_done(&raw) => Got::Yes,
        (Stage::Answers, Some(Read::Bytes(raw))) => match check_record(&raw) {
            Ok(record) => {
                apply::apply(record);
                Got::Yes
            }
            Err(why) => {
                apply::refused(why.name());
                Got::No
            }
        },
        (Stage::Answers, _) => {
            apply::say(b"[POLICY] setup marker without answers: nothing restored\n");
            Got::No
        }
        (Stage::Values, Some(Read::Bytes(raw))) => {
            values::apply(&raw);
            Got::Yes
        }
        (Stage::Settle, None) => super::wait::settle(),
        (Stage::Finished, _) => Got::Yes,
        _ => Got::No,
    }
}

fn stage_read(stage: Stage, pid: u32) -> Option<Read> {
    let (path, max) = match stage {
        Stage::Marker => (DONE_PATH, DONE.len() + 1),
        Stage::Answers => (ANSWERS_PATH, ANSWERS_LEN + 1),
        Stage::Values => (VALUES_PATH, VALUES_ASK),
        Stage::Settle | Stage::Finished => return None,
    };
    Some(match vfs::read_file_within(pid, path, max as u32, VFS_DEADLINE_MS) {
        Ok(raw) => Read::Bytes(raw),
        Err(vfs::TIMEOUT) => Read::Late,
        Err(_) => Read::Absent,
    })
}
