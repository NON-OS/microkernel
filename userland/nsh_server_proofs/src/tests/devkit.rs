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

//! The devkit's files: the machine's records as /nonos/nsh holds them, no
//! host key among them, an enrolment that names the host keys the machine
//! made, a role table that vouches at the clock the files were made at and
//! stops a day later, the client files the loopback proofs hand `nsh`, and
//! secrets written readable by their owner alone.

use super::devkit_host::{file, made_at, public, roles};
use crate::kit::devkit::{client, write, Window};
use crate::kit::roles_build::HOUR;
use nsh_server::Roles;
use nsh_wire::Enrol;
use std::os::unix::fs::PermissionsExt;

/// November 2026: a clock other than the role table's.
const AT: u64 = 1_793_000_000_000;

#[test]
fn devkit_machine_records_are_the_ones_nsh_attest_reads_and_hold_no_secret() {
    let (_, files) = made_at(AT);
    let names: Vec<_> = files.iter().map(|f| f.name).collect();
    assert_eq!(names, ["roles", "roles.anchor", "revocations", "enrol"]);
    assert_eq!(file(&files, "roles.anchor").len(), 32);
    assert!(files.iter().all(|f| !f.secret && !f.bytes.is_empty()), "no host secret");
    assert!(file(&files, "roles").len() <= nsh_front_wire::TABLE_MAX);
    assert!(file(&files, "revocations").len() <= nsh_front_wire::LIST_MAX);
    assert!(file(&files, "enrol").len() <= nsh_front_wire::ENROL_MAX);
}

#[test]
fn devkit_enrolment_names_the_host_keys_the_machine_made() {
    let (_, files) = made_at(AT);
    let enrol = Enrol::parse(file(&files, "enrol")).expect("an nsh-enrol/1 certificate");
    let keys = public();
    assert_eq!(enrol.host_keys, &nsh_crypto::key_digest(&keys[..32], &keys[32..]));
}

#[test]
fn devkit_role_table_vouches_from_its_clock_for_a_day() {
    let (w, files) = made_at(AT);
    let store = roles(&files);
    let fresh = |t: u64| store.with(|p, r| r.fresh(t, p.revocation_max_age_ms)).expect("vouched");
    assert!(fresh(AT), "at the clock they were made at");
    assert!(fresh(AT - HOUR / 2), "a guest clock a little behind");
    assert!(fresh(w.list_stale_ms()), "the last fresh moment");
    assert!(!fresh(w.list_stale_ms() + 1), "a day on, the list is stale");
}

#[test]
fn devkit_windows_refuse_a_clock_they_cannot_open_at() {
    assert_eq!(Window::at(HOUR - 1), None);
    assert_eq!(Window::at(u64::MAX), None);
    let w = Window::at(AT).expect("a window");
    assert!(w.opens_ms < AT && AT < w.grant_ends_ms && w.grant_ends_ms < w.enrol_ends_ms);
}

#[test]
fn devkit_client_files_are_the_ones_nsh_reads() {
    let w = Window::at(AT).expect("a window");
    let (dir, _) =
        super::cli_files::files("devkit", "127.0.0.1:9", &crate::kit::devkit::make::grant(&w));
    for f in client(&w) {
        assert_eq!(std::fs::read(dir.join(f.name)).expect(f.name), f.bytes, "{}", f.name);
    }
}

#[test]
fn devkit_writes_secrets_readable_by_their_owner_alone() {
    let dir = std::env::temp_dir().join(format!("nsh-devkit-mode-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a directory");
    std::fs::write(dir.join("user.secret"), b"left open").expect("a stale file");
    std::fs::set_permissions(dir.join("user.secret"), PermissionsExt::from_mode(0o644))
        .expect("open");
    let files = client(&Window::at(AT).expect("a window"));
    assert_eq!(
        files.iter().filter(|f| f.secret).map(|f| f.name).collect::<Vec<_>>(),
        ["user.secret"]
    );
    write(&dir, &files).expect("written");
    for f in &files {
        let meta = std::fs::metadata(dir.join(f.name)).expect(f.name);
        let want = if f.secret { 0o600 } else { 0o644 };
        assert_eq!(meta.permissions().mode() & 0o777, want, "{}", f.name);
        assert_eq!(std::fs::read(dir.join(f.name)).expect(f.name), f.bytes);
    }
}

/// A symbolic link at a fixture's name is replaced, never written through.
#[test]
fn devkit_never_writes_through_a_link() {
    let dir = std::env::temp_dir().join(format!("nsh-devkit-link-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a directory");
    let target = dir.join("elsewhere");
    std::fs::write(&target, b"untouched").expect("a target");
    std::os::unix::fs::symlink(&target, dir.join("user.secret")).expect("a link");
    write(&dir, &client(&Window::at(AT).expect("a window"))).expect("written");
    assert_eq!(std::fs::read(&target).expect("the target"), b"untouched");
    let at = dir.join("user.secret");
    let meta = std::fs::symlink_metadata(&at).expect("the secret");
    assert!(meta.file_type().is_file(), "a file, not the link");
    assert_eq!(meta.permissions().mode() & 0o777, 0o600);
}
