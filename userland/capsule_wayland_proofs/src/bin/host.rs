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

//! nonos-wayland-host: the personality's Wayland server on a Unix socket of
//! the host, so a real client (foot, GTK, Qt) runs against the shipped code.
//!
//!   nonos-wayland-host <socket> [control-socket]
//!
//! Each client is one guest (session.rs). The control socket takes one line
//! a command (control.rs): input to route, the display's size, a dump of
//! the window's pixels. NONOS_WAYLAND_DISPLAY=<w>x<h> sets the display's
//! size from the start, which sets its scale (the brand rule);
//! NONOS_WAYLAND_CLIPBOARD=1 gives the desk a clipboard service, which the
//! server copies a guest's text to and pastes from. Every line the server
//! says goes to stderr.

use std::os::fd::AsRawFd;
use std::os::unix::net::{UnixListener, UnixStream};

use nonos_capsule_wayland_proofs::host::{control, Client};

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: nonos-wayland-host <socket> [control-socket]");
        std::process::exit(2);
    };
    nonos_libc::world::with(|w| w.echo = true);
    if let Some((w, h)) = std::env::var("NONOS_WAYLAND_DISPLAY").ok().and_then(|v| {
        let (w, h) = v.split_once('x')?;
        Some((w.parse().ok()?, h.parse().ok()?))
    }) {
        nonos_app_skeleton::desk::with(|d| (d.width, d.height) = (w, h));
    }
    if std::env::var("NONOS_WAYLAND_CLIPBOARD").is_ok_and(|v| v == "1") {
        nonos_app_skeleton::desk::with(|d| d.clipboard_on = true);
    }
    let _ = std::fs::remove_file(&path);
    let listener = match UnixListener::bind(&path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[host] bind {path}: {e}");
            std::process::exit(1);
        }
    };
    let ctl = args.next().and_then(|p| {
        let _ = std::fs::remove_file(&p);
        UnixListener::bind(&p).ok()
    });
    listener.set_nonblocking(true).ok();
    ctl.iter().for_each(|c| drop(c.set_nonblocking(true)));
    // A server as slow as one under emulation: each turn takes this long.
    let slow = std::env::var("NONOS_WAYLAND_SLOW_MS").ok().and_then(|v| v.parse::<u64>().ok());
    let mut clients: Vec<Client> = Vec::new();
    let mut ctls: Vec<(UnixStream, Vec<u8>)> = Vec::new();
    loop {
        let mut polls = vec![poll_of(listener.as_raw_fd())];
        polls.extend(ctl.iter().map(|c| poll_of(c.as_raw_fd())));
        polls.extend(ctls.iter().map(|c| poll_of(c.0.as_raw_fd())));
        polls.extend(clients.iter().map(Client::poll));
        let wait = clients.iter_mut().filter_map(Client::tick).min().unwrap_or(50).min(50);
        // SAFETY: a slice of pollfd this frame owns.
        unsafe { libc::poll(polls.as_mut_ptr(), polls.len() as _, wait as i32) };
        if let Some(ms) = slow {
            std::thread::sleep(std::time::Duration::from_millis(ms));
        }
        while let Ok((s, _)) = listener.accept() {
            eprintln!("[host] client connected");
            clients.push(Client::new(s));
        }
        if let Some(c) = &ctl {
            while let Ok((s, _)) = c.accept() {
                s.set_nonblocking(true).ok();
                ctls.push((s, Vec::new()));
            }
        }
        ctls.retain_mut(|(s, buf)| control::read(s, buf, &mut clients));
        clients.retain_mut(Client::turn);
    }
}

fn poll_of(fd: i32) -> libc::pollfd {
    libc::pollfd { fd, events: libc::POLLIN, revents: 0 }
}
