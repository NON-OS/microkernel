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

//! What a probed document says, for the operator who enrols the machine and
//! writes its accepted release: the attestation key and PCRs 0, 1, 2 and 7
//! the enrolment binds, the applications PCR 4 records, PCRs 8 and 9, and
//! the registry's Network holders. nsh.front is the one holding exactly
//! `FRONT_CAPS`, with or without `DEBUG`, which a development twin's kernel
//! adds for its serial line; the line it suggests names it and the others as the
//! network stack. Nothing here is checked: the probe's document is unproven
//! until the client checks it against the enrolment and release made from
//! it.

use super::report::hex as to_hex;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use nsh_attest::log::replay;
use nsh_attest::steps::NETWORK;
use nsh_attest::Doc;

/// nsh.front's manifest: CoreExec, Network, IPC and Memory.
pub const FRONT_CAPS: u64 = 0x1D;
/// `Capability::Debug`, granted to nsh.front only by a development twin.
pub const DEBUG: u64 = 1 << 8;
const ENTRY_LEN: usize = 45;

pub struct Found {
    pub lines: Vec<String>,
    /// The suggested `--releases` line, if the loader is known (given, or
    /// the one application PCR 4 records) and exactly one entry holds
    /// nsh.front's capabilities.
    pub release: Option<String>,
}

pub fn findings(
    doc: &[u8],
    entries: &[u8],
    version: &str,
    loader: Option<[u8; 32]>,
) -> Result<Found, String> {
    let d = Doc::parse(doc).map_err(|e| format!("the document does not parse: {e:?}"))?;
    let apps =
        replay(d.log).map_err(|e| format!("the event log does not replay: {e:?}"))?.applications;
    if !entries.len().is_multiple_of(ENTRY_LEN) {
        return Err("the registry entries are not whole".into());
    }
    let pcr = |i| d.pcr(i).map_or_else(|| "-".into(), |v| to_hex(v));
    let mut lines = alloc::vec![format!("ak {}", to_hex(d.ak_public))];
    lines.push(format!("pcrs {}{}{}{}", pcr(0), pcr(1), pcr(2), pcr(7)));
    lines.extend(apps.iter().map(|a| format!("pcr4-application {}", to_hex(a))));
    lines.push(format!("pcr8 {}", pcr(8)));
    lines.push(format!("pcr9 {}", pcr(9)));
    let net: Vec<(&[u8], u64)> =
        entries.chunks_exact(ENTRY_LEN).map(caps).filter(|(_, c)| c & NETWORK != 0).collect();
    lines.extend(net.iter().map(|(m, c)| format!("network {} caps {c:#x}", to_hex(m))));
    let loader = loader.or(match apps[..] {
        [one] => Some(one),
        _ => None,
    });
    let release = loader.and_then(|l| suggest(version, &pcr(8), &pcr(9), &l, &net));
    Ok(Found { lines, release })
}

fn caps(e: &[u8]) -> (&[u8], u64) {
    let mut c = [0u8; 8];
    c.copy_from_slice(&e[36..44]);
    (&e[4..36], u64::from_be_bytes(c))
}

fn suggest(
    version: &str,
    p8: &str,
    p9: &str,
    loader: &[u8; 32],
    net: &[(&[u8], u64)],
) -> Option<String> {
    let fronts: Vec<&[u8]> =
        net.iter().filter(|(_, c)| (*c & !DEBUG) == FRONT_CAPS).map(|(m, _)| *m).collect();
    let [front] = fronts[..] else { return None };
    let others: Vec<String> =
        net.iter().filter(|(m, _)| *m != front).map(|(m, _)| to_hex(m)).collect();
    let stack = if others.is_empty() { "-".into() } else { others.join(",") };
    Some(format!("release {version} {p8} {p9} {} {} {stack}", to_hex(loader), to_hex(front)))
}
