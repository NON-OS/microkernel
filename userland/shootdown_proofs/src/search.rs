// NONOS Operating System (AGPL-3.0-or-later)
//! Breadth-first search over every reachable state of a round.

use std::collections::{HashSet, VecDeque};

use crate::model::{check, successors, Config, State};

pub struct Outcome {
    pub states: usize,
    /// The first broken rule, with the steps that reach it from the start.
    pub violation: Option<(&'static str, Vec<State>)>,
    /// A reachable state with no step left before the round returned.
    pub stuck: Option<State>,
}

pub fn explore(cfg: &Config) -> Outcome {
    let start = State::start(cfg);
    let mut seen: HashSet<State> = HashSet::new();
    let mut parent: std::collections::HashMap<State, State> = Default::default();
    let mut queue = VecDeque::new();
    let mut next = Vec::new();
    seen.insert(start);
    queue.push_back(start);
    let mut stuck = None;
    while let Some(s) = queue.pop_front() {
        if let Err(why) = check(&s, cfg) {
            let mut path = vec![s];
            let mut at = s;
            while let Some(p) = parent.get(&at) {
                path.push(*p);
                at = *p;
            }
            path.reverse();
            return Outcome { states: seen.len(), violation: Some((why, path)), stuck };
        }
        successors(&s, cfg, &mut next);
        if next.is_empty() && s.init != crate::model::Init::Done && stuck.is_none() {
            stuck = Some(s);
        }
        for t in next.drain(..) {
            if seen.insert(t) {
                parent.insert(t, s);
                queue.push_back(t);
            }
        }
    }
    Outcome { states: seen.len(), violation: None, stuck }
}
