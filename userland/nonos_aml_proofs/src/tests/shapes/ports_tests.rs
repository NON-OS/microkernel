// NONOS Operating System (AGPL-3.0-or-later)
//! _UPC, _PLD and _BCL under lying shapes. The models (recalled,
//! unverified): _UPC (ACPI 6.5, 9.14) is a package whose first two
//! elements are integers, the connector type within a byte; _PLD (6.1.8)
//! is a buffer, bare or first in a package, revision in bits 6:0, 16 bytes
//! for revision 1 and 20 after it, revision 0 refused, the panel in byte 8
//! bits 5:3 with 6 and 7 unknown; _BCL (B.6.2) keeps the levels after the
//! first two that are integers of at most 100, sorted, each once, and is
//! refused when none is left, so no lie sets a brightness past 100.

use super::gen::{lie, stranger, Rng};
use super::run::{elements, hold, int};
use crate::object::Object;
use crate::ports::{upc, Upc};
use crate::video::{parse_bcl, Levels};

fn upc_model(o: &Object) -> Option<Upc> {
    let e = elements(o)?;
    let connector = u8::try_from(int(&e, 1)?).ok()?;
    Some(Upc { connectable: int(&e, 0)? != 0, connector })
}

fn bcl_model(o: &Object) -> Option<Levels> {
    let e = elements(o)?;
    let pct = |i: usize| int(&e, i).filter(|v| *v <= 100).map(|v| v as u8);
    let mut steps: Vec<u8> = (2..e.len()).filter_map(pct).collect();
    steps.sort();
    steps.dedup();
    (!steps.is_empty()).then(|| Levels { on_ac: pct(0), on_battery: pct(1), steps })
}

#[test]
fn upc_and_bcl_are_their_models() {
    let port = |r: &mut Rng| {
        let kind = [0, 3, 9, 0xFF, 0x100][r.below(5) as usize];
        let e = vec![r.int_below(2), r.int(kind), Object::Integer(0), Object::Integer(0)];
        lie(r, e)
    };
    hold("_UPC", 0x09C, port, upc, upc_model);
    let levels = |r: &mut Rng| {
        let n = r.below(6) as usize;
        let mut e: Vec<Object> = (0..n + 2).map(|_| r.int_below(130)).collect();
        if r.below(4) == 0 {
            e = e.into_iter().map(|o| if r.below(3) == 0 { stranger(r) } else { o }).collect();
        }
        lie(r, e)
    };
    hold("_BCL", 0xBC1, levels, parse_bcl, bcl_model);
}

#[path = "pld_tests.rs"]
mod pld;
