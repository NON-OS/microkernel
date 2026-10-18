// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded scenes: three mutexes at random sync levels, firmware free,
//! holding the Global Lock a few looks or for good, or waiting for it, and
//! three to eight operations, each followed by its index stored to Debug.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    /// A mutex, 0 to 2, and the timeout: 0, 5 ms or for ever.
    Acquire(u8, u16),
    Release(u8),
    AcquireGl,
    ReleaseGl,
    Notify(u8),
    Sleep(u8),
    Stall(u8),
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub levels: [u8; 3],
    pub ops: Vec<Op>,
    /// None: no FACS; else firmware's looks held (u32::MAX for good).
    pub holds: Option<u32>,
    pub waits: bool,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn scene(seed: u64) -> Scene {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let levels = [below(s, 4) as u8, 4 + below(s, 8) as u8, 8 + below(s, 8) as u8];
    // Releases mostly name the mutex last taken, so most scenes run on.
    let mut taken: Vec<u8> = Vec::new();
    let ops = (0..3 + below(s, 6))
        .map(|_| {
            let m = below(s, 3) as u8;
            match below(s, 9) {
                0 | 1 => {
                    taken.push(m);
                    Op::Acquire(m, [0, 5, 0xFFFF][below(s, 3) as usize])
                }
                2 | 3 => Op::Release(match below(s, 5) {
                    0 => m,
                    _ => taken.pop().unwrap_or(m),
                }),
                4 => Op::AcquireGl,
                5 => Op::ReleaseGl,
                6 => Op::Notify(below(s, 4) as u8),
                7 => Op::Sleep(10 + below(s, 3) as u8),
                _ => Op::Stall(1 + below(s, 50) as u8),
            }
        })
        .collect();
    let holds = [None, Some(0), Some(0), Some(1 + below(s, 3) as u32), Some(u32::MAX)];
    Scene { levels, ops, holds: holds[below(s, 5) as usize], waits: below(s, 3) == 0 }
}
