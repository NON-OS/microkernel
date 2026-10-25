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
//! Device data from a hostile codec, seeded: every parameter, connection
//! list entry (short and long form, ranges among them) and pin configuration
//! the description leaves open is a word hashed from the seed. The walk, the
//! plan and the programming run on each, and must end, in time, inside their
//! bounds, with every output path a real chain of the codec's own widgets.

use std::time::{Duration, Instant};

use crate::constants::{
    FUNCTION_GROUP_AUDIO, PARAM_AUDIO_WIDGET_CAP, PARAM_CONNLIST_LEN, PARAM_FUNCTION_GROUP_TYPE,
    PARAM_PCM, PARAM_PIN_CAP, PARAM_SUBNODE_COUNT, PCM_BITS_16, PCM_RATE_48K, PINCAP_OUT,
    WCAP_CONN_LIST, WCAP_FORMAT_OVRD, WIDGET_TYPE_DAC, WIDGET_TYPE_PIN,
};
use crate::controller::codec::path::MAX_DEPTH;
use crate::controller::codec::plan::{plan, Class};
use crate::controller::codec::program::program;
use crate::controller::codec::walk::walk;
use crate::controller::codec::widget::{Codec, MAX_CONN, MAX_WIDGETS};
use crate::sim::{start, SimCodec};

const SEEDS: u64 = 256;
/// Far past what the slowest seed takes: a walk is at most a few hundred
/// verbs, each a round trip through the model's thread.
const PER_SEED: Duration = Duration::from_secs(20);

/// A speaker pin, default config: fixed, speaker, analog, location internal.
const SPEAKER_CFG: u32 = 0x9017_0110;

/// Codec 0, all noise but for what `shaped` gives it: a root and an audio
/// function group at node 1 whose widgets start at node 2, a DAC at 2 that
/// plays 48 kHz 16-bit and a speaker pin at 3 whose first connection is
/// the DAC. The widgets past them, and the pin's other connections, stay
/// noise, so the route search has a real path among hostile neighbours.
fn hostile(seed: u64, shaped: bool) -> SimCodec {
    let mut c = SimCodec { noise: Some(seed), ..Default::default() };
    if shaped {
        let widgets = 2 + (seed % 60) as u32;
        c.param(0, PARAM_SUBNODE_COUNT, 1 << 16 | 1);
        c.param(1, PARAM_FUNCTION_GROUP_TYPE, FUNCTION_GROUP_AUDIO);
        c.param(1, PARAM_SUBNODE_COUNT, 2 << 16 | widgets);
        c.param(2, PARAM_AUDIO_WIDGET_CAP, WIDGET_TYPE_DAC << 20 | WCAP_FORMAT_OVRD | 1);
        c.param(2, PARAM_PCM, PCM_RATE_48K | PCM_BITS_16);
        c.param(3, PARAM_AUDIO_WIDGET_CAP, WIDGET_TYPE_PIN << 20 | WCAP_CONN_LIST | 1);
        c.param(3, PARAM_PIN_CAP, PINCAP_OUT);
        c.param(3, PARAM_CONNLIST_LEN, 1 + (seed % 7) as u32);
        let n = c.node(3);
        n.pin_cfg = SPEAKER_CFG;
        n.conn = vec![2, (seed >> 8) as u8, (seed >> 16) as u8, 0x7f, 0, 3, 2];
    }
    c
}

fn widgets_hold_their_bounds(c: &Codec) {
    assert!(c.n <= MAX_WIDGETS);
    for w in c.all() {
        assert!(w.nid <= 0x7f, "widget {:#x} past the node range", w.nid);
        assert!(w.n_conn as usize <= MAX_CONN);
        assert!(w.connections().iter().all(|&n| n <= 0x7f), "a connection no verb can address");
    }
}

/// Each output path starts at a pin, ends at a DAC, and every step is the
/// connection its selector names on a widget the walk recorded.
fn paths_are_real_chains(c: &Codec, class: &Class) {
    let Class::Analog(p) = class else { return };
    for out in p.iter() {
        let path = &out.path;
        let len = path.len as usize;
        assert!((1..=MAX_DEPTH).contains(&len), "a path of {len} nodes");
        let at = |i: usize| {
            c.get(path.nodes[i]).unwrap_or_else(|| panic!("{:#x} not walked", path.nodes[i]))
        };
        assert_eq!(at(0).ty(), WIDGET_TYPE_PIN, "the path does not start at a pin");
        assert_eq!(at(len - 1).ty(), WIDGET_TYPE_DAC, "the path does not end at a DAC");
        for i in 0..len - 1 {
            let conn = at(i).connections();
            let sel = path.sel[i] as usize;
            assert!(sel < conn.len(), "selector {sel} past {} connections", conn.len());
            assert_eq!(conn[sel], path.nodes[i + 1], "step {i} is not the connection selected");
        }
    }
}

#[test]
fn a_hostile_codec_is_walked_planned_and_programmed_within_bounds() {
    let (mut walked, mut planned) = (0, 0);
    for seed in 0..SEEDS {
        let sim = start(vec![(0, hostile(seed, seed % 4 != 0))]);
        let mut link = sim.link();
        let t = Instant::now();
        if let Ok(Some(codec)) = walk(&mut link, 0) {
            walked += 1;
            widgets_hold_their_bounds(&codec);
            let class = plan(&codec);
            paths_are_real_chains(&codec, &class);
            if let Class::Analog(p) = &class {
                planned += 1;
                let _ = program(&mut link, &codec, p, 1);
            }
        }
        assert!(t.elapsed() < PER_SEED, "seed {seed} took {:?}", t.elapsed());
    }
    // The sweep proves nothing if the noise never reached the deep code.
    assert!(walked >= SEEDS / 2, "only {walked} of {SEEDS} seeds were walked");
    assert!(planned > 0, "no seed reached a plan");
}
