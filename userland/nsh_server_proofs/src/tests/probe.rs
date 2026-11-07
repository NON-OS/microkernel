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

//! What nsh-probe reads from a host's document is what that document holds:
//! the attestation key and PCRs the enrolment binds, PCR 4's applications,
//! PCRs 8 and 9, the Network holders, and a release line naming the spec's
//! own release when the loader and nsh.front can be told apart.

use crate::kit::attest_build::ak::Key;
use crate::kit::attest_build::parts::{entry_by, release};
use crate::kit::attest_build::spec::Spec;
use crate::kit::attest_build::tcg_log::loader;
use nsh_attest::steps::NETWORK;
use nsh_client::connect::probe_show::{findings, DEBUG, FRONT_CAPS};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// A machine whose registry holds nsh.front (its own capability mask) and
/// one more Network holder.
fn spec() -> Spec {
    Spec {
        entries: vec![
            entry_by(0, [0xFA; 32], 0, 0),
            entry_by(5, [0xF0; 32], FRONT_CAPS, 0),
            entry_by(6, [0xE1; 32], NETWORK | 1 << 3, 0),
        ],
        ..Spec::default()
    }
}

#[test]
fn the_findings_are_the_documents() {
    let s = spec();
    let b = s.build();
    let f = findings(&b.doc, &b.entries, "0.9.4", Some(loader())).expect("findings");
    let l = &s.log.pcrs;
    assert_eq!(f.lines[0], format!("ak {}", hex(&Key::new(1).public())));
    assert_eq!(
        f.lines[1],
        format!("pcrs {}{}{}{}", hex(&l[0]), hex(&l[1]), hex(&l[2]), hex(&l[7]))
    );
    let apps: Vec<&String> =
        f.lines.iter().filter(|l| l.starts_with("pcr4-application ")).collect();
    assert_eq!(apps.len(), 2, "the loader and the boot manager");
    assert!(f.lines.contains(&format!("network {} caps {FRONT_CAPS:#x}", hex(&[0xF0; 32]))));
    let r = release();
    let want = format!(
        "release 0.9.4 {} {} {} {} {}",
        hex(&r.pcr8),
        hex(&r.pcr9),
        hex(&r.loader),
        hex(&r.front),
        hex(&[0xE1; 32])
    );
    assert_eq!(f.release, Some(want));
}

#[test]
fn no_line_is_suggested_without_a_loader_or_one_front() {
    let b = spec().build();
    assert_eq!(
        findings(&b.doc, &b.entries, "v", None).expect("findings").release,
        None,
        "two applications, none named"
    );
    let mut two = spec();
    two.entries.push(entry_by(7, [0xF7; 32], FRONT_CAPS, 0));
    let b = two.build();
    assert_eq!(findings(&b.doc, &b.entries, "v", Some(loader())).expect("findings").release, None);
}

/// A development twin's kernel grants nsh.front Debug for its serial line;
/// it is still the one nsh.front. Any other bit beside its mask is not.
#[test]
fn nsh_front_is_told_with_or_without_debug() {
    let mut dev = spec();
    dev.entries[1] = entry_by(5, [0xF0; 32], FRONT_CAPS | DEBUG, 0);
    let b = dev.build();
    let f = findings(&b.doc, &b.entries, "0.9.4", Some(loader())).expect("findings");
    let want = format!("{} {}", hex(&release().front), hex(&[0xE1; 32]));
    assert!(f.release.expect("a release line").ends_with(&want));

    let mut other = spec();
    other.entries[1] = entry_by(5, [0xF0; 32], FRONT_CAPS | 1 << 31, 0);
    let b = other.build();
    let f = findings(&b.doc, &b.entries, "0.9.4", Some(loader())).expect("findings");
    assert_eq!(f.release, None, "a Network holder with AttestRead is not nsh.front");
}

#[test]
fn a_broken_document_or_registry_is_refused() {
    let b = spec().build();
    assert!(findings(b"not a document", &b.entries, "v", None).is_err());
    assert!(findings(&b.doc, &b.entries[..44], "v", None).is_err());
}

/// Over real TCP, the probe takes the host's proof, writes what it kept, and
/// the host never sees a login: it is aborted before USER_AUTH.
#[test]
fn the_probe_stops_before_any_credential() {
    use super::cli_files::files;
    use super::loopback_host::serve;
    use crate::kit::host::Kernel;
    use crate::kit::roles::Table;
    use crate::kit::roles_build::grant::GrantB;
    use nsh_server::Event as S;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let host = std::thread::spawn(move || serve(&listener, Kernel::honest(), Table::shipped()));
    let (dir, nsh_args) = files("probe", &addr, &GrantB::default());
    let keep = |name: &str| {
        nsh_args
            .iter()
            .position(|a| a == name)
            .map(|i| [nsh_args[i].clone(), nsh_args[i + 1].clone()])
    };
    let mut args: Vec<String> =
        ["--connect", "--host", "--fleet", "--authority", "--key", "--grant"]
            .iter()
            .flat_map(|n| keep(n).expect(n))
            .collect();
    let out_dir = dir.join("probed");
    args.extend([
        "--out".into(),
        out_dir.to_string_lossy().into_owned(),
        "--loader".into(),
        hex(&loader()),
    ]);
    let mut out = Vec::new();
    let status = nsh_client::connect::probe::probe(&args, &mut out);
    let events = host.join().expect("the host");
    let text = String::from_utf8_lossy(&out);
    assert_eq!(status, 0, "{text}");
    assert!(
        !events.iter().any(|e| matches!(e, S::Admit(_) | S::Started { .. })),
        "no login reached the host"
    );
    assert!(text.starts_with(&format!("ak {}", hex(&Key::new(1).public()))), "{text}");
    for f in ["doc", "entries", "found"] {
        use std::os::unix::fs::PermissionsExt;
        let meta = std::fs::symlink_metadata(out_dir.join(f)).expect(f);
        assert!(meta.is_file(), "{f}");
        assert_eq!(meta.permissions().mode() & 0o777, 0o600, "{f}");
    }
}

#[test]
fn the_probe_says_a_host_that_hangs_up_before_its_proof_serves_nothing_there() {
    use super::cli_files::files;
    use crate::kit::roles_build::grant::GrantB;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    // Takes the connection and closes it, as a QEMU forward does when
    // nothing in the guest listens.
    let host = std::thread::spawn(move || drop(listener.accept().expect("accept")));
    let (dir, nsh_args) = files("probe-hangs-up", &addr, &GrantB::default());
    let keep = |name: &str| {
        nsh_args
            .iter()
            .position(|a| a == name)
            .map(|i| [nsh_args[i].clone(), nsh_args[i + 1].clone()])
    };
    let mut args: Vec<String> =
        ["--connect", "--host", "--fleet", "--authority", "--key", "--grant"]
            .iter()
            .flat_map(|n| keep(n).expect(n))
            .collect();
    args.extend(["--out".into(), dir.join("probed").to_string_lossy().into_owned()]);
    let mut out = Vec::new();
    let status = nsh_client::connect::probe::probe(&args, &mut out);
    host.join().expect("the host");
    let text = String::from_utf8_lossy(&out);
    assert_eq!(status, 1, "{text}");
    assert_eq!(text, format!("nsh-probe: {}\n", nsh_client::connect::probe::CLOSED));
    assert!(!dir.join("probed").join("doc").exists(), "nothing kept");
}
