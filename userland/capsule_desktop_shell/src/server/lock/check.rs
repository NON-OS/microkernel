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

//! Input while the lock screen or a passphrase prompt is up: all of it is
//! the lock's. On the lock screen a key edits the field and Enter sends it
//! to the keyring; the volume, brightness and lid keys still work, and
//! nothing else (no chord, no power prompt, no menu) does. What the router
//! sent while locked and is read after the unlock is the lock's too, and is
//! dropped (state/lock/released.rs).

use nonos_libc::{mk_uptime_ms, INPUT_KIND_KEY_DOWN};
use nonos_lock::wire::{answer, Answer};

use crate::protocol::{read_u16, read_u32};
use crate::state::lock::released::{is_locks, posted_ns, released_at};
use crate::state::lock::{Checked, Known};
use crate::state::system_key::{system_key, SystemKey};
use crate::state::Context;

const MOD_CTRL: u16 = nonos_app_skeleton::MOD_CTRL;
const MOD_ALT: u16 = 1 << 2;
const MOD_META: u16 = 1 << 3;

/// Some(true) when the lock took the frame: it is up and every input is
/// its own. None when neither the lock screen nor a prompt is up.
pub fn filter(ctx: &mut Context, kind: u16, buf: &[u8]) -> Option<bool> {
    if ctx.lock.screen.covers() {
        locked(ctx, kind, buf);
        return Some(true);
    }
    if ctx.lock.choose.is_some() {
        super::prompt::input(ctx, kind, buf);
        return Some(true);
    }
    if is_locks(ctx.lock.released_ns, posted_ns(buf)) {
        return Some(true);
    }
    None
}

fn locked(ctx: &mut Context, kind: u16, buf: &[u8]) {
    if kind != INPUT_KIND_KEY_DOWN {
        return;
    }
    let code = read_u32(buf, 12).unwrap_or(0);
    let flags = read_u16(buf, 10).unwrap_or(0);
    match system_key(code) {
        Some(SystemKey::Power) => return,
        Some(_) => {
            crate::server::handlers::system_keys::key(ctx, code);
            return;
        }
        None => {}
    }
    if flags & (MOD_CTRL | MOD_ALT | MOD_META) != 0 {
        return;
    }
    let now = mk_uptime_ms();
    match ctx.lock.screen.key(code, now) {
        crate::state::lock::Asked::Redraw => super::band::show(ctx),
        crate::state::lock::Asked::Check => submit(ctx),
        crate::state::lock::Asked::Nothing => {}
    }
}

/// The field to the keyring, and the screen after its answer.
fn submit(ctx: &mut Context) {
    // Drawn first: the derivation takes a moment, and the screen says so.
    super::band::show(ctx);
    let rid = ctx.issue_request_id();
    let reply = crate::keyring_client::check(rid, ctx.lock.screen.field.bytes());
    let checked = match reply.as_ref().map(|(s, b)| answer(*s, b)) {
        Some(Answer::Done(blob)) => {
            ctx.lock.verifier.took(blob);
            Checked::Right
        }
        Some(Answer::Wrong(c, blob)) => {
            ctx.lock.verifier.took(blob);
            Checked::Wrong { failures: c.failures, wait_ms: c.wait_ms }
        }
        Some(Answer::Wait(c)) => Checked::Wait { wait_ms: c.wait_ms },
        Some(Answer::NoRecord) => {
            // The keyring lost its record: the shell hands it back and the
            // next attempt is checked against it.
            let blob = ctx.lock.verifier.blob().to_vec();
            if ctx.lock.verifier.known() == Known::Set && !blob.is_empty() {
                super::start::load(ctx, &blob);
            }
            Checked::Unavailable
        }
        _ => Checked::Unavailable,
    };
    ctx.lock.screen.answered(checked, mk_uptime_ms());
    if !ctx.lock.screen.covers() {
        release(ctx);
        return;
    }
    // Still locked: the note under the field says why, in the band.
    super::band::show(ctx);
}

/// The right passphrase: the router lets go, the windows show again. The
/// curtain still shown leaves the trusted band first: an opaque twin of
/// itself, the move composes only where layers between the bands are (the
/// compositor's state/twin.rs), and the desktop's chrome is then presented
/// in the chrome's band. Moved after the repaint, the chrome's twin was
/// placed under it and taken from over it, each step the whole screen.
fn release(ctx: &mut Context) {
    super::router::tell(ctx);
    // Dated once the router has dated its own unlock, so nothing it sent the
    // shell as the lock's is read as the desktop's.
    ctx.lock.released_ns = Some(released_at(mk_uptime_ms()));
    ctx.lock.outputs.clear();
    crate::server::trusted_band::sync(ctx);
    crate::server::repaint::repaint(ctx);
    crate::server::grabs::sync(ctx);
}
