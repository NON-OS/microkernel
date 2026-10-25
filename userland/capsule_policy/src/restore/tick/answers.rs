// NONOS Operating System (AGPL-3.0-or-later)

//! Setup's kept answers restored, when its marker says it finished.

use nonos_app_skeleton::clients::vfs;
use nonos_policy_proto::setup_record::{check_record, is_done, ANSWERS_LEN, ANSWERS_PATH};
use nonos_policy_proto::setup_record::{DONE, DONE_PATH};

pub(super) fn restore(pid: u32) {
    // One byte past each, so a longer file fails is_done or check_record
    // rather than being cut to a marker or a record that passes.
    let done = vfs::read_file(pid, DONE_PATH, DONE.len() as u32 + 1).is_ok_and(|raw| is_done(&raw));
    if !done {
        return;
    }
    let Ok(raw) = vfs::read_file(pid, ANSWERS_PATH, ANSWERS_LEN as u32 + 1) else {
        return super::super::apply::say(
            b"[POLICY] setup marker without answers: nothing restored\n",
        );
    };
    match check_record(&raw) {
        Ok(record) => super::super::apply::apply(record),
        Err(why) => return super::super::apply::refused(why.name()),
    }
    // What Settings changed after setup goes back over the answers.
    super::super::values::restore(pid);
}
