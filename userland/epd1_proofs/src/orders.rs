// NONOS Operating System (AGPL-3.0-or-later)
//! The three events `step` answers, run in a given order, and every order
//! of a given length.

use crate::step::{built, exposed, root, IDLE};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Built,
    Exposed,
    Root,
}

const EVENTS: [Event; 3] = [Event::Built, Event::Exposed, Event::Root];

#[derive(Default)]
pub struct Run {
    pub switched: usize,
    pub with: usize,
    pub without: usize,
    pub early: bool,
    pub built: bool,
}

pub fn run(order: &[Event]) -> Run {
    let (mut state, mut out) = (IDLE, Run::default());
    for &event in order {
        match event {
            Event::Built => {
                state = built(state);
                out.built = true;
            }
            Event::Exposed => {
                let in_time = out.built && out.without == 0;
                let (next, yes) = exposed(state);
                out.early |= in_time && !yes;
                out.switched += usize::from(yes);
                state = next;
            }
            Event::Root => {
                let (next, yes) = root(state);
                if yes {
                    out.with += 1
                } else {
                    out.without += 1
                }
                state = next;
            }
        }
    }
    out
}

pub fn every(len: usize, check: &mut impl FnMut(&[Event])) {
    let mut order = vec![Event::Built; len];
    for n in 0..3usize.pow(len as u32) {
        let mut k = n;
        for slot in order.iter_mut() {
            *slot = EVENTS[k % 3];
            k /= 3;
        }
        check(&order);
    }
}
