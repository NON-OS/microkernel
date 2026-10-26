// NONOS Operating System (AGPL-3.0-or-later)

//! One try at writing the settings record, and the passes it has taken.

use spin::Mutex;

use super::live::write_record;
use crate::progress::{Progress, Say};
use crate::progress_say::{done, first_late};

/// The passes the record now being written has taken.
static PROGRESS: Mutex<Progress> = Mutex::new(Progress::new());

/// Write the record as the store holds it now: true once it is kept. The
/// first try vfs did not answer in time, and the try that kept it, are said.
pub(super) fn once() -> bool {
    let (kept, late) = write_record(&super::snapshot::record());
    let mut progress = PROGRESS.lock();
    if let Say::FirstLate(pass) = progress.pass(late) {
        first_late(b"settings record", pass);
    }
    if kept {
        done(b"settings record", *progress);
        *progress = Progress::new();
    }
    kept
}
