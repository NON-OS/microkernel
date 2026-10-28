// NONOS Operating System (AGPL-3.0-or-later)
//! The step tables themselves: every register aligned to its width, every
//! PHY register a word, and no step writing a bit wider than its register.

use crate::helpers::CODES;
use crate::rtl::autoneg::AUTONEG;
use crate::rtl::link_down::LINK_DOWN;
use crate::rtl::link_up::link_up;
use crate::rtl::model::identify;
use crate::rtl::op::{Op, Target};
use crate::rtl::reg::Width;
use crate::rtl::start::start;
use crate::rtl::station::station;
use crate::rtl::usb_speed::UsbSpeed;
use crate::rtl::wake::wake;

fn fits(t: Target, bits: u32) {
    match t {
        Target::Mac(r) => {
            let (align, max) = match r.width {
                Width::W8 => (1, 0xff),
                Width::W16 => (2, 0xffff),
                Width::W32 => (4, u32::MAX),
            };
            assert_eq!(r.addr % align, 0, "{r:?} unaligned");
            assert!(bits <= max, "{r:?}: {bits:#x} wider than the register");
        }
        Target::Phy(a) => {
            assert!(a % 2 == 0 && (0xa000..=0xbffe).contains(&a), "PHY {a:#06x}");
            assert!(bits <= 0xffff, "PHY {a:#06x}: {bits:#x}");
        }
    }
}

#[test]
fn every_step_names_a_register_it_fits() {
    for code in CODES {
        let m = identify(code).unwrap();
        for speed in [UsbSpeed::Full, UsbSpeed::High, UsbSpeed::Super] {
            for word in [0x13, 0x0b, 0x0a, 0x06, 0x05] {
                let all = [
                    wake(m, speed),
                    station([0xff; 6]).to_vec(),
                    AUTONEG.to_vec(),
                    start(m),
                    link_up(m, speed, word),
                    LINK_DOWN.to_vec(),
                ]
                .concat();
                for op in all {
                    match op {
                        Op::Edit(t, off, on) => fits(t, off | on),
                        Op::Put(t, v) => fits(t, v),
                        Op::Until(t, mask, _, _) => fits(t, mask),
                        Op::EditIf(test, bits, t, off, on) => {
                            fits(test, bits);
                            fits(t, off | on);
                        }
                        Op::Pause(_) => {}
                    }
                }
            }
        }
    }
}
