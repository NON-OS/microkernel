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

//! The devkit: writes a development machine's records into RECORDS and the
//! client files that pair with them into CLIENT, from keys made afresh from
//! the OS generator on every run, so the two halves of one run pair with
//! each other and with nothing else. It makes no host key: the machine
//! makes its own at its first boot and writes its enrolment request
//! (host.public, enrol.doc) to /nonos/nsh. Every window opens at this
//! machine's clock, or at the one given in milliseconds since the epoch.
//! Usage: `nsh-devkit RECORDS CLIENT [NOW_MS]`, or
//! `nsh-devkit enrol RECORDS CLIENT PUBLIC DOC LOADER [NOW_MS]`, which also
//! writes the enrolment naming the machine whose host keys are in PUBLIC
//! and whose attestation document is in DOC, LOADER being the loader's
//! PCR 4 digest in hex. One run writes every file, since the enrolment's
//! authority is the one the client files trust.

use nsh_server_proofs::kit::devkit::fresh::fresh;
use nsh_server_proofs::kit::devkit::{
    client, enrolment, records, write, Fixture, Machine, Window, HOST_PUBLIC_LEN,
};
use std::path::Path;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

const USAGE: &str =
    "usage: nsh-devkit RECORDS CLIENT [NOW_MS] | enrol RECORDS CLIENT PUBLIC DOC LOADER [NOW_MS]";

fn wall_ms() -> Option<u64> {
    let since = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    u64::try_from(since.as_millis()).ok()
}

fn fail(why: &str, code: i32) -> ! {
    eprintln!("nsh-devkit: {why}");
    exit(code)
}

fn read(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|e| fail(&format!("{path}: {e}"), 1))
}

/// The enrolment for the machine whose keys are in `public` and whose
/// document is in `doc`.
fn enrol(w: &Window, public: &str, doc: &str, loader: &str) -> Fixture {
    let keys: [u8; HOST_PUBLIC_LEN] = read(public).try_into().unwrap_or_else(|_| {
        fail(&format!("{public}: not {HOST_PUBLIC_LEN} bytes of host keys"), 1)
    });
    let loader = unhex(loader).unwrap_or_else(|| fail("LOADER is 64 hex digits", 2));
    let m =
        Machine::from_doc(&read(doc), &loader).unwrap_or_else(|e| fail(&format!("{doc}: {e}"), 1));
    enrolment(w, &m, &keys)
}

fn unhex(s: &str) -> Option<[u8; 32]> {
    let b = s.as_bytes();
    if b.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (o, pair) in out.iter_mut().zip(b.chunks_exact(2)) {
        let digit = |c: u8| char::from(c).to_digit(16);
        *o = u8::try_from(digit(pair[0])? * 16 + digit(pair[1])?).ok()?;
    }
    Some(out)
}

fn window(now: Option<u64>) -> Window {
    now.and_then(Window::at).unwrap_or_else(|| fail("no clock to open the windows at", 2))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !fresh() {
        fail("the OS generator would not give a key space", 1);
    }
    let (dirs, machine, now) = match args.as_slice() {
        [e, r, c, p, d, l] if e == "enrol" => ((r, c), Some((p, d, l)), wall_ms()),
        [e, r, c, p, d, l, n] if e == "enrol" => ((r, c), Some((p, d, l)), n.parse().ok()),
        [r, c] if r != "enrol" => ((r, c), None, wall_ms()),
        [r, c, n] if r != "enrol" => ((r, c), None, n.parse().ok()),
        _ => fail(USAGE, 2),
    };
    let w = window(now);
    let mut machine_files = records(&w);
    if let Some((public, doc, loader)) = machine {
        machine_files.push(enrol(&w, public, doc, loader));
    }
    put(dirs.0, &machine_files);
    put(dirs.1, &client(&w));
    eprintln!(
        "nsh-devkit: revocation list fresh until {} ms; grant from {} ms until {} ms",
        w.list_stale_ms(),
        w.opens_ms,
        w.grant_ends_ms
    );
}

fn put(dir: &str, fixtures: &[Fixture]) {
    if let Err(e) = write(Path::new(dir), fixtures) {
        fail(&format!("{dir}: {e}"), 1);
    }
    for f in fixtures {
        eprintln!("nsh-devkit: wrote {} ({} bytes)", f.name, f.bytes.len());
    }
}
