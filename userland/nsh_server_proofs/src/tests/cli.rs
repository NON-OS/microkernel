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

//! `nsh`, the client's command line, against the loopback host: it checks
//! the host, logs in, types what it is given, prints the shell's output,
//! keeps the record with the host's document, and exits with the shell's
//! status. A host it cannot check, or bad arguments, end it with 255 or 2.

use super::cli_files::files;
use super::loopback_host::serve;
use crate::kit::host::Kernel;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use std::net::TcpListener;
use std::sync::mpsc::channel;

pub(super) fn nsh(args: &[String], typed: &[&[u8]]) -> (i32, String) {
    let (tx, rx) = channel();
    typed.iter().for_each(|t| tx.send(t.to_vec()).expect("typed"));
    let mut out = Vec::new();
    let status = nsh_client::connect::run(args, rx, &mut out);
    drop(tx);
    (status, String::from_utf8_lossy(&out).into_owned())
}

#[test]
fn nsh_logs_in_runs_a_command_and_keeps_the_record() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let host = std::thread::spawn(move || serve(&listener, Kernel::honest(), Table::shipped()));
    let (dir, args) = files("good", &addr, &GrantB::default());
    let (status, out) = nsh(&args, &[b"echo nsh-cli-$((40+2))\n", b"exit\n"]);
    host.join().expect("the host");
    assert_eq!(status, 0, "{out}");
    assert!(out.contains("nsh-cli-42"), "{out}");
    let kept: Vec<_> = std::fs::read_dir(dir.join("records")).expect("records").collect();
    assert_eq!(kept.len(), 2, "the record and the document");
    for f in kept {
        use std::os::unix::fs::PermissionsExt;
        let meta = std::fs::symlink_metadata(f.expect("entry").path()).expect("kept");
        assert!(meta.is_file());
        assert_eq!(meta.permissions().mode() & 0o777, 0o600, "owner only");
    }
}

#[test]
fn nsh_refuses_a_host_it_cannot_check() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let kernel = Kernel { stale: true, ..Kernel::honest() };
    let host = std::thread::spawn(move || serve(&listener, kernel, Table::shipped()));
    let (_, args) = files("stale", &addr, &GrantB::default());
    let (status, out) = nsh(&args, &[b"echo never\n"]);
    host.join().expect("the host");
    assert_eq!(status, 255);
    assert!(!out.contains("never"));
}

#[test]
fn nsh_refuses_bad_arguments() {
    let (_, mut args) = files("args", "127.0.0.1:9", &GrantB::default());
    assert_eq!(nsh(&args[..4], &[]).0, 2, "missing names");
    args.push("--frob".into());
    args.push("x".into());
    assert_eq!(nsh(&args, &[]).0, 2, "an unknown name");
}

#[test]
fn nsh_gives_up_on_a_host_that_never_answers() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    // Takes the connection and says nothing, until the client hangs up.
    let host = std::thread::spawn(move || {
        let (mut sock, _) = listener.accept().expect("accept");
        let mut buf = [0u8; 4096];
        while std::io::Read::read(&mut sock, &mut buf).is_ok_and(|n| n > 0) {}
    });
    let (_, mut args) = files("silent", &addr, &GrantB::default());
    args.extend(["--handshake".into(), "1".into()]);
    let began = std::time::Instant::now();
    let (status, out) = nsh(&args, &[b"echo never\n"]);
    let took = began.elapsed();
    host.join().expect("the host");
    assert_eq!(status, 255);
    assert!(!out.contains("never"));
    assert!(took >= std::time::Duration::from_secs(1), "{took:?}");
    assert!(took < std::time::Duration::from_secs(10), "gave up in time, not {took:?}");
}

#[test]
fn nsh_gives_up_at_once_on_a_host_that_hangs_up_before_the_login() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    // Takes the connection and closes it, as a QEMU forward does when
    // nothing in the guest listens.
    let host = std::thread::spawn(move || drop(listener.accept().expect("accept")));
    let (_, mut args) = files("hangs-up", &addr, &GrantB::default());
    args.extend(["--handshake".into(), "60".into()]);
    let began = std::time::Instant::now();
    let (status, out) = nsh(&args, &[b"echo never\n"]);
    let took = began.elapsed();
    host.join().expect("the host");
    // Refused as gone before the login (Ended::Closed, whose line
    // cli_explain checks): at once, not after the handshake's minute.
    assert_eq!(status, 255);
    assert!(!out.contains("never"));
    assert!(took < std::time::Duration::from_secs(10), "{took:?}");
}
