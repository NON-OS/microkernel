// NONOS Operating System (AGPL-3.0-or-later)
//! Binding a modelled chip, bringing its link up, and finding accesses in
//! its log.

use nonos_usbnet::mock::{drawn, MockBus};
use nonos_usbnet::{Bind, Nic};

use crate::chip::{plug, Access, Plugged};
use crate::glue::{paused, HELD};
use crate::rtl::bind;
use crate::rtl::state::Rtl;
use crate::rtl::watch::LOOK_EVERY;

/// Gigabit, full duplex, link up.
pub const GIGA_UP: u32 = 0x10 | 0x02 | 0x01;

pub fn try_bind(p: &Plugged) -> Bind<Rtl<MockBus>> {
    HELD.with(|h| h.borrow_mut().clear());
    paused();
    bind(p.bus.clone(), &p.found, drawn())
}

/// A chip of `code` bound, its link still down.
pub fn bound(code: u16) -> (Plugged, Rtl<MockBus>) {
    let p = plug(code);
    match try_bind(&p) {
        Bind::Ours(r) => (p, r),
        Bind::NotOurs => panic!("{code:04x}: not ours"),
        Bind::Failed(what, e) => panic!("{code:04x}: {what} {e}"),
    }
}

/// The link word set to `word` and the driver's clock run to its next look.
pub fn link_to(p: &Plugged, r: &mut Rtl<MockBus>, word: u32) -> Result<(), i32> {
    p.chip.borrow_mut().link = word;
    for _ in 1..LOOK_EVERY {
        r.tick()?;
    }
    r.tick()
}

/// A chip of `code` bound and its link up at a gigabit.
pub fn up(code: u16) -> (Plugged, Rtl<MockBus>) {
    let (p, mut r) = bound(code);
    link_to(&p, &mut r, GIGA_UP).unwrap();
    assert!(r.link_up(), "{code:04x}: the link did not come up");
    (p, r)
}

/// Where the first access matching `f` is in the log.
pub fn first(p: &Plugged, f: impl Fn(&Access) -> bool) -> Option<usize> {
    p.chip.borrow().log.iter().position(f)
}

/// Where the last one is.
pub fn last(p: &Plugged, f: impl Fn(&Access) -> bool) -> Option<usize> {
    p.chip.borrow().log.iter().rposition(f)
}

/// A write to (block, dword) setting `bits` of the data.
pub fn wrote(block: u16, at: u16, bits: u32) -> impl Fn(&Access) -> bool {
    move |a| matches!(*a, Access::Write(b, d, _, v) if b == block && d == at && v & bits == bits)
}

/// The codes this driver binds.
pub const CODES: [u16; 6] = [0x5c00, 0x5c10, 0x5c20, 0x5c30, 0x6000, 0x6010];
