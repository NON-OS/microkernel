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

//! The circuit: four Poseidon chains in four membership regions, and one wire.
//!
//! | region | row 0, pinned | private | pinned at the checkpoint |
//! |---|---|---|---|
//! | bootloader | lane 0 leaf domain, lane 5 kind 3, lanes 6 and 7 zero | digest, path in B | B |
//! | kernel | lane 0 leaf domain, lane 5 kind 0, lanes 6 and 7 zero | digest, path in P | P |
//! | device | lane 0 device domain, lanes 5 to 7 zero | `s`, path in R | R |
//! | tag | lane 0 tag domain, lanes 5 and 6 the scope, lane 7 zero | `s` | t |
//!
//! Row 0 of an opening is the first compression's state, `[node, sibling]`. So
//! each region's first step is the leaf hash, and pinning lanes 0 and 5 to 7
//! fixes the domain and the kind while lanes 1 to 4 carry the private digest.
//! The kind is pinned, never witnessed: a bootloader slot cannot open as a
//! kernel, and a padding slot, kind 2, opens as neither.
//!
//! The one binding between regions is `s`: lanes 1 to 4 of row 0 in the device
//! region equal the same lanes in the tag region, so the tag is the enrolled
//! device's and no other secret's.

use alloc::vec;
use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::{
    Air, MultiMembership, Opening, Publics, ShieldRegion, WiredMultiGen, RATE,
};
use stark_proofs::crypto::stark::field::Fp;
use stark_proofs::shield::wire::offsets;
use stark_proofs::shield::wire_class::{tie, Class};
use stark_proofs::shield::wire_pack::{groups_enforce, packed_groups, CAP};

use crate::domain::{DEVICE_DOMAIN, KIND_BOOTLOADER, KIND_KERNEL, LEAF_DOMAIN, TAG_DOMAIN};
use crate::native::words_of;
use crate::params::{
    hasher, BOOT_DEPTH, KERNEL_DEPTH, LOG_ROUNDS, LOG_TRACE, MASK_COLUMNS, PAD_LOG,
};
use crate::statement::Statement;
use crate::witness::{Path, Slot, Witness};

const ROUNDS: usize = 1 << LOG_ROUNDS;
/// The four chains and the padding region.
const REGIONS: usize = 5;
const DEVICE: usize = 2;
const TAG: usize = 3;

pub struct Built {
    pub wired: WiredMultiGen,
    pub traces: Vec<Vec<Fp>>,
}

/// A chain whose first compression is pinned at row 0 and whose walked digest
/// is pinned to `root` at the checkpoint's successor row.
fn chain(opening: Opening, pins: [(usize, Fp); 4], root: [Fp; RATE]) -> MultiMembership {
    let depth = opening.siblings.len();
    let mut bound: Vec<(usize, usize, Fp)> = pins.iter().map(|&(c, v)| (c, 0, v)).collect();
    bound.extend(root.iter().enumerate().map(|(c, r)| (c, depth * ROUNDS, *r)));
    MultiMembership::new_witness_bound(hasher(), LOG_ROUNDS, vec![opening], bound).with_split()
}

fn opening(head: [Fp; RATE], first: [Fp; RATE], path: &Path, root: [Fp; RATE]) -> Opening {
    let mut siblings = Vec::with_capacity(1 + path.siblings.len());
    siblings.push(first);
    siblings.extend_from_slice(&path.siblings);
    let mut directions = Vec::with_capacity(1 + path.right.len());
    directions.push(false);
    directions.extend_from_slice(&path.right);
    Opening { leaf: head, root, siblings, directions }
}

fn blank(depth: usize) -> Path {
    Path { siblings: vec![[Fp::ZERO; RATE]; depth], right: vec![false; depth] }
}

/*
 * What a dishonest prover writes where the circuit pins something else. The
 * honest builder never differs from the pins, so a tamper test that used it
 * would be refused by the root check and prove nothing about the pin it names.
 * Tests set these to model the attacker; production passes the default.
 */
#[derive(Default, Clone, Copy)]
pub(crate) struct Forge {
    /// The kind written into the bootloader chain's lane 5.
    pub boot_kind: Option<u64>,
    /// The secret written into the tag region.
    pub tag_secret: Option<[Fp; RATE]>,
}

