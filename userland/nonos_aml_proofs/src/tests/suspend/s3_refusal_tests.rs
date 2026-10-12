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

//! S3 refused at each point it can be: no \_S3 runs nothing at all; a held
//! Global Lock or hardware the kernel has not wired writes no PM1 control
//! and leaves the platform as it found it, every wake device disarmed.

use super::logged::logged;
use super::model::Model;
use super::rig::{laptop, policy, VECTOR};
use crate::rig::Rig;
use crate::suspend::{suspend_s3, HwError, Outcome, SuspendError};

fn attempt(with_s3: bool, hw: &mut Model) -> (Outcome, Vec<String>) {
    let mut r = laptop(with_s3);
    let wake = policy(&r);
    let log = hw.log.clone();
    let Rig { aml, probe } = &mut r;
    let out = suspend_s3(aml, &mut logged(probe, log.clone()), hw, &wake, VECTOR);
    let lines = log.borrow().clone();
    (out, lines)
}

fn ends_as_it_began(lines: &[String], hw: &Model) {
    assert!(!lines.iter().any(|l| l.starts_with("hw sleep")), "no PM1 sleep write");
    assert!(hw.armed.is_empty());
    let tail: Vec<&str> = lines.iter().rev().take(2).map(String::as_str).collect();
    assert_eq!(tail, ["\\_SI_._SST(1)", "\\_TTS(0)"], "back to working");
    assert!(lines.contains(&"\\_WAK(3)".to_string()));
}

#[test]
fn without_s3_nothing_runs() {
    let mut hw = Model::default();
    let (out, lines) = attempt(false, &mut hw);
    assert_eq!(out.error, Some(SuspendError::NoS3));
    assert!(lines.is_empty(), "{lines:?}");
}

#[test]
fn a_held_global_lock_stops_before_any_write() {
    let mut hw = Model { lock_held: true, ..Model::default() };
    let (out, lines) = attempt(true, &mut hw);
    assert_eq!((out.entered, out.error), (false, Some(SuspendError::LockHeld)));
    assert!(!lines.iter().any(|l| l.starts_with("hw vector") || l.starts_with("hw clear")));
    ends_as_it_began(&lines, &hw);
}

#[test]
fn unwired_hardware_at_each_step() {
    for step in ["vector", "clear", "sleep"] {
        let mut hw = Model { unwired: Some(step), ..Model::default() };
        let (out, lines) = attempt(true, &mut hw);
        assert_eq!(out.error, Some(SuspendError::Hardware(HwError::NotWired)), "{step}");
        ends_as_it_began(&lines, &hw);
    }
}
