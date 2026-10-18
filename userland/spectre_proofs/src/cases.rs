// NONOS Operating System (AGPL-3.0-or-later)
//! Every combination of the decision's inputs the proofs walk: each ID field
//! at each value the decision tells apart, each core class, and each of the
//! three firmware answers for each of the three workarounds.

use crate::spectre::answer::{Answer, Firmware};
use crate::spectre::ids::Ids;

pub const ARM: u64 = 0x41;
pub const A53: u64 = 0xD03;
pub const A57: u64 = 0xD07;
pub const A72: u64 = 0xD08;
pub const A73: u64 = 0xD09;
pub const A75: u64 = 0xD0A;
pub const A76: u64 = 0xD0B;
pub const A78: u64 = 0xD41;

/// Arm cores of each kind, an Arm part the tables do not name, and two
/// other implementers (0x61 Apple, 0x51 Qualcomm).
pub const CORES: [(u64, u64); 10] = [
    (ARM, A53),
    (ARM, A57),
    (ARM, A72),
    (ARM, A73),
    (ARM, A75),
    (ARM, A76),
    (ARM, A78),
    (ARM, 0xD4F),
    (0x61, 0x022),
    (0x51, 0x800),
];

pub const ANSWERS: [Answer; 3] = [Answer::Call, Answer::NotRequired, Answer::Unknown];

/// MIDR_EL1 with architecture 0xF, variant and revision 0.
pub fn midr(implementer: u64, part: u64) -> u64 {
    (implementer << 24) | (0xF << 16) | (part << 4)
}

/// Fields as (csv2, csv3, ssbs, ecbhb, clrbhb, e0pd), placed where the
/// Arm ARM puts them.
pub fn ids(f: [u64; 6], core: (u64, u64)) -> Ids {
    let [csv2, csv3, ssbs, ecbhb, clrbhb, e0pd] = f;
    Ids {
        pfr0: (csv2 << 56) | (csv3 << 60),
        pfr1: ssbs << 4,
        mmfr1: ecbhb << 60,
        mmfr2: e0pd << 60,
        isar2: clrbhb << 28,
        midr: midr(core.0, core.1),
    }
}

pub fn every(mut check: impl FnMut(&Ids, &Firmware)) {
    for csv2 in 0..4 {
        for bits in 0..32u64 {
            let f = [csv2, bits & 1, bits >> 1 & 1, bits >> 2 & 1, bits >> 3 & 1, bits >> 4 & 1];
            for core in CORES {
                let id = ids(f, core);
                for wa1 in ANSWERS {
                    for wa2 in ANSWERS {
                        for wa3 in ANSWERS {
                            check(&id, &Firmware { wa1, wa2, wa3 });
                        }
                    }
                }
            }
        }
    }
}
