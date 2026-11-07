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

//! `nsh-probe`: dial a host, take its HOST_PROOF, and stop before any
//! credential is sent. The client checks the host's signature and its
//! enrolment as `nsh` does; the attester keeps the document and entries
//! and refuses them, so the session aborts before USER_AUTH. What it keeps
//! is written to `--out` (`doc`, `entries`, `found`, and `releases`
//! when one line can be suggested; `--loader` names the loader when PCR 4
//! records more than one application) and printed: the inputs for enrolling a
//! live machine and writing its accepted release. They are unproven until
//! `nsh` checks the machine against what is made from them.

use super::probe_keep::Keep;
use super::probe_show::findings;
use super::{args::hex, files, urandom};
use crate::{Client, Config, Policy};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::time::{Duration, Instant};

pub const USAGE: &str =
    "usage: nsh-probe --connect HOST:PORT --host NAME --fleet HEX --authority FILE \
--key FILE --grant FILE --out DIR [--version NAME] [--loader HEX]";

struct Probe {
    connect: String,
    host: String,
    fleet: [u8; 16],
    authority: String,
    key: String,
    grant: String,
    out: String,
    version: String,
    loader: Option<[u8; 32]>,
}

pub fn probe(argv: &[String], out: &mut dyn Write) -> i32 {
    let Some(p) = parse(argv) else {
        let _ = writeln!(out, "{USAGE}");
        return 2;
    };
    match run(&p) {
        Ok(lines) => {
            lines.iter().for_each(|l| drop(writeln!(out, "{l}")));
            0
        }
        Err(why) => {
            let _ = writeln!(out, "nsh-probe: {why}");
            1
        }
    }
}

fn parse(argv: &[String]) -> Option<Probe> {
    let mut got: Vec<(&str, &str)> = Vec::new();
    let mut rest = argv;
    while let [n, v, tail @ ..] = rest {
        got.push((n.as_str(), v.as_str()));
        rest = tail;
    }
    let one = |k: &str| {
        let mut it = got.iter().filter(|(n, _)| *n == k);
        let v = it.next()?.1.to_string();
        it.next().is_none().then_some(v)
    };
    let known = [
        "--connect",
        "--host",
        "--fleet",
        "--authority",
        "--key",
        "--grant",
        "--out",
        "--version",
        "--loader",
    ];
    if !rest.is_empty() || got.iter().any(|(n, _)| !known.contains(n)) {
        return None;
    }
    Some(Probe {
        connect: one("--connect")?,
        host: one("--host")?.to_ascii_lowercase(),
        fleet: hex(&one("--fleet")?)?,
        authority: one("--authority")?,
        key: one("--key")?,
        grant: one("--grant")?,
        out: one("--out")?,
        version: one("--version").unwrap_or_else(|| "dev".into()),
        loader: match one("--loader") {
            Some(l) => Some(hex(&l)?),
            None => None,
        },
    })
}

fn run(p: &Probe) -> Result<Vec<String>, String> {
    let (doc, entries) = take_proof(p)?;
    let found = findings(&doc, &entries, &p.version, p.loader)?;
    let dir = Path::new(&p.out);
    std::fs::create_dir_all(dir).map_err(|e| alloc::format!("{}: {e}", p.out))?;
    let mut found_text = found.lines.join("\n");
    found_text.push('\n');
    let mut files =
        alloc::vec![("doc", doc), ("entries", entries), ("found", found_text.into_bytes())];
    if let Some(r) = &found.release {
        files.push(("releases", alloc::format!("{r}\n").into_bytes()));
    }
    for (name, bytes) in &files {
        keep(&dir.join(name), bytes).map_err(|e| alloc::format!("{name}: {e}"))?;
    }
    let mut lines = found.lines;
    lines.push(found.release.unwrap_or_else(|| "no single release line could be suggested".into()));
    Ok(lines)
}

/// Dial, run the handshake to HOST_PROOF, and return what the attester kept.
fn take_proof(p: &Probe) -> Result<(Vec<u8>, Vec<u8>), String> {
    let policy = Policy {
        fleet_id: p.fleet,
        authorities: alloc::vec![files::authority(&p.authority)?],
        revoked: Vec::new(),
    };
    let now =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_| "clock")?;
    let cfg = Config {
        host: p.host.as_bytes().to_vec(),
        now_ms: now.as_millis() as u64,
        policy,
        grant: files::read(&p.grant)?,
    };
    let (mut seed, mut random) = ([0u8; 32], [0u8; 32]);
    if !urandom(&mut seed) || !urandom(&mut random) {
        return Err("no randomness".into());
    }
    let user = files::user_key(&p.key)?;
    let mut c =
        Client::new(cfg, Keep::default(), user, nsh_crypto::Secret::take(&mut seed), random)
            .map_err(|e| alloc::format!("refused before connecting: {e:?}"))?;
    let addr =
        p.connect.to_socket_addrs().map_err(|e| alloc::format!("{}: {e}", p.connect))?.next();
    let addr = addr.ok_or_else(|| alloc::format!("{}: no address", p.connect))?;
    let mut sock = TcpStream::connect_timeout(&addr, Duration::from_secs(10))
        .map_err(|e| alloc::format!("{addr}: {e}"))?;
    sock.set_read_timeout(Some(Duration::from_millis(50))).map_err(|e| e.to_string())?;
    let deadline = Instant::now() + PROOF_WAIT;
    let mut buf = alloc::vec![0u8; 65_536];
    let mut closed = false;
    while !c.is_over() && Instant::now() < deadline {
        if let Err(e) = sock.write_all(&c.take_output()) {
            if !hung_up(&e) {
                return Err(e.to_string());
            }
            closed = true;
            break;
        }
        match sock.read(&mut buf) {
            Ok(0) => {
                closed = true;
                break;
            }
            Ok(n) => {
                let fed = c.feed(&buf[..n], &mut urandom);
                if let Err(why) = fed {
                    if c.attester().seen.borrow().is_none() {
                        return Err(alloc::format!(
                            "the host was refused before its attestation: {why:?}"
                        ));
                    }
                }
            }
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(e) if hung_up(&e) => {
                closed = true;
                break;
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    let _ = sock.write_all(&c.take_output());
    let kept = c.attester().seen.borrow_mut().take();
    kept.ok_or_else(|| if closed { CLOSED.into() } else { SILENT.into() })
}

/// The peer closed or reset the connection.
fn hung_up(e: &std::io::Error) -> bool {
    matches!(
        e.kind(),
        ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::BrokenPipe
    )
}

/// How long the host has, from the connection, to send its HOST_PROOF.
const PROOF_WAIT: Duration = Duration::from_secs(60);
/// The host hung up before its HOST_PROOF.
pub const CLOSED: &str = "the host closed the connection before its HOST_PROOF; check that it \
     serves nsh/1 there: a NONOS guest's nsh.front listens only while its network route is \
     Direct, and a QEMU port forward takes the connection even when nothing in the guest listens";
/// The host kept the connection and sent no HOST_PROOF in PROOF_WAIT.
pub const SILENT: &str = "the host sent no HOST_PROOF in 60 s";

/// Owner-only, as the devkit writes, never through a link at the name: a
/// probe run again replaces what the last one kept.
fn keep(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    match std::fs::remove_file(path) {
        Err(e) if e.kind() != ErrorKind::NotFound => return Err(e),
        _ => {}
    }
    files::write_private(path, bytes)
}
