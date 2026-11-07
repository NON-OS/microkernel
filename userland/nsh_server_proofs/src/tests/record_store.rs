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

//! The record as a host keeps it on disk: byte for byte the record the
//! client received, its checkpoints beside it, each behind the bytes it
//! covers, so a session a crash cut short checks to its last checkpoint.
//! A session's files are owner-only and never written over.

use super::good::{check, received};
use super::record::run_with;
use crate::kit::host::{config, Keys};
use crate::kit::run::Run;
use nsh_client::Verdict;
use nsh_record_store::{Keeper, Store, CKPT_LEN};
use nsh_server::{Config, EndReason, Event as S};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn dir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("nsh-store-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

/// A session of five commands, checkpointed every two entries.
fn session(exit: bool) -> Run {
    let mut run = run_with(Config { ckpt_every: 2, ..config() }, Keys::default());
    for n in 0..5u8 {
        run.host(|s, now, r| s.command(&[b'a' + n], now, r)).expect("command");
    }
    if exit {
        run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    }
    run
}

/// The id the session started under.
fn sid(run: &Run) -> [u8; 16] {
    let id = run.s_events.iter().find_map(|e| match e {
        S::Started { session_id, .. } => Some(*session_id),
        _ => None,
    });
    id.expect("a session")
}

fn keep_all(store: &Store, events: &[S]) {
    let mut k = Keeper::new(store);
    events.iter().for_each(|e| k.keep(e).expect("kept"));
}

fn kept(store: &Store, run: &Run) -> (Vec<u8>, Vec<u8>) {
    let (rec, ck) = store.paths(&sid(run));
    (std::fs::read(rec).expect("the record"), std::fs::read(ck).expect("the checkpoints"))
}

#[test]
fn the_kept_record_is_the_one_the_client_received() {
    let run = session(true);
    let store = Store::open(dir("whole")).expect("a store");
    keep_all(&store, &run.s_events);
    let (rec, ck) = kept(&store, &run);
    assert_eq!(rec, received(&run));
    let ckpts: Vec<&[u8]> = ck.chunks(CKPT_LEN).collect();
    assert_eq!(ckpts, run.checkpoints().iter().map(Vec::as_slice).collect::<Vec<_>>());
    assert!(matches!(
        check(&run, &rec, &ckpts).expect("checks").verdict,
        Verdict::Complete { entries: 6, .. }
    ));
}

#[test]
fn a_session_cut_short_checks_to_its_last_checkpoint() {
    let run = session(false);
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Closed(EndReason::Exit))));
    let store = Store::open(dir("cut")).expect("a store");
    keep_all(&store, &run.s_events);
    let (rec, ck) = kept(&store, &run);
    let ckpts: Vec<&[u8]> = ck.chunks(CKPT_LEN).collect();
    assert_eq!(ckpts.len(), 2, "after entries 2 and 4");
    let got = check(&run, &rec, &ckpts).expect("a record without its end checks to its checkpoint");
    assert_eq!(got.verdict, Verdict::CutShort { seq: 3, boxed: false });
    let torn = &rec[..rec.len() - 3];
    let got = check(&run, torn, &ckpts).expect("a torn last entry is behind the checkpoint");
    assert_eq!(got.verdict, Verdict::CutShort { seq: 3, boxed: false });
}

#[test]
fn every_checkpoint_follows_the_bytes_it_covers() {
    let run = session(true);
    let store = Store::open(dir("order")).expect("a store");
    let (path, _) = store.paths(&sid(&run));
    let mut k = Keeper::new(&store);
    let mut grown = Vec::new();
    for e in &run.s_events {
        k.keep(e).expect("kept");
        match e {
            S::Record(b) => grown.extend_from_slice(b),
            S::Checkpoint(_) => assert_eq!(std::fs::read(&path).expect("the record"), grown),
            _ => {}
        }
    }
}

#[test]
fn a_session_is_never_written_over() {
    let run = session(true);
    let store = Store::open(dir("twice")).expect("a store");
    keep_all(&store, &run.s_events);
    let before = kept(&store, &run);
    let mut again = Keeper::new(&store);
    let started = run.s_events.iter().find(|e| matches!(e, S::Started { .. })).expect("started");
    let err = again.keep(started).expect_err("the same id twice");
    assert_eq!(err.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(kept(&store, &run), before);
}

#[test]
fn the_files_are_owner_only() {
    let run = session(true);
    let store = Store::open(dir("modes")).expect("a store");
    keep_all(&store, &run.s_events);
    let mode =
        |p: &std::path::Path| std::fs::metadata(p).expect("a file").permissions().mode() & 0o777;
    let (rec, ck) = store.paths(&sid(&run));
    assert_eq!((mode(store.dir()), mode(&rec), mode(&ck)), (0o700, 0o600, 0o600));
}

#[test]
fn a_directory_others_can_reach_is_refused() {
    let d = dir("open");
    std::fs::create_dir_all(&d).expect("a directory");
    std::fs::set_permissions(&d, std::fs::Permissions::from_mode(0o750)).expect("a mode");
    let err = Store::open(&d).err().expect("refused");
    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
    let link = dir("link");
    std::os::unix::fs::symlink(&d, &link).expect("a link");
    assert!(Store::open(&link).is_err(), "a link is not the directory");
}

#[test]
fn a_misplaced_or_misshapen_checkpoint_is_refused() {
    let run = session(true);
    let store = Store::open(dir("bad")).expect("a store");
    let ckpt = run.checkpoints().remove(0);
    let mut early = Keeper::new(&store);
    assert!(early.keep(&S::Checkpoint(ckpt.clone())).is_err(), "before the session");
    let mut k = Keeper::new(&store);
    for e in run.s_events.iter().take_while(|e| !matches!(e, S::Checkpoint(_))) {
        k.keep(e).expect("kept");
    }
    assert!(k.keep(&S::Checkpoint(ckpt[..CKPT_LEN - 1].to_vec())).is_err());
    let started = run.s_events.iter().find(|e| matches!(e, S::Started { .. })).expect("started");
    assert!(k.keep(started).is_err(), "a second session on one keeper");
}

#[test]
fn the_store_and_the_checker_agree_on_a_checkpoint() {
    assert_eq!(CKPT_LEN, nsh_client::CKPT_LEN);
}

#[test]
fn no_more_than_a_header_is_held_before_the_session() {
    let store = Store::open(dir("held")).expect("a store");
    let mut k = Keeper::new(&store);
    let half = nsh_server::RECORD_HEADER_MAX / 2;
    k.keep(&S::Record(vec![0; half])).expect("half a header");
    k.keep(&S::Record(vec![0; nsh_server::RECORD_HEADER_MAX - half])).expect("all of a header");
    assert!(k.keep(&S::Record(vec![0])).is_err(), "one byte past the largest header");
    let mut fresh = Keeper::new(&store);
    assert!(fresh.keep(&S::Record(vec![0; nsh_server::RECORD_HEADER_MAX + 1])).is_err());
}

#[test]
fn every_header_the_server_sends_fits() {
    let run = session(true);
    let first = run.s_events.iter().find_map(|e| match e {
        S::Record(b) => Some(b.len()),
        _ => None,
    });
    assert!(first.expect("a header") <= nsh_server::RECORD_HEADER_MAX);
}
