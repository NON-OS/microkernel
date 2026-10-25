// NONOS Operating System (AGPL-3.0-or-later)
//! `pick` against a plain statement of the rule, over many generated CPUs.

use crate::arch::fdt::find::idle::IdleState;
use crate::format::is_standby;
use crate::rules::Allow;
use crate::select::pick;

/// The rule, written out: the first listed of the eligible states whose
/// residency no other eligible state exceeds.
fn rule(
    states: &[IdleState],
    list: &[u8],
    ext: bool,
    refused: u8,
    budget: u64,
    allow: Allow,
) -> Option<u8> {
    let ok = |&i: &u8| {
        let Some(s) = states.get(i as usize) else { return false };
        let cost = s.residency_us as u64 + s.exit_us as u64;
        i < 8
            && (allow.timer_stop || !s.timer_stops)
            && (allow.powerdown || is_standby(s.param, ext))
            && refused >> i & 1 == 0
            && cost <= budget
    };
    let deepest = list.iter().filter(|i| ok(i)).map(|&i| states[i as usize].residency_us).max()?;
    list.iter().copied().find(|i| ok(i) && states[*i as usize].residency_us == deepest)
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) % bound
    }
}

#[test]
fn pick_follows_the_rule() {
    let mut r = Lcg(36);
    let mut entered = 0;
    for _ in 0..200_000 {
        let n = r.next(11) as usize;
        let states: Vec<IdleState> = (0..n)
            .map(|i| IdleState {
                phandle: i as u32 + 1,
                param: [0, 1 << 16, 1 << 30, 3][r.next(4) as usize],
                entry_us: r.next(50) as u32,
                exit_us: r.next(50) as u32,
                residency_us: r.next(400) as u32,
                timer_stops: r.next(4) == 0,
            })
            .collect();
        let list: Vec<u8> = (0..r.next(5)).map(|_| r.next(12) as u8).collect();
        let (ext, refused, budget) = (r.next(2) == 1, r.next(256) as u8, r.next(500));
        let allow = Allow { powerdown: r.next(2) == 1, timer_stop: r.next(2) == 1 };
        let got = pick(&states, &list, ext, refused, budget, allow);
        assert_eq!(got, rule(&states, &list, ext, refused, budget, allow), "{states:?} {list:?}");
        entered += got.is_some() as u32;
    }
    assert!(entered > 10_000, "the cases hardly ever enter a state: {entered}");
}
