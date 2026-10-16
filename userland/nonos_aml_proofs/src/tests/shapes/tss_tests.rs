// NONOS Operating System (AGPL-3.0-or-later)
//! _TSS: five integers a state, the percentage first and never rising.

use super::super::gen::Rng;
use super::super::run::{elements, hold, some};
use super::{states, words};
use crate::cpu::{parse_tss, TState};
use crate::object::Object;

fn tss(o: &Object) -> Option<Vec<TState>> {
    let mut out: Vec<TState> = Vec::new();
    for s in elements(o)?.iter().take(16) {
        let Some(w) = words(s, 5) else { break };
        let most = out.last().map_or(100, |t| t.percent);
        if w[0] < 1 || w[0] > most {
            break;
        }
        let [percent, milliwatts, latency_us, control, status] = [w[0], w[1], w[2], w[3], w[4]];
        out.push(TState { percent, milliwatts, latency_us, control, status });
    }
    some(out)
}

#[test]
fn tss_is_its_model() {
    let make = |r: &mut Rng| {
        let mut left = 100u64;
        states(r, 5, &mut move |r| {
            left = match r.below(12) {
                0 => left + 10,
                1 => 0,
                _ => left.saturating_sub(r.below(25)),
            };
            left
        })
    };
    hold("_TSS", 0x755, make, |o| some(parse_tss(o)), tss);
}
