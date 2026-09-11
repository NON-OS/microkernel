// NONOS Operating System (AGPL-3.0-or-later)
//! A software TPM 2.0 on a socket, for the life of one test.
//!
//! `swtpm socket --tpm2` speaks raw command bytes on its server socket, which
//! is exactly what the kernel puts through the CRB window, so the same
//! `Vec<u8>` the builders produce is what gets sent.
//!
//! Unix sockets rather than TCP ports. Each test needs its own TPM, because
//! each mutates PCR state, and six of them start at once under the test
//! harness: allocating two free ports per instance raced, and the loser saw
//! its connection closed rather than a response. A socket in a directory
//! named after the process cannot collide. The path stays short because the
//! sockaddr caps at 104 bytes and overruns it silently.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub struct Swtpm {
    child: Child,
    stream: UnixStream,
    _state: tempdir::Dir,
}

impl Swtpm {
    /// `None` when swtpm is not installed. The live tests then pass as
    /// skipped, and say so, rather than pretending to a result.
    pub fn start() -> Option<Self> {
        let state = tempdir::Dir::new();
        let sock = state.path().join("s");
        let ctrl = state.path().join("c");
        let child = Command::new("swtpm")
            .args(["socket", "--tpm2"])
            .arg(format!("--tpmstate=dir={}", state.path().display()))
            .arg(format!("--server=type=unixio,path={}", sock.display()))
            .arg(format!("--ctrl=type=unixio,path={}", ctrl.display()))
            .arg("--flags=not-need-init,startup-clear")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Ok(stream) = UnixStream::connect(&sock) {
                return Some(Swtpm { child, stream, _state: state });
            }
            if Instant::now() >= deadline {
                let mut dead = child;
                let _ = dead.kill();
                let _ = dead.wait();
                return None;
            }
            std::thread::sleep(Duration::from_millis(25));
        }
    }

    /// One command, one response. The response size is in its own header.
    pub fn run(&mut self, cmd: &[u8]) -> Vec<u8> {
        self.stream.write_all(cmd).expect("write to swtpm");
        let mut head = [0u8; 10];
        self.stream.read_exact(&mut head).expect("response header");
        let size = u32::from_be_bytes([head[2], head[3], head[4], head[5]]) as usize;
        let mut resp = head.to_vec();
        resp.resize(size.max(10), 0);
        self.stream.read_exact(&mut resp[10..]).expect("response body");
        resp
    }
}

impl Drop for Swtpm {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A directory that goes away with the value, without a crate for it.
pub mod tempdir {
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU32, Ordering};

    static NEXT: AtomicU32 = AtomicU32::new(0);

    pub struct Dir(PathBuf);

    impl Dir {
        pub fn new() -> Self {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let p = PathBuf::from(format!("/tmp/nonos-tpm-{}-{n}", std::process::id()));
            let _ = std::fs::remove_dir_all(&p);
            std::fs::create_dir_all(&p).expect("state dir");
            Dir(p)
        }
        pub fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
