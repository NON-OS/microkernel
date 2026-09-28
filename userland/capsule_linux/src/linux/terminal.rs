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

//! A program the terminal's `linux` command names.
//!
//! The terminal starts this capsule with `linux <program> [args...]` and
//! shows what it prints. The program is a path in the Linux tree, or a bare
//! name looked up in /bin. A name that is a link, as busybox's applets are,
//! runs what it links to with the typed name first, which is how a multi-call
//! program tells which one it is. Like every program here, it must carry a
//! proof that verifies; naming one grants nothing.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU8, Ordering};

use nonos_libc::mk_args;

use super::launch::Launch;
use super::origin::Origin;
use crate::linux::file::{key, store_read, visible};
use crate::linux::guest::Links;
use crate::linux::start::say;

const MAX_ARGS: usize = 1024;

/// 0 not yet looked, 1 not started by the terminal, 2 started by it.
static STARTED: AtomicU8 = AtomicU8::new(0);

/// True when the terminal's `linux` command started this capsule.
pub(super) fn started() -> bool {
    match STARTED.load(Ordering::Relaxed) {
        0 => {
            let mut buf = [0u8; MAX_ARGS];
            let n = mk_args(buf.as_mut_ptr(), buf.len());
            let first = usize::try_from(n)
                .ok()
                .and_then(|n| buf.get(..n))
                .and_then(|got| got.split(|b| *b == 0).find(|p| !p.is_empty()));
            let yes = first == Some(b"linux".as_slice());
            STARTED.store(if yes { 2 } else { 1 }, Ordering::Relaxed);
            yes
        }
        seen => seen == 2,
    }
}

/// None when the terminal did not start this capsule; otherwise what to run,
/// or None inside when there is nothing to run, with the reason said.
pub(super) fn requested(max_image: u32) -> Option<Option<Launch>> {
    let mut buf = [0u8; MAX_ARGS];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    let got = buf.get(..usize::try_from(n).ok()?)?;
    let mut parts = got.split(|b| *b == 0).filter(|p| !p.is_empty());
    if parts.next()? != b"linux" {
        return None;
    }
    let Some(program) = parts.next() else {
        say(b"usage: linux <program> [arguments]\n");
        return Some(None);
    };
    let args = parts.map(<[u8]>::to_vec).collect();
    Some(launch(program, args, max_image))
}

fn launch(program: &[u8], mut args: Vec<Vec<u8>>, max_image: u32) -> Option<Launch> {
    if !super::settle::wait_settled() {
        say(b"[LINUX] the store never settled\n");
        return None;
    }
    let named = match program.first() {
        Some(b'/') => visible(b"/", program),
        _ => visible(b"/bin", program),
    };
    let path = Links::load().follow(named.clone(), true);
    if path != named {
        let typed = named.rsplit(|b| *b == b'/').next().unwrap_or(&named);
        args.insert(0, typed.to_vec());
    }
    match store_read(&key(&path), max_image) {
        Ok(bytes) => Some(Launch { path, bytes, origin: Origin::Store, args }),
        Err(_) => {
            let line = alloc::format!(
                "linux: no program {} in the Linux tree\n",
                alloc::string::String::from_utf8_lossy(&named)
            );
            say(line.as_bytes());
            None
        }
    }
}
