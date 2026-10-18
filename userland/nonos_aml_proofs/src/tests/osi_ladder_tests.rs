// NONOS Operating System (AGPL-3.0-or-later)
//! _OSI as firmware calls it: one string argument, and the ladder of Ifs a
//! table climbs at load time to set its idea of the OS year.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::object::Object;
use crate::run::{boot, call, num};

#[test]
fn osi_takes_one_string() {
    let (mut aml, _) = boot(&[], 2);
    let r = call(&mut aml, "\\_OSI", vec![Object::Integer(5)]);
    assert_eq!(r.unwrap_err().kind, FaultKind::Type);
}

#[test]
fn the_firmware_ladder_lands_on_the_newest_release() {
    // As firmware sets its OS year: each If raises OSYS when _OSI says yes.
    let rung = |s: &str, year: u64| {
        if_(&cat(&[&name("\\_OSI"), &string(s)]), &store(&int(year), &name("OSYS")))
    };
    let src = cat(&[
        &name_obj("OSYS", &int(2000)),
        &rung("Windows 2009", 2009),
        &rung("Windows 2015", 2015),
        &rung("Windows 2022", 2022),
        &rung("Windows 2099", 2099),
    ]);
    let (mut aml, _) = boot(&src, 2);
    assert_eq!(num(&call(&mut aml, "\\OSYS", vec![]).unwrap()), 2022);
}
