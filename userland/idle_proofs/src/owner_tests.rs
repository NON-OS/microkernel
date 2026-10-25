// NONOS Operating System (AGPL-3.0-or-later)
//! The broadcast frame's owner never stops its timer or powers down, over
//! every CPU, owner, frame and resume state, and through `pick` over
//! generated states: whatever the tree offers, the owner waits awake.

use crate::arch::fdt::find::idle::IdleState;
use crate::format::is_standby;
use crate::rules::{allowance, Allow};
use crate::select::pick;

#[test]
fn the_owner_is_never_allowed_either() {
    for cpu in 0..5 {
        for owner in [None, Some(0), Some(1), Some(4), Some(9)] {
            for (frame, resume) in [(false, false), (false, true), (true, false), (true, true)] {
                let got = allowance(cpu, owner, frame, resume);
                let owns = owner == Some(cpu);
                let want = Allow {
                    powerdown: resume && !owns,
                    timer_stop: frame && !owns && owner.is_some(),
                };
                assert_eq!(got, want, "cpu {cpu} owner {owner:?} frame {frame} resume {resume}");
            }
        }
    }
}

#[test]
fn the_owner_picks_neither_from_any_tree() {
    let mut seed = 38u64;
    let mut next = |bound: u64| {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) % bound
    };
    for _ in 0..100_000 {
        let states: Vec<IdleState> = (0..1 + next(8))
            .map(|_| IdleState {
                param: [0, 1 << 16, 1 << 30][next(3) as usize],
                residency_us: next(400) as u32,
                timer_stops: next(2) == 0,
                ..IdleState::default()
            })
            .collect();
        let list: Vec<u8> = (0..states.len() as u8).collect();
        let (owner, ext) = (next(4) as usize, next(2) == 1);
        let allow = allowance(owner, Some(owner), true, true);
        if let Some(i) = pick(&states, &list, ext, 0, u64::MAX, allow) {
            let state = &states[usize::from(i)];
            assert!(!state.timer_stops && is_standby(state.param, ext), "owner took {state:?}");
        }
    }
}
