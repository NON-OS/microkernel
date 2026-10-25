// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded shutdowns: \_TTS absent or a loop; \_PTS absent or a store of
//! its argument and a loop, holding a mutex or \_GL (released, or left
//! held as buggy firmware does); \_S5 a Name, or a Method that takes the
//! same mutex for ever and returns what \_PTS stored; and which of \_TTS
//! and \_PTS the budget cuts.

pub struct Scene {
    pub tts: Option<u8>,
    /// The loop's length, and what it holds: 0 nothing, 1 \MUTX, 2 \_GL.
    pub pts: Option<(u8, u8)>,
    pub releases: bool,
    pub s5_method: bool,
    /// True: the budget cuts \_PTS; else \_TTS.
    pub cut_pts: bool,
}

pub fn below(s: &mut u64, n: u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s % n.max(1)
}

pub fn scene(seed: u64) -> Scene {
    let s = &mut (seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let tts = (below(s, 4) != 0).then(|| below(s, 6) as u8);
    let pts = (below(s, 5) != 0).then(|| (below(s, 6) as u8, below(s, 3) as u8));
    let cut_pts = match (tts, pts) {
        (None, _) => true,
        (_, None) => false,
        _ => below(s, 2) == 0,
    };
    Scene { tts, pts, releases: below(s, 2) == 0, s5_method: below(s, 3) != 0, cut_pts }
}
