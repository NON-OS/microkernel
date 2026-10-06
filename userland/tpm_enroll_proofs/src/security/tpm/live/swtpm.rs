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

//! A software TPM 2.0 on TCP, for the life of one test.
//!
//! `swtpm socket --tpm2` takes raw TPM command bytes on its server port, the
//! bytes the kernel puts through the CRB window. The control port sits one
//! above, where the swtpm TCTI of tpm2-tools looks for it. One instance at a
//! time: the transport is one process-wide port, and two at once could race.

use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::Ordering;
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use super::dir::Dir;
use super::swtpm_io::free_pair;
use crate::security::tpm::PORT;

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

pub struct Swtpm {
    pub(super) child: Child,
    pub(super) port: u16,
    pub dir: Dir,
    pub(super) _one: MutexGuard<'static, ()>,
}

impl Swtpm {
    /// A fresh TPM, or `None` when swtpm is not installed. The test then
    /// passes as skipped, and says so.
    pub fn start(test: &str) -> Option<Self> {
        Self::start_in(Dir::new(), test)
    }

    /// A TPM over the state already in `dir`, as swtpm_setup left it.
    pub fn start_in(dir: Dir, test: &str) -> Option<Self> {
        let one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
        let port = free_pair();
        let Ok(child) = Command::new("swtpm")
            .args(["socket", "--tpm2", "--tpmstate", &format!("dir={}", dir.path().display())])
            .args(["--server", &format!("type=tcp,port={port}")])
            .args(["--ctrl", &format!("type=tcp,port={}", port + 1)])
            .args(["--flags", "not-need-init,startup-clear"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        else {
            eprintln!("{test}: swtpm not installed, live test skipped");
            return None;
        };
        let t = Swtpm { child, port, dir, _one: one };
        let deadline = Instant::now() + Duration::from_secs(10);
        while TcpStream::connect(("127.0.0.1", port)).is_err() {
            assert!(Instant::now() < deadline, "swtpm never opened port {port}");
            std::thread::sleep(Duration::from_millis(20));
        }
        PORT.store(port, Ordering::SeqCst);
        Some(t)
    }
}
