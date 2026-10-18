// NONOS Operating System (AGPL-3.0-or-later)
//! One seeded program: four mutexes at random sync levels and \_GL (index
//! 4, level 0), two events, which mutexes another evaluation holds, the
//! time budget, and up to 24 operations with timeouts from none to for
//! ever.

#[derive(Clone, Copy, Debug)]
pub enum SyncOp {
    Acquire(usize, u16),
    Release(usize),
    Wait(usize, u64),
    Signal(usize),
    Reset(usize),
}

pub struct Program {
    pub levels: [u8; 5],
    pub foreign: [bool; 5],
    pub time_ms: u64,
    pub ops: Vec<SyncOp>,
}

pub fn roll(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n
}

pub fn program(s: &mut u64) -> Program {
    let mut levels = [0u8; 5];
    let mut foreign = [false; 5];
    for i in 0..4 {
        // Mostly few levels, so programs run past an Acquire; now and then
        // any of the sixteen.
        let span = if roll(s, 4) == 0 { 16 } else { 2 };
        levels[i] = roll(s, span) as u8;
        foreign[i] = roll(s, 5) == 0;
    }
    let time_ms = [20, 100, 1_000, 100_000][roll(s, 4) as usize];
    let (mut ops, mut taken) = (Vec::new(), Vec::new());
    for _ in 0..roll(s, 25) {
        ops.push(match roll(s, 10) {
            0..=2 => {
                let m = roll(s, 5) as usize;
                taken.push(m);
                SyncOp::Acquire(m, [0, 1, 7, 50, 900, 0xFFFF][roll(s, 6) as usize])
            }
            // Mostly one taken before, so programs run on past a Release.
            3..=5 => match taken.len() {
                n if n > 0 && roll(s, 4) != 0 => SyncOp::Release(taken[roll(s, n as u64) as usize]),
                _ => SyncOp::Release(roll(s, 5) as usize),
            },
            6 => SyncOp::Wait(
                roll(s, 2) as usize,
                [0, 3, 40, 700, 0xFFFF, 0x1_0000][roll(s, 6) as usize],
            ),
            7 | 8 => SyncOp::Signal(roll(s, 2) as usize),
            _ => SyncOp::Reset(roll(s, 2) as usize),
        });
    }
    Program { levels, foreign, time_ms, ops }
}
