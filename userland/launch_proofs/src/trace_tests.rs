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

use crate::launch_trace::line::{label, render, Text, LINE_MAX};
use crate::launch_trace::phase::Phase;
use crate::launch_trace::record::{Record, NAME_MAX, ORDER, PHASES};
use crate::launch_trace::table::Table;

fn line(r: &Record, end: &[u8]) -> String {
    let mut t = Text::new();
    render(r, end, &mut t);
    String::from_utf8(t.as_bytes().to_vec()).unwrap()
}

#[test]
fn order_lists_every_phase_once_by_discriminant() {
    assert_eq!(ORDER.len(), PHASES);
    for (i, p) in ORDER.iter().enumerate() {
        assert_eq!(*p as usize, i);
    }
}

#[test]
fn labels_are_distinct_and_have_no_spaces() {
    for a in ORDER {
        assert!(!label(a).contains(&b' ') && !label(a).contains(&b'='));
        for b in ORDER {
            if a != b {
                assert_ne!(label(a), label(b));
            }
        }
    }
}

#[test]
fn spans_measure_from_the_previous_phase_reached() {
    let mut r = Record::begin(b"app.clock", 1_000_000);
    r.mark(Phase::CertVerified, 1_400_000);
    r.mark(Phase::ManifestVerified, 1_900_000);
    // No attestation: the publisher tier skips it.
    r.mark(Phase::ProcessCreated, 2_000_000);
    assert_eq!(r.span_ns(Phase::CertVerified), Some(400_000));
    assert_eq!(r.span_ns(Phase::ManifestVerified), Some(500_000));
    assert_eq!(r.span_ns(Phase::Attested), None);
    assert_eq!(r.span_ns(Phase::ProcessCreated), Some(100_000));
    assert_eq!(r.span_ns(Phase::Begin), None);
    assert_eq!(r.total_ns(), 1_000_000);
}

#[test]
fn spans_sum_to_the_total() {
    let mut r = Record::begin(b"x", 7);
    let mut t = 7u64;
    for (i, p) in ORDER.iter().enumerate().skip(1) {
        t += (i as u64) * 1_337;
        if i % 3 != 0 {
            r.mark(*p, t);
        }
    }
    let sum: u64 = ORDER.iter().filter_map(|p| r.span_ns(*p)).sum();
    assert_eq!(sum, r.total_ns());
}

#[test]
fn a_phase_keeps_its_first_mark() {
    let mut r = Record::begin(b"x", 0);
    r.mark(Phase::FirstSyscall, 50);
    r.mark(Phase::FirstSyscall, 90);
    assert_eq!(r.span_ns(Phase::FirstSyscall), Some(50));
}

#[test]
fn a_clock_that_starts_at_zero_still_counts_as_reached() {
    let r = Record::begin(b"x", 0);
    assert!(r.reached(Phase::Begin));
    assert_eq!(r.total_ns(), 0);
}

#[test]
fn a_backward_clock_reads_zero_not_a_wrapped_span() {
    let mut r = Record::begin(b"x", 1_000);
    r.mark(Phase::CertVerified, 10);
    assert_eq!(r.span_ns(Phase::CertVerified), Some(0));
    assert_eq!(r.total_ns(), 0);
}

#[test]
fn long_names_are_cut_to_the_record() {
    let name = [b'a'; NAME_MAX + 9];
    let r = Record::begin(&name, 0);
    assert_eq!(r.name().len(), NAME_MAX);
}

#[test]
fn the_line_names_every_phase_in_order_in_microseconds() {
    let mut r = Record::begin(b"app.clock", 0);
    r.set_pid(12);
    r.mark(Phase::CertVerified, 410_000);
    r.mark(Phase::ManifestVerified, 798_000);
    r.mark(Phase::ProcessCreated, 893_999);
    r.mark(Phase::FirstFrame, 9_000_000);
    assert_eq!(
        line(&r, b"frame"),
        "[LAUNCH] app.clock pid=12 cert=410 manifest=388 attest=- create=95 elf=- \
         install=- queue=- syscall=- surface=- frame=8106 total=9000 end=frame"
    );
}

#[test]
fn the_longest_line_fits_the_buffer() {
    let name = [b'z'; NAME_MAX];
    let mut r = Record::begin(&name, 0);
    r.set_pid(u32::MAX);
    for (i, p) in ORDER.iter().enumerate().skip(1) {
        r.mark(*p, if i == ORDER.len() - 1 { u64::MAX } else { (i as u64) * (u64::MAX / 11) });
    }
    let text = line(&r, b"displaced");
    assert!(text.len() < LINE_MAX, "{} bytes", text.len());
    assert!(text.ends_with("end=displaced"));
}

#[test]
fn the_table_completes_a_launch_on_its_first_frame() {
    let mut t: Table<4> = Table::new();
    let mut r = Record::begin(b"a", 0);
    r.set_pid(5);
    assert!(t.insert(r).is_none());
    assert!(t.mark(5, Phase::FirstSyscall, 10).is_none());
    assert!(t.mark(5, Phase::FirstSurface, 20).is_none());
    let done = t.mark(5, Phase::FirstFrame, 30).expect("complete");
    assert_eq!(done.span_ns(Phase::FirstFrame), Some(10));
    assert_eq!(t.len(), 0);
    assert!(t.mark(5, Phase::FirstFrame, 40).is_none());
}

#[test]
fn the_table_ignores_pids_it_does_not_hold() {
    let mut t: Table<2> = Table::new();
    assert!(t.mark(9, Phase::FirstFrame, 1).is_none());
    assert!(t.take(9).is_none());
}

#[test]
fn a_full_table_hands_back_the_launch_it_displaces() {
    let mut t: Table<2> = Table::new();
    let mut displaced = Vec::new();
    for pid in 1..=5u32 {
        let mut r = Record::begin(b"x", 0);
        r.set_pid(pid);
        if let Some(old) = t.insert(r) {
            displaced.push(old.pid());
        }
    }
    assert_eq!(t.len(), 2);
    assert_eq!(displaced.len(), 3);
    let mut held: Vec<u32> = (1..=5).filter(|p| !displaced.contains(p)).collect();
    held.sort();
    for pid in held {
        assert!(t.take(pid).is_some());
    }
}

#[test]
fn a_reused_pid_replaces_its_old_launch() {
    let mut t: Table<4> = Table::new();
    let mut a = Record::begin(b"old", 0);
    a.set_pid(3);
    let mut b = Record::begin(b"new", 0);
    b.set_pid(3);
    assert!(t.insert(a).is_none());
    let old = t.insert(b).expect("old launch returned");
    assert_eq!(old.name(), b"old");
    assert_eq!(t.take(3).map(|r| r.name().to_vec()), Some(b"new".to_vec()));
}

#[test]
fn an_exit_takes_the_launch_out_of_the_table() {
    let mut t: Table<4> = Table::new();
    let mut r = Record::begin(b"x", 0);
    r.set_pid(8);
    t.insert(r);
    assert!(t.take(8).is_some());
    assert_eq!(t.len(), 0);
}
