// NONOS Operating System (AGPL-3.0-or-later)

//! The settings record's write against vfs itself, and whether a failure
//! was vfs giving no answer in time.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;
use nonos_policy_proto::settings_record::{SETTINGS_DIR, VALUES_PATH};

use super::write_steps::{write, Vfs};


struct Live {
    pid: u32,
    late: bool,
}

impl Live {
    fn seen(&mut self, r: Result<(), &'static str>) -> Result<(), ()> {
        self.late |= r == Err(vfs::TIMEOUT);
        r.map_err(|_| ())
    }
}

impl Vfs for Live {
    fn mkdir(&mut self, path: &[u8]) -> Result<(), ()> {
        let r = vfs::mkdir(self.pid, path);
        self.seen(r)
    }
    fn unlink(&mut self, path: &[u8]) -> Result<(), ()> {
        let r = vfs::unlink(self.pid, path);
        self.seen(r)
    }
    fn write_file(&mut self, path: &[u8], data: &[u8]) -> Result<(), ()> {
        let r = vfs::write_file(self.pid, path, data);
        self.seen(r)
    }
    fn persist(&mut self, path: &[u8]) -> Result<(), ()> {
        let r = vfs::persist(self.pid, path);
        self.seen(r)
    }
}

/// Write `bytes` as the settings record and keep it: whether it was kept,
/// and whether any call got no answer in time.
pub(super) fn write_record(bytes: &[u8]) -> (bool, bool) {
    let mut live = Live { pid: mk_getpid(), late: false };
    let kept = write(&mut live, &[b"/nonos", SETTINGS_DIR], VALUES_PATH, bytes).is_ok();
    (kept, live.late)
}
