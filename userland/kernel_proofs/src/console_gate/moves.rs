// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Every step that can be taken from a world.

use super::actors::{Hold, Taker, World};
use super::drawer::step;
use super::events::{Actor, Event};
use super::gate::Gate;
use super::hold_steps::hold;
use super::others::taker;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub actor: Actor,
    /// A takeover's claim made ready at its admit, or bootfb's HELD
    /// cleared by a release while the hold's wait still runs.
    pub pick: bool,
}

/// The moves to try: each actor's next step, and each choice both ways.
pub fn moves(w: &World) -> Vec<Move> {
    let one = |actor| Move { actor, pick: false };
    let both = |actor| [Move { actor, pick: false }, Move { actor, pick: true }];
    let mut v = vec![one(Actor::Drawer(0)), one(Actor::Drawer(1))];
    for i in 0..2 {
        match w.takers[i] {
            Taker::Quiet => v.extend(both(Actor::Taker(i))),
            _ => v.push(one(Actor::Taker(i))),
        }
    }
    match w.hold {
        Hold::Set => v.extend(both(Actor::Hold)),
        _ => v.push(one(Actor::Hold)),
    }
    v
}

/// The world after `m` and what the step was; None if it cannot be taken.
pub fn apply(g: &Gate, w: &World, m: Move) -> Option<(World, Event)> {
    let mut n = *w;
    let e = match m.actor {
        Actor::Drawer(i) => step(g, &mut n.flags, &mut n.drawers[i]),
        Actor::Taker(i) => taker(g, &mut n.flags, &mut n.takers[i], m.pick),
        Actor::Hold => hold(&mut n.flags, &mut n.hold, m.pick),
    }?;
    Some((n, e))
}
