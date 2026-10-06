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

//! tpm2-tools and openssl as the tests drive them: the registrar's half of
//! enrollment, and the reference the kernel's bytes are held to.

use std::process::Command;

use super::dir::Dir;
use super::swtpm::Swtpm;

/// `cmd` with `args`, against `t` when given; its stdout. A tool that fails
/// fails the test, with what it printed.
pub fn tool(t: Option<&Swtpm>, cmd: &str, args: &[&str]) -> Vec<u8> {
    let mut c = Command::new(cmd);
    c.args(args);
    if let Some(t) = t {
        c.env("TPM2TOOLS_TCTI", t.tcti());
    }
    let out = c.output().unwrap_or_else(|e| panic!("{cmd} did not run: {e}"));
    assert!(out.status.success(), "{cmd} {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// The registrar: `tpm2_makecredential`, offline, to the EK public area for
/// `name`, split into the bodies of TPM2B_ID_OBJECT and TPM2B_ENCRYPTED_SECRET.
pub fn make_credential(dir: &Dir, ek: &[u8], name: &[u8], secret: &[u8]) -> (Vec<u8>, Vec<u8>) {
    std::fs::write(dir.file("mc.ek"), ek).expect("write ek public");
    std::fs::write(dir.file("mc.secret"), secret).expect("write secret");
    let (e, s, o) = (dir.file("mc.ek"), dir.file("mc.secret"), dir.file("mc.cred"));
    let n = hex(name);
    tool(None, "tpm2_makecredential", &["-T", "none", "-e", &e, "-s", &s, "-n", &n, "-o", &o]);
    let cred = std::fs::read(&o).expect("read credential");
    /* tpm2-tools' file header: magic 0xBADCC0DE, version 1 */
    assert_eq!(cred[..8], [0xBA, 0xDC, 0xC0, 0xDE, 0, 0, 0, 1]);
    let (blob, at) = tpm2b(&cred, 8);
    let (enc, end) = tpm2b(&cred, at);
    assert_eq!(end, cred.len(), "nothing after the encrypted secret");
    (blob.to_vec(), enc.to_vec())
}

fn tpm2b(b: &[u8], at: usize) -> (&[u8], usize) {
    let n = u16::from_be_bytes([b[at], b[at + 1]]) as usize;
    (&b[at + 2..at + 2 + n], at + 2 + n)
}

/// Transient objects, and sessions loaded or saved, that the TPM holds.
pub fn held(t: &Swtpm) -> (usize, usize) {
    let count = |caps: &[&str]| -> usize {
        let list = |c: &&str| tool(Some(t), "tpm2_getcap", &[c]);
        caps.iter().map(|c| String::from_utf8_lossy(&list(c)).matches("- 0x").count()).sum()
    };
    let sessions = count(&["handles-loaded-session", "handles-saved-session"]);
    (count(&["handles-transient"]), sessions)
}
