// NONOS Operating System (AGPL-3.0-or-later)
//! One lie told in a sound transfer, and what must still be taken: the
//! frames before a message that ends the walk; every frame but that of a
//! message whose frame lies; and how many messages are dropped.

use core::ops::Range;

use super::build::{put, transfer, Rng, Transfer};
pub use super::lies::{Lie, LIES};

pub struct Told {
    pub lie: Lie,
    pub t: Transfer,
    pub expect: (Vec<Range<usize>>, u32),
}

pub fn tell(r: &mut Rng) -> Told {
    let lie = LIES[r.range(0, LIES.len() - 1)];
    let mut t = transfer(r);
    let j = r.range(0, t.msgs.len() - 1);
    let (at, frame) = t.msgs[j].clone();
    let len = u32::from_le_bytes(t.raw[at + 4..at + 8].try_into().unwrap()) as usize;
    let before = |t: &Transfer| t.msgs[..j].iter().map(|m| m.1.clone()).collect::<Vec<_>>();
    let but = |t: &Transfer| {
        t.msgs.iter().enumerate().filter(|m| m.0 != j).map(|m| m.1 .1.clone()).collect()
    };
    let expect = match lie {
        Lie::WrongType => {
            (put(&mut t.raw, at, r.range(2, u32::MAX as usize) as u32), (before(&t), 1)).1
        }
        Lie::LenShort => (put(&mut t.raw, at + 4, r.range(0, 43) as u32), (before(&t), 1)).1,
        Lie::LenPastTransfer => {
            let past = t.raw.len() - at + r.range(1, 1 << 20);
            (put(&mut t.raw, at + 4, past as u32), (before(&t), 1)).1
        }
        Lie::OffsetLow => (put(&mut t.raw, at + 8, r.range(0, 35) as u32), (but(&t), 1)).1,
        Lie::FrameShort => (put(&mut t.raw, at + 12, r.range(0, 13) as u32), (but(&t), 1)).1,
        Lie::FrameLong => {
            // The last message grown so a frame over 1514 bytes fits in it:
            // only the frame's own bound can refuse it.
            let (last, _) = t.msgs[t.msgs.len() - 1];
            let flen = r.range(1515, 3000);
            t.raw.truncate(last + 44);
            t.raw.extend(core::iter::repeat_n(0x5A, flen));
            put(&mut t.raw, last + 4, (44 + flen) as u32);
            put(&mut t.raw, last + 8, 36);
            put(&mut t.raw, last + 12, flen as u32);
            let kept = t.msgs[..t.msgs.len() - 1].iter().map(|m| m.1.clone()).collect();
            (kept, 1)
        }
        Lie::FramePastMessage => {
            let off = len - 8 - frame.len() + r.range(1, 64);
            (put(&mut t.raw, at + 8, off as u32), (but(&t), 1)).1
        }
        Lie::OffsetWraps => {
            (put(&mut t.raw, at + 8, u32::MAX - r.range(0, 64) as u32), (but(&t), 1)).1
        }
        Lie::TrailingJunk => {
            let all = t.msgs.iter().map(|m| m.1.clone()).collect();
            // Right after the last message, ahead of any zero padding.
            let (last, _) = t.msgs[t.msgs.len() - 1];
            let end =
                last + u32::from_le_bytes(t.raw[last + 4..last + 8].try_into().unwrap()) as usize;
            t.raw.truncate(end);
            t.raw.extend([7, 0, 0, 0, 60, 0, 0, 0]);
            (all, 1)
        }
    };
    Told { lie, t, expect }
}
