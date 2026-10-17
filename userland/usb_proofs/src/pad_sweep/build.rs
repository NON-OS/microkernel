// NONOS Operating System (AGPL-3.0-or-later)
//! A gamepad's report descriptor: a Joystick or Game Pad application (or
//! at times a mouse's, which is no gamepad), report IDs or none, input
//! groups inside it, some in a nested collection, some after it ends. One
//! lie may make it no gamepad at all: Push nested past 8, a Pop with
//! nothing pushed, a report ID of 0 or past a byte, or 65 usages before
//! one Input. The layout stated is its fields, when it is a gamepad
//! application with a field kept.

use super::collect::{collection, end};
pub use super::collect::{Lie, LIES};
use super::groups::group;
use super::writer::{F, W};
use crate::rng::Rng;

pub struct Desc {
    pub bytes: Vec<u8>,
    pub layout: Option<(bool, Vec<F>)>,
    /// Each report's bits.
    pub reports: Vec<(u8, u32)>,
}

pub fn descriptor(r: &mut Rng, lie: Lie) -> Desc {
    let mut w = W { ids: r.one_in(2), ..W::default() };
    let app = r.pick(&[5i64, 5, 4, 2]);
    w.item(r, 0x04, 1, false);
    w.item(r, 0x08, app, false);
    w.g.page = 1;
    collection(&mut w, r, 1, app != 2);
    let groups = 1 + r.below(6);
    let j = r.below(groups);
    for k in 0..groups {
        if w.ids && (k == 0 || r.one_in(3)) {
            let id = 1 + r.below(3) as i64;
            w.item(r, 0x84, id, false);
            w.g.id = id as u8;
        }
        if k == j {
            match lie {
                Lie::PushDeep => (0..9).for_each(|_| w.out.push(0xA4)),
                Lie::PopEmpty => w.out.push(0xB4),
                Lie::BadId => {
                    let id = r.pick(&[0, 256, 0x1_0000]);
                    w.item(r, 0x84, id, false);
                }
                Lie::Usages => (0..65).for_each(|_| w.out.extend([0x09, 0x30])),
                Lie::Sound => {}
            }
            w.broken = lie != Lie::Sound;
        }
        let nested = r.one_in(5);
        if nested {
            collection(&mut w, r, 0, false);
        }
        group(&mut w, r);
        if nested {
            end(&mut w);
        }
    }
    end(&mut w);
    for _ in 0..r.below(2) {
        group(&mut w, r);
    }
    let ok = !w.broken && w.seen && !w.fields.is_empty();
    let layout = ok.then(|| (w.ids, w.fields.clone()));
    Desc { bytes: w.out, layout, reports: w.offsets }
}
