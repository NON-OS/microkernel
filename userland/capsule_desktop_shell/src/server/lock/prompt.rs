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

//! The prompt that chooses or changes the passphrase, in the trusted band
//! like the lock screen: no window can draw over it or type into it. The
//! passphrase goes from the field to the keyring and nowhere else. Input
//! posted while it was up and read after it closed, typed while the keyring
//! took the new passphrase, is the prompt's and is dropped
//! (state/lock/released.rs).

use nonos_libc::{
    mk_keeping, mk_uptime_ms, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_KEY_DOWN, INPUT_KIND_TOUCH,
};
use nonos_lock::wire::{answer, Answer};

use crate::protocol::{read_i32, read_u16, read_u32};
use crate::server::repaint::repaint;
use crate::state::lock::choose::ChooseNote;
use crate::state::lock::geometry::{inside, prompt};
use crate::state::lock::released::released_at;
use crate::state::lock::{words, Choose, Chosen, Purpose};
use crate::state::{Context, NotifyLevel};

const MOD_CTRL: u16 = nonos_app_skeleton::MOD_CTRL;

/// Ask for a first passphrase: once a boot, where none is set.
pub fn ask_first(ctx: &mut Context) {
    ctx.lock.verifier.asked();
    open(ctx, Purpose::First);
}

/// Settings asked: change the passphrase, or choose one where none is set.
pub fn ask_change(ctx: &mut Context) {
    if ctx.lock.screen.covers() {
        return;
    }
    let purpose = if ctx.lock.offered() { Purpose::Change } else { Purpose::First };
    open(ctx, purpose);
}

fn open(ctx: &mut Context, purpose: Purpose) {
    crate::server::handlers::escape_chord::close_chrome(ctx);
    let rid = ctx.issue_request_id();
    ctx.lock.outputs =
        crate::compositor_client::output_rects::output_rects(ctx.ports.compositor_port, rid)
            .unwrap_or_else(|| alloc::vec![(0, 0, ctx.width, ctx.height)]);
    ctx.lock.choose = Some(Choose::new(purpose));
    repaint(ctx);
}

fn close(ctx: &mut Context) {
    ctx.lock.choose = None;
    ctx.lock.released_ns = Some(released_at(mk_uptime_ms()));
    ctx.lock.outputs.clear();
    repaint(ctx);
}

pub(super) fn input(ctx: &mut Context, kind: u16, buf: &[u8]) {
    let asked = match kind {
        INPUT_KIND_KEY_DOWN => {
            if read_u16(buf, 10).unwrap_or(0) & MOD_CTRL != 0 {
                return;
            }
            let code = read_u32(buf, 12).unwrap_or(0);
            ctx.lock.choose.as_mut().map_or(Chosen::Nothing, |c| c.key(code))
        }
        INPUT_KIND_BUTTON_DOWN | INPUT_KIND_TOUCH => press(ctx, buf),
        _ => Chosen::Nothing,
    };
    match asked {
        Chosen::Nothing => {}
        Chosen::Redraw => repaint(ctx),
        Chosen::Cancel => close(ctx),
        Chosen::Save => save(ctx),
    }
}

fn press(ctx: &mut Context, buf: &[u8]) -> Chosen {
    let (x, y) = (read_i32(buf, 16).unwrap_or(-1), read_i32(buf, 20).unwrap_or(-1));
    let (Ok(x), Ok(y)) = (u32::try_from(x), u32::try_from(y)) else {
        return Chosen::Nothing;
    };
    let out = ctx.lock.outputs.first().copied().unwrap_or((0, 0, ctx.width, ctx.height));
    let p = prompt(out, crate::render::ui_font::px(1));
    let Some(choose) = ctx.lock.choose.as_mut() else {
        return Chosen::Nothing;
    };
    if inside(p.primary, x, y) {
        choose.enter()
    } else if inside(p.secondary, x, y) {
        Chosen::Cancel
    } else {
        Chosen::Nothing
    }
}

/// Current and new to the keyring; the prompt closes when it took them.
fn save(ctx: &mut Context) {
    repaint(ctx);
    let rid = ctx.issue_request_id();
    let Some(choose) = ctx.lock.choose.as_ref() else { return };
    let reply = crate::keyring_client::set(rid, choose.current.bytes(), choose.new.bytes());
    let note = match reply.as_ref().map(|(s, b)| answer(*s, b)) {
        Some(Answer::Done(blob)) => {
            ctx.lock.verifier.took(blob);
            let said = if mk_keeping() { words::CHANGED } else { words::KEPT_THIS_BOOT };
            ctx.toasts.push(said, NotifyLevel::Info, crate::server::toast_clock::now());
            close(ctx);
            crate::render::sync_toast_layer(ctx);
            return;
        }
        Some(Answer::Wrong(_, blob)) => {
            ctx.lock.verifier.took(blob);
            ChooseNote::Wrong
        }
        Some(Answer::Wait(_)) => ChooseNote::Wait,
        _ => ChooseNote::Failed,
    };
    if let Some(c) = ctx.lock.choose.as_mut() {
        c.refused(note);
    }
    repaint(ctx);
}
