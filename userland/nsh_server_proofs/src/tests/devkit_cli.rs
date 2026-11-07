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

//! `nsh` against a loopback host serving the devkit's records at this
//! machine's clock, opened as nsh.front opens them, with host keys the
//! machine made itself: the client checks the host's keys, enrolment and
//! attestation, logs in under the devkit's grant, runs a command and exits.
//! The same records served a day on are refused: the revocation list has
//! gone stale, and no one is admitted until the operator refreshes it.

use super::cli::nsh;
use super::cli_files::args;
use super::cli_releases::releases;
use super::devkit_host::{config, file, keys, made_at, roles};
use super::loopback_host::{serve_on, Clock};
use crate::kit::devkit::{client, write, Window};
use crate::kit::host::Kernel;
use crate::kit::roles_build::HOUR;
use nsh_server::Server;
use std::net::TcpListener;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn wall_ms() -> u64 {
    let since = SystemTime::now().duration_since(UNIX_EPOCH).expect("a clock after 1970");
    u64::try_from(since.as_millis()).expect("a clock in range")
}

/// The client's files, made at `w`, beside the spec's releases.
fn client_dir(name: &str, w: &Window) -> PathBuf {
    let d = std::env::temp_dir().join(format!("nsh-devkit-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join("records")).expect("a directory");
    write(&d, &client(w)).expect("the client files");
    std::fs::write(d.join("releases"), releases()).expect("the releases");
    d
}

/// Serve the devkit's host files once at `host_ms`, and run `nsh`.
fn login(name: &str, made_ms: u64, host_ms: u64) -> (i32, String) {
    let (w, files) = made_at(made_ms);
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let clock = Clock::at(host_ms);
    let enrol = file(&files, "enrol");
    let s = Server::new(config(enrol), Kernel::honest(), keys(), roles(&files), clock.now());
    let s = s.expect("a valid config");
    let host = std::thread::spawn(move || serve_on(&listener, s, &clock));
    let args = args(&client_dir(name, &w), &addr);
    let got = nsh(&args, &[b"echo nsh-devkit-$((40+2))\n", b"exit\n"]);
    host.join().expect("the host");
    got
}

#[test]
fn nsh_logs_in_to_the_devkit_host_at_the_wall_clock() {
    let now = wall_ms();
    let (status, out) = login("good", now, now);
    assert_eq!(status, 0, "{out}");
    assert!(out.contains("nsh-devkit-42"), "{out}");
}

#[test]
fn nsh_is_refused_by_devkit_fixtures_a_day_old() {
    let now = wall_ms();
    let stale = Window::at(now).expect("a window").list_stale_ms() + HOUR;
    let (status, out) = login("stale", now, stale);
    assert_ne!(status, 0, "{out}");
    assert!(!out.contains("nsh-devkit-42"), "{out}");
}
