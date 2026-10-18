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

//! A controller that restarts at every step of a link: at each packet the
//! host sends, from the first pairing PDU through encryption, HID set-up
//! and the keys, it restarts out loud, silently, or by turning the link's
//! encryption off. Whatever the step, no ACL from the host reaches the
//! link once its encryption is lost, the link ends within its bounds, and
//! a bond is kept only by a pairing that finished.

use super::{A_DOWN, KEYS};
use crate::air::{Act, Restart};
use crate::fixture::{air, keyboard, link};

struct Seen {
    steps: u64,
    clear: u32,
    paired: bool,
    bonds: usize,
}

fn run(restart: Option<(u64, Restart)>) -> Seen {
    let (mut host, ours, mut env) = keyboard();
    let script =
        [Act::Notify(KEYS, A_DOWN.to_vec()), Act::Notify(KEYS, vec![0; 8]), Act::Drop(0x13)];
    air(&mut host).script.extend(script);
    air(&mut host).lies.restart = restart;
    let _ = link(&mut host, &ours, &mut env);
    let lies = &air(&mut host).lies;
    let bonds = env.bonds.book().bonds().len();
    Seen { steps: lies.steps, clear: lies.clear, paired: env.said("paired"), bonds }
}

#[test]
fn no_restart_lets_a_link_send_in_the_clear_or_keep_a_half_bond() {
    let whole = run(None);
    assert!(whole.paired && whole.bonds == 1 && whole.steps > 20, "{} steps", whole.steps);
    for at in 1..=whole.steps {
        for how in [Restart::Loud, Restart::Silent, Restart::Unsealed] {
            let s = run(Some((at, how)));
            assert_eq!(s.clear, 0, "{how:?} at step {at}: ACL after encryption was lost");
            assert_eq!(s.bonds, usize::from(s.paired), "{how:?} at step {at}: a half bond");
        }
    }
}
