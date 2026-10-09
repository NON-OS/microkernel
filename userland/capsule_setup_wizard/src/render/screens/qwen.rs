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

/*
 * The Qwen step: which pinned tier the Terminal's qwen runs when no tier
 * is named, chosen among those that fit this machine's memory, by the
 * rule the store and the model fetcher hold a tier to. It starts on the
 * stick tier, Qwen3 0.6B, when that fits (qwen/default.rs).
 */

use crate::render;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::{lines, text::cat, text::size};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

use super::{mode, qwen_rows};
use crate::qwen::For;
use crate::render::layout::QWEN_AFTER_LINES;

/*
 * Setup runs from the release stick, which carries one tier, Qwen3 0.6B,
 * under its live plan (tools/nonos_seal/media.py): the kernel imports it
 * with no network, held to its signed SHA-256 pin. Every other tier is
 * downloaded, from Hugging Face unless this build names a NONOS mirror
 * (mk/22-models.mk), and held to its pin the same way.
 */
const WHERE: &[&[u8]] = &[
    b"qwen runs the tier chosen here. Qwen3 0.6B is on this stick and",
    b"installs offline; any other tier is downloaded and checked",
    b"against its signed SHA-256 pin.",
    b"The installed NONOS keeps the model on its disk.",
];

/* An amnesic stick: the model is held in memory for this session. */
const SESSION: &[&[u8]] = &[
    b"This stick keeps no disk of its own. Qwen3 0.6B is on this stick",
    b"and installs offline into memory for this session; any other tier",
    b"is downloaded into memory and checked against its signed SHA-256",
    b"pin. Either is gone at power off.",
];

/* No NONOS disk at all: nothing can hold a model. */
const NO_DISK: &[&[u8]] = &[
    b"This boot has no NONOS disk, so no model can be kept, not even",
    b"in memory, and none is chosen. Start from a NONOS stick, or",
    b"choose Install on the Mode step for the tier an install runs.",
];

/* The store has not settled: whether this boot has a disk is not known yet. */
const FINDING: &[&[u8]] =
    &[b"Still finding this boot's NONOS disk. The tiers open here as", b"soon as it has loaded."];

/* QEMU's software CPU: the last line says why a small tier is chosen. */
const TCG: &[u8] = b"QEMU's software CPU is slow: the smallest Qwen3 answers soonest.";

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Qwen model", b"Up and Down to choose a tier", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let room = l.line_rows(QWEN_AFTER_LINES, 2);
    let install = mode::keeps(ctx);
    let who = ctx.qwen.who(install);
    let y = qwen_rows::draw(buf, spx, w, h, x, l.body_y, &ctx.qwen, install, room);
    let (mut line, mut s) = ([0u8; 96], [0u8; 24]);
    let (head, rest): (&[u8], &[u8]) = match who {
        For::Installed => (
            b". A tier is listed when what it needs to run",
            b"leaves 1 GiB for the system, as the Store and qwen get hold it.",
        ),
        For::Session => (
            b". Held in memory, a tier is listed when its file",
            b"and its run fit in memory less the larger of 1 GiB and a quarter.",
        ),
        For::NoDisk => (b".", b""),
    };
    let memory: &[u8] = match ctx.qwen.memory {
        Some(m) => cat(&mut line, &[b"Memory: ", size(m, &mut s), head]),
        None => b"This machine's memory could not be read, so no tier fits.",
    };
    let rule: &[&[u8]] = &[memory, rest];
    let rule = if ctx.qwen.memory.is_some() && !rest.is_empty() { rule } else { &rule[..1] };
    let y = lines::text(buf, spx, w, h, x, y + l.gap, rule, HINT);
    let said: &[&[u8]] = match who {
        For::Installed if ctx.qwen.tcg => &[WHERE[0], WHERE[1], WHERE[2], TCG],
        For::Installed => WHERE,
        For::Session => SESSION,
        For::NoDisk if ctx.qwen.pending => FINDING,
        For::NoDisk => NO_DISK,
    };
    lines::text(buf, spx, w, h, x, y + l.gap, said, FG);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let install = mode::keeps(ctx);
    let mut row = ctx.qwen.row(install);
    if let Some(o) = list_nav(&mut row, ctx.qwen.open(install) + 1, code) {
        ctx.qwen.picked = Some(row);
        return o;
    }
    default_key(code)
}
