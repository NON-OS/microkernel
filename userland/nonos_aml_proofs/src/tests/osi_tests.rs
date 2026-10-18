// NONOS Operating System (AGPL-3.0-or-later)
//! _OSI: every Windows version string Microsoft publishes, oldest to newest,
//! is answered; nothing else is; and the ladder firmware climbs at load time
//! ends on the newest.

use crate::enc::*;
use crate::exec::{windows, ExecLimits, Machine, Personality};
use crate::load::Aml;
use crate::object::Object;
use crate::probe::Probe;
use crate::run::num;

#[path = "osi_table_tests.rs"]
mod table;

#[path = "osi_sweep_tests.rs"]
mod sweep;

#[path = "opcode_table_tests.rs"]
mod opcodes;

fn osi(p: Personality, s: &[u8]) -> u64 {
    let mut aml = Aml::default();
    aml.rt.personality = p;
    let mut probe = Probe::default();
    Machine::new(&mut aml, &mut probe, ExecLimits::DEFAULT)
        .load_table(&mut aml, table(b"DSDT", 2, &[]))
        .unwrap();
    let id = crate::consts::at(&aml, "\\_OSI").unwrap();
    num(&Machine::new(&mut aml, &mut probe, ExecLimits::DEFAULT)
        .evaluate(&mut aml, id, vec![Object::string(s)])
        .unwrap())
}

#[test]
fn every_published_windows_string_is_answered() {
    assert_eq!(windows().count(), 23);
    for s in windows() {
        assert_eq!(osi(Personality::DEFAULT, s), u64::MAX, "{}", String::from_utf8_lossy(s));
    }
}

#[test]
fn nothing_else_is() {
    for s in [
        &b"Linux"[..],
        b"Module Device",
        b"Processor Device",
        b"3.0 Thermal Model",
        b"Windows 2099",
        b"windows 2015",
        b"Windows 2015 ",
        b"",
    ] {
        assert_eq!(osi(Personality::DEFAULT, s), 0, "{}", String::from_utf8_lossy(s));
    }
}

#[test]
fn the_answers_can_be_capped_at_a_release() {
    let p = Personality::windows_up_to(b"Windows 2015");
    assert_eq!(osi(p, b"Windows 2015"), u64::MAX);
    assert_eq!(osi(p, b"Windows 2009"), u64::MAX);
    assert_eq!(osi(p, b"Windows 2016"), 0);
    assert_eq!(osi(Personality::windows_up_to(b"no such string"), b"Windows 2000"), 0);
}
