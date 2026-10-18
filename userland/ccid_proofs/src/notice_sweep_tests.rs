// NONOS Operating System (AGPL-3.0-or-later)
//! Slot-change notices over seeded readers (CCID 1.1, 6.3.1, as recalled,
//! unverified): 50h, then two bits a slot from slot 0 up, the low one a
//! card present, the high one a change. Each reader sends one to four
//! notices, each sound or told one lie, and every slot is then asked once
//! and again. The rule: a slot changed when some notice of type 50h that
//! reached its pair set the high bit, and it is among the reader's slots
//! and the 64 the driver remembers; asked once, it is forgotten.

use crate::ccid::Slots;
use crate::exchange_sweep::rng::{rounds, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Lie {
    Sound,
    /// Another notification's type byte.
    OtherType,
    /// Cut before the reader's last slot.
    Short,
    /// Bytes past the reader's last slot, all bits set.
    Long,
}

const LIES: [Lie; 4] = [Lie::Sound, Lie::OtherType, Lie::Short, Lie::Long];

#[test]
fn every_slot_changes_only_as_a_whole_notice_of_its_own_says() {
    let mut told = [0u32; 4];
    for seed in 1..rounds(20_000) {
        let r = &mut Rng::new(seed);
        let some = r.range(0, 255);
        let max_slot = r.one_of(&[0, 1, 62, 63, 64, 255, some]) as u8;
        let (mut slots, mut want) = (Slots::default(), [false; 256]);
        for _ in 0..r.range(1, 4) {
            let lie = LIES[r.range(0, 3)];
            let pairs = max_slot as usize + 1;
            let mut n = vec![0x50];
            n.extend((0..pairs.div_ceil(4)).map(|_| r.range(0, 255) as u8));
            match lie {
                Lie::OtherType => n[0] = [0x51, 0xD0, 0x00, 0x10][r.range(0, 3)],
                Lie::Short => n.truncate(r.range(0, n.len() - 1)),
                Lie::Long => n.extend((0..r.range(1, 40)).map(|_| 0xFF)),
                Lie::Sound => {}
            }
            told[lie as usize] += 1;
            for s in 0..=max_slot as usize {
                let bit = 2 * s + 1;
                let set = n.get(1 + bit / 8).is_some_and(|b| b >> (bit % 8) & 1 == 1);
                want[s] |= n.first() == Some(&0x50) && set && s < 64;
            }
            slots.seen(&n, max_slot);
        }
        for s in 0..=255u8 {
            let at = format!("seed {seed}: reader of slots 0..={max_slot}, slot {s}");
            assert_eq!(slots.take_changed(s), want[s as usize], "{at}");
            assert!(!slots.take_changed(s), "{at}: asked again");
        }
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 5_000, "{lie:?} was told only {n} times");
    }
}
