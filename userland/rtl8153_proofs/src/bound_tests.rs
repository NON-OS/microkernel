// NONOS Operating System (AGPL-3.0-or-later)
//! Every wait ends. A chip that never sets what a wait looks for is looked
//! at exactly as often as the wait's bound says; a wait that must end
//! stops the bind by name with ETIMEDOUT, one that need not lets it go on.

use nonos_usbnet::Bind;

use crate::chip::{plug, Access, Stuck, PLA};
use crate::glue::paused;
use crate::helpers::try_bind;
use crate::rtl::bounds::PHY_SETTLED;
use crate::rtl::bounds::{ALDPS_OFF, AUTOLOAD, LINK_LIST, MAC_RESET, PHY_ON, PHY_RESET};

fn stuck(
    code: u16,
    s: Stuck,
) -> (crate::chip::Plugged, Bind<crate::rtl::state::Rtl<nonos_usbnet::mock::MockBus>>) {
    let p = plug(code);
    p.chip.borrow_mut().stuck = s;
    let got = try_bind(&p);
    (p, got)
}

fn looks(p: &crate::chip::Plugged, a: Access) -> usize {
    p.chip.borrow().log.iter().filter(|x| **x == a).count()
}

#[test]
fn a_wait_that_must_end_stops_the_bind_by_name_after_its_bound() {
    let cases = [
        (Stuck::Autoload, AUTOLOAD, Access::Read(PLA, 0xe004), 0),
        (Stuck::MacReset, MAC_RESET, Access::Read(PLA, 0xe810), 0),
        (Stuck::LinkList, LINK_LIST, Access::Read(PLA, 0xe84c), 1),
        (Stuck::PhyOn, PHY_ON, Access::PhyRead(0xa420), 1),
    ];
    for code in [0x5c10, 0x6010] {
        for (s, w, reg, before) in cases {
            let (p, got) = stuck(code, s);
            let Bind::Failed(what, e) = got else { panic!("{code:04x} {s:?}: bound") };
            assert_eq!((what, e), (w.fail.unwrap(), -110), "{code:04x} {s:?}");
            assert_eq!(looks(&p, reg), w.tries as usize + before, "{code:04x} {s:?}");
        }
    }
    let (_p, _) = stuck(0x5c30, Stuck::Autoload);
    assert_eq!(paused(), AUTOLOAD.tries as u64 * AUTOLOAD.every_ms as u64);
}

#[test]
fn a_wait_that_may_end_unmet_lets_the_bind_go_on_after_its_bound() {
    let (p, got) = stuck(0x5c20, Stuck::Aldps);
    assert!(matches!(got, Bind::Ours(_)));
    assert_eq!(looks(&p, Access::Read(PLA, 0xe000)), ALDPS_OFF.tries as usize);
    let (p, got) = stuck(0x6000, Stuck::PhyReset);
    assert!(matches!(got, Bind::Ours(_)));
    let log = p.chip.borrow().log.clone();
    let reset =
        log.iter().position(|a| matches!(*a, Access::PhyWrite(0xa400, v) if v & 0x8000 != 0));
    let after = log[reset.unwrap()..].iter().filter(|a| **a == Access::PhyRead(0xa400)).count();
    assert_eq!(after, PHY_RESET.tries as usize);
    let (p, got) = stuck(0x5c00, Stuck::PhySettle);
    assert!(matches!(got, Bind::Failed("PHY never came on", -110)));
    let n = (PHY_SETTLED.tries + PHY_ON.tries) as usize;
    assert_eq!(
        looks(&p, Access::PhyRead(0xa420)),
        n,
        "the settle wait goes on, then LAN-on gives up"
    );
}
