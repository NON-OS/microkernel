// NONOS Operating System (AGPL-3.0-or-later)
//! Rounds over a batch's bounds: ids at, below and past the slots asked for, repeated or missing.

use super::rng::Rng;
use crate::net::rpc::call_results;

fn reply(ids: &[String]) -> String {
    let objects: Vec<String> = ids
        .iter()
        .map(|id| format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":\"0xab\"}}"))
        .collect();
    format!("[{}]", objects.join(","))
}

/// Run `rounds` rounds from `seed`; returns the number run.
pub fn batch_rounds(seed: u64, rounds: usize) -> usize {
    let mut rng = Rng::new(seed);
    for _ in 0..rounds {
        let n = 1 + rng.below(8);
        let mut ids: Vec<String> = (0..n).map(|i| i.to_string()).collect();
        for _ in 0..n {
            let (a, b) = (rng.below(n), rng.below(n));
            ids.swap(a, b);
        }
        assert_eq!(call_results(&reply(&ids), n).map(|v| v.len()), Ok(n), "every slot, any order");
        let past = [n.to_string(), usize::MAX.to_string(), String::from("99999999999999999999999")];
        for id in past {
            let mut more = ids.clone();
            more.push(id);
            assert!(call_results(&reply(&more), n).is_err(), "an id past the slots");
        }
        let mut twice = ids.clone();
        twice.push(ids[rng.below(n)].clone());
        assert!(call_results(&reply(&twice), n).is_err(), "an id answered twice");
        let mut short = ids.clone();
        short.remove(rng.below(n));
        assert!(call_results(&reply(&short), n).is_err(), "a slot left unanswered");
    }
    rounds
}
