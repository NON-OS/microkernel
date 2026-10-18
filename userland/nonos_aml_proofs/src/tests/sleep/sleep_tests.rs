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

//! \_Sx read through the interpreter, a Name or a method alike, and
//! \_TTS then \_PTS run before it with the state as their argument.

use crate::enc::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::sleep::{prepare, sleep_types, sx_seg, SleepTypes};
use crate::tests::platform::run::evaluator;

fn types(src: &[u8]) -> Option<SleepTypes> {
    let (mut aml, _) = boot(src, 2);
    sleep_types(&mut aml, &mut evaluator(&mut Probe::default()), 5)
}

#[test]
fn sx_as_a_name_a_method_and_the_old_one_integer_form() {
    let pkg4 = package(4, &[int(5), int(6), int(0), int(0)]);
    assert_eq!(types(&name_obj("\\_S5", &pkg4)), Some(SleepTypes { a: 5, b: 6 }));
    let method_s5 = method("\\_S5", 0, &ret(&package(2, &[int(7), int(0x0F)])));
    assert_eq!(types(&method_s5), Some(SleepTypes { a: 7, b: 7 }), "masked to three bits");
    assert_eq!(
        types(&name_obj("\\_S5", &package(1, &[int(0x0705)]))),
        Some(SleepTypes { a: 5, b: 7 })
    );
    assert_eq!(types(&name_obj("\\_S5", &int(5))), None, "not a package");
    assert_eq!(types(&name_obj("\\_S3", &pkg4)), None, "no \\_S5");
    assert_eq!(sx_seg(5), *b"_S5_");
}

#[test]
fn tts_then_pts_then_sx() {
    let debug = |v: &[u8]| store(v, &op(0x5B31));
    let src = cat(&[
        &method("\\_TTS", 1, &debug(&e(0x72, &[&arg(0), &int(0x10), &[0]]))),
        &method("\\_PTS", 1, &debug(&arg(0))),
        &name_obj("\\_S5", &package(2, &[int(5), int(5)])),
    ]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    let p = prepare(&mut aml, &mut evaluator(&mut probe), 5);
    assert_eq!(
        (p.tts, p.pts, p.types),
        (Some(Ok(())), Some(Ok(())), Some(SleepTypes { a: 5, b: 5 }))
    );
    assert_eq!(probe.debug, ["Integer(21)", "Integer(5)"]);
}

#[test]
fn a_failed_pts_still_reads_sx_and_absent_methods_are_none() {
    let fatal = cat(&[&op(0x5B32), &[1], &7u32.to_le_bytes(), &int(0)]);
    let src =
        cat(&[&method("\\_PTS", 1, &fatal), &name_obj("\\_S5", &package(2, &[int(5), int(0)]))]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    let p = prepare(&mut aml, &mut evaluator(&mut probe), 5);
    assert!(p.tts.is_none());
    assert!(matches!(p.pts, Some(Err(_))));
    assert_eq!(p.types, Some(SleepTypes { a: 5, b: 0 }));
}
