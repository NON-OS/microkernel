// NONOS Operating System (AGPL-3.0-or-later)
//! Which CPUs turn the trampoline on: exactly those the speculation
//! decision calls exposed, which is a Cortex-A75 whose ID registers claim
//! neither CSV3 nor E0PD, and on a mixed machine those cores and no other.

use crate::spectre::answer::{Answer, Firmware};
use crate::spectre::decide::decide;
use crate::spectre::ids::Ids;
use crate::spectre::plan::Meltdown;
use crate::step::{built, exposed, root, IDLE};

const ARM: u64 = 0x41;
const A75: u64 = 0xD0A;
const ANY: Firmware = Firmware { wa1: Answer::Unknown, wa2: Answer::Unknown, wa3: Answer::Unknown };

/// A CPU's ID registers: MIDR from implementer and part, CSV3 at
/// ID_AA64PFR0_EL1 63:60 and E0PD at ID_AA64MMFR2_EL1 63:60.
fn cpu(implementer: u64, part: u64, csv3: u64, e0pd: u64) -> Ids {
    let midr = implementer << 24 | 0xF << 16 | part << 4;
    Ids { pfr0: csv3 << 60, mmfr2: e0pd << 60, midr, ..Ids::default() }
}

fn is_exposed(ids: &Ids) -> bool {
    decide(ids, &ANY).meltdown == Meltdown::Exposed
}

#[test]
fn exposed_is_an_a75_without_csv3_or_e0pd_and_nothing_else() {
    for implementer in [ARM, 0x00, 0x42, 0x43, 0x46, 0x48, 0x4E, 0x51, 0x61, 0xC0] {
        for part in 0..0x1000 {
            for csv3 in 0..3 {
                for e0pd in 0..3 {
                    let want = implementer == ARM && part == A75 && csv3 == 0 && e0pd == 0;
                    assert_eq!(is_exposed(&cpu(implementer, part, csv3, e0pd)), want);
                }
            }
        }
    }
}

/// A75, A55, A76, an A75 claiming E0PD, an Apple core.
const KINDS: [(u64, u64, u64); 5] =
    [(ARM, A75, 0), (ARM, 0xD05, 0), (ARM, 0xD0B, 0), (ARM, A75, 1), (0x61, 0x022, 0)];

/// A machine of `cores`, brought up as the kernel does: the boot CPU's
/// plan, the tables built and the boot CPU turned on if exposed, then each
/// secondary deciding and asking, then the first user root.
fn boot(cores: &[(u64, u64, u64)]) -> (Vec<bool>, bool) {
    let mut state = IDLE;
    let plans: Vec<bool> = cores.iter().map(|&(i, p, e)| is_exposed(&cpu(i, p, 0, e))).collect();
    state = built(state);
    let mut on = Vec::new();
    for &exposed_cpu in &plans {
        let (next, yes) = if exposed_cpu { exposed(state) } else { (state, false) };
        on.push(yes);
        state = next;
    }
    (on, root(state).1)
}

#[test]
fn on_any_mix_exactly_the_exposed_cores_switch() {
    for count in 1..=4u32 {
        for n in 0..5usize.pow(count) {
            let cores: Vec<_> = (0..count).map(|k| KINDS[n / 5usize.pow(k) % 5]).collect();
            let (on, carried) = boot(&cores);
            let want: Vec<bool> =
                cores.iter().map(|&(i, p, e)| i == ARM && p == A75 && e == 0).collect();
            assert_eq!(on, want, "{cores:x?}");
            assert_eq!(carried, want.contains(&true), "{cores:x?}");
        }
    }
}