fn slot_chain(
    kind: u64,
    written: u64,
    slot: Option<&Slot>,
    depth: usize,
    root: [Fp; RATE],
) -> Option<MultiMembership> {
    let dom = Fp::from_u64(LEAF_DOMAIN);
    let k = Fp::from_u64(kind);
    let wk = Fp::from_u64(written);
    let (d, path) = match slot {
        Some(s) if s.path.siblings.len() == depth && s.path.right.len() == depth => {
            (words_of(&s.digest), s.path.clone())
        }
        Some(_) => return None,
        None => ([Fp::ZERO; RATE], blank(depth)),
    };
    let o = opening([dom, d[0], d[1], d[2]], [d[3], wk, Fp::ZERO, Fp::ZERO], &path, root);
    Some(chain(o, [(0, dom), (5, k), (6, Fp::ZERO), (7, Fp::ZERO)], root))
}

/*
 * Build the circuit for `st`. With a witness it is the prover's, traces filled;
 * without, it is the verifier's shape, whose constraints, boundaries and
 * periodic columns depend on the statement alone.
 */
pub fn build(st: &Statement, w: Option<&Witness>) -> Option<Built> {
    build_with(st, w, Forge::default())
}

pub(crate) fn build_with(st: &Statement, w: Option<&Witness>, forge: Forge) -> Option<Built> {
    let z = Fp::ZERO;
    let s = w.map(|w| w.secret).unwrap_or([z; RATE]);
    let device_path = match w {
        Some(w)
            if w.device.siblings.len() == st.device_depth
                && w.device.right.len() == st.device_depth =>
        {
            w.device.clone()
        }
        Some(_) => return None,
        None => blank(st.device_depth),
    };
    let dev = Fp::from_u64(DEVICE_DOMAIN);
    let tg = Fp::from_u64(TAG_DOMAIN);
    let (e0, e1) = (st.scope[0], st.scope[1]);
    let ts = forge.tag_secret.unwrap_or(s);

    let chains = [
        slot_chain(
            KIND_BOOTLOADER,
            forge.boot_kind.unwrap_or(KIND_BOOTLOADER),
            w.map(|w| &w.bootloader),
            BOOT_DEPTH,
            st.boot_root,
        )?,
        slot_chain(KIND_KERNEL, KIND_KERNEL, w.map(|w| &w.kernel), KERNEL_DEPTH, st.kernel_root)?,
        chain(
            opening([dev, s[0], s[1], s[2]], [s[3], z, z, z], &device_path, st.device_root),
            [(0, dev), (5, z), (6, z), (7, z)],
            st.device_root,
        ),
        chain(
            opening([tg, ts[0], ts[1], ts[2]], [ts[3], e0, e1, z], &blank(0), st.tag),
            [(0, tg), (5, e0), (6, e1), (7, z)],
            st.tag,
        ),
    ];

    /*
     * The padding: a region with no constraints, no pins and a zero witness,
     * stacked after the chains. It exists only so the trace is long enough for
     * FRI to fold three times; nothing reads it and nothing is bound to it.
     */
    let pad = Publics { log_t: PAD_LOG, words: Vec::new() };
    let mut traces: Vec<Vec<Fp>> = chains.iter().map(|c| c.trace()).collect();
    traces.push(pad.trace());
    let mut rows: Vec<usize> = chains.iter().map(Air::rows).collect();
    rows.push(Air::rows(&pad));
    let (off, span) = offsets(&rows);
    let classes: Vec<Class> =
        (1..=RATE).map(|lane| tie(&[(off[DEVICE], lane), (off[TAG], lane)])).collect();
    let groups = packed_groups(span, &classes, CAP);
    if !groups_enforce(&groups, &classes) {
        return None;
    }
    let mut regions: Vec<ShieldRegion> = chains.into_iter().map(ShieldRegion::Membership).collect();
    regions.push(ShieldRegion::Publics(pad));
    let kinds: Vec<usize> = (0..REGIONS).collect();
    let wired = WiredMultiGen::new_kinds(regions, &kinds, groups)
        .with_mask(MASK_COLUMNS)
        .with_ext_challenges();
    if Air::log_trace_len(&wired) != LOG_TRACE {
        return None;
    }
    Some(Built { wired, traces })
}
