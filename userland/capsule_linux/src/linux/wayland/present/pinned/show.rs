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

//! A commit shown from its pinned buffer, and the window's way back to its
//! own buffers. Shown so, the window's own buffer holds its frame and the
//! frame's ground where the client's pixels go, and the compositor draws
//! the client's buffer over that (OP_LAYER_CONTENT): what the copy path
//! made by flattening those pixels over the same ground (frame/fill.rs).

use nonos_app_skeleton::clients::compositor::{layer_content, layer_content_parts, Content};
use nonos_app_skeleton::discover::{lookup_port, lookup_service};
use nonos_toolkit::decorations::frame_bg;

use crate::linux::guest::Guest;

use super::super::super::commit::damage_show::damage::Damage;
use super::super::super::commit::damage_show::damage_boxes::{Boxes, MAX_BOXES};
use super::super::super::region::opaque_whole;
use super::super::super::reshape::{reshape, Reshape, Shape};
use super::super::draw::{draw, whole, Drew};
use super::super::frame::fill::window_bytes;
use super::super::frame::geometry::Around;
use super::super::frame::now::around_shape;
use super::super::frame::redraw::redraw;
use super::super::scaled;
use super::held::{count, now, Held};
use super::pins::spare;
use super::rule::{paced, pinned, run, Facts};

// Every box kept apart fits one OP_LAYER_CONTENT_PARTS.
const _: () = assert!(MAX_BOXES <= nonos_app_skeleton::swap::MAX_PARTS);

/// What showing a commit from its pin takes.
#[derive(Clone, Copy)]
struct Plan {
    a: Around,
    /// The shape shown: the window geometry's box of the buffer, or all of it.
    shape: Shape,
    /// The buffer's own shape, which its pin holds whole.
    own: Shape,
    /// The box's corner in the buffer's pixels: (0, 0) for a buffer shown
    /// whole, else where the window starts inside the shadow its client
    /// draws round it.
    cut: (u32, u32),
    /// The bytes of the window's own buffer (frame/fill.rs window_bytes).
    bytes: usize,
    opaque: bool,
    span: (u64, u64),
}

/// How a commit this path took was shown.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Taken {
    pub drew: Drew,
    /// Drawn whole into the window's own buffers by the copy path, so the
    /// buffer was read out of the guest whole.
    pub copied: bool,
}

/// `buffer` committed on the window `surface`, `said` of it changed. Shown
/// from its pin when rule.rs allows and the kernel pins it: Drew::Held, and
/// the window keeps the buffer until the compositor lets it go (held.rs).
/// A window that showed a pin until now and cannot show this one so is
/// drawn whole into its own buffers first. None: the copy path shows it.
pub fn present(
    guest: &mut Guest,
    surface: u32,
    buffer: u32,
    said: Damage,
    boxes: &Boxes,
) -> Option<Taken> {
    let win = &mut guest.scene.win;
    if win.committed != Some(buffer) {
        win.turn = win.committed.replace(buffer);
    }
    let at = now();
    let win = &guest.scene.win;
    let run = run(win.run, win.own_last.or(win.idle_back), at);
    let taken = match plan(guest, surface, buffer, (at, run))
        .and_then(|p| show(guest, (surface, buffer), p, (said, boxes)))
    {
        Some(taken) => Some(taken),
        None => guest.scene.win.content.is_some().then(|| copied(leave(guest, surface, buffer))),
    };
    let win = &mut guest.scene.win;
    let own = win.content.is_none().then_some(at);
    win.idle_back = win.idle_back.and(own);
    win.own_last = own;
    win.run = if own.is_some() { run } else { 0 };
    taken
}

/// The window that showed one buffer for rule.rs IDLE_MS, or whose
/// subsurfaces changed, draws it into its own buffers and gives it back.
pub fn give_back(guest: &mut Guest) -> Drew {
    let (Some(h), Some(shown)) = (guest.scene.win.content, guest.scene.win.shown) else {
        return Drew::Nothing;
    };
    leave(guest, shown.surface, h.buffer)
}

fn plan(guest: &mut Guest, surface: u32, buffer: u32, (at, run): (u64, u32)) -> Option<Plan> {
    let (scene, spans) = (&mut guest.scene, &guest.memfd_spans);
    let on =
        scene.win.shown.as_ref().is_some_and(|s| s.surface == surface) && scene.win.out.is_some();
    let layered = !scene.win.base.is_empty() || scene.subs.shown_on(surface);
    let owes = scene.win.owing.is_some() || scene.win.fit.due().is_some() || scene.win.minimized;
    let src = scaled::window(scene, spans, surface, buffer).ok()?;
    let a = around_shape(scene, src.shape);
    let key = (a, scene.win.frame.hover, scene.win.focused);
    let same_shape = reshape(scene.win.shape, src.shape) == Reshape::Same
        && scene.win.origin == src.origin
        && scene.win.bufs.shape() == a.laid(src.shape);
    // At its own size: a window geometry's box of it is drawn from the pin
    // as that box (Content view), a scaled one only by the copy.
    let plain = src.raw.is_none();
    let span = super::span::span(scene, spans, buffer);
    let own = scene.buffers.iter().find(|b| b.id == buffer).map(|b| (b.width, b.height, b.stride));
    let f = Facts {
        on,
        layered,
        owes,
        same_shape,
        chrome_current: !a.framed() || scene.win.frame.chrome == Some(key),
        plain,
        pinnable: span.is_some() && own.is_some(),
        paced: paced(
            scene.win.idle_back,
            scene.win.own_last,
            run,
            at,
            spare(scene, &scene.win, buffer),
        ),
    };
    // XRGB, or an opaque region over the whole surface: its alpha unread.
    // The region is held to the surface from its corner, so a box cut from
    // inside it is taken as blended unless the buffer is XRGB.
    let whole = src.cut == (0, 0);
    let opaque = src.opaque || (whole && opaque_whole(scene, surface, (src.shape.0, src.shape.1)));
    let bytes = window_bytes(a, &src);
    let (shape, cut) = (src.shape, src.cut);
    pinned(&f).then_some(Plan { a, shape, own: own?, cut, bytes, opaque, span: span? })
}

fn show(
    guest: &mut Guest,
    (surface, buffer): (u32, u32),
    p: Plan,
    (said, boxes): (Damage, &Boxes),
) -> Option<Taken> {
    let reader = lookup_service(b"compositor")?.pid;
    let port = lookup_port(b"compositor")?;
    let t = now();
    let peers = (guest.pid, reader, port);
    let handle = super::pins::pin(&mut guest.scene, peers, buffer, p.span, p.own, t)?;
    let seen = u32::try_from(count(handle)).ok()?;
    let was = guest.scene.win.content;
    // A ground that did not go up may have reached the kept client pixels:
    // the window is drawn whole, as the copy draws a window new.
    if was.is_none() && !ground(guest, p) {
        return Some(copied(copied_whole(guest, surface, buffer)));
    }
    let (w, h, _) = p.shape;
    // Said in the buffer's pixels; told in the box's, as the copy reads it.
    let said = said.moved(p.cut).on(w, h);
    let changed = changed(said, was.map(|c| c.buffer), buffer, (w, h));
    let Some(changed) = changed else { return Some(held()) };
    let on = guest.scene.win.out?;
    let at = if p.a.framed() { (p.a.cx, p.a.cy) } else { (0, 0) };
    let view = (p.cut.0, p.cut.1, w, h);
    let c = Content { on, handle, at, opaque: p.opaque, changed, view };
    let rid = guest.scene.next_serial();
    // Two boxes or more said apart inside the box: each told alone, so the
    // rows between them are not composed again.
    let apart = match (said, was) {
        (Damage::Rect(x0, y0, x1, y1), Some(_)) => boxes.apart(p.cut, (x0, y0, x1, y1)),
        _ => None,
    };
    let told = match apart.map(|b| b.areas()) {
        Some((parts, n)) => layer_content_parts(port, rid, &c, &parts[..n]),
        None => layer_content(port, rid, &c),
    };
    if told.is_err() {
        // Refused over the ground just drawn: drawn whole instead. A window
        // that showed a pin goes back to its own buffers (present, leave).
        return was.is_none().then(|| copied(copied_whole(guest, surface, buffer)));
    }
    let scene = &mut guest.scene;
    scene.releasing.retain(|r| r.buffer != buffer);
    scene.win.content = Some(Held { buffer, handle, seen, since: t });
    if let Some(prev) = was.filter(|c| c.buffer != buffer) {
        scene.releasing.push(Held { since: t, ..prev });
    }
    Some(held())
}

/// The box of a window `w` x `h` the compositor draws again when the layer
/// that drew `was` draws `buffer`, `said` of it changed; None when nothing
/// did. A layer that drew no buffer until now draws it all. A client's
/// other buffer, of the window's shape (rule.rs same_shape), is a
/// swapchain's next: what it said changed is told against what the window
/// showed, as wl_surface.damage is, so that box alone, as a present of the
/// compositor's own does. A new buffer said nothing of is drawn whole.
pub fn changed(
    said: Damage,
    was: Option<u32>,
    buffer: u32,
    (w, h): (u32, u32),
) -> Option<(u32, u32, u32, u32)> {
    match (said, was) {
        (_, None) => Some((0, 0, w, h)),
        (Damage::None, Some(b)) if b == buffer => None,
        (Damage::Rect(x0, y0, x1, y1), Some(_)) => Some((x0, y0, x1 - x0, y1 - y0)),
        (Damage::None | Damage::Whole, Some(_)) => Some((0, 0, w, h)),
    }
}

fn held() -> Taken {
    Taken { drew: Drew::Held, copied: false }
}

fn copied(drew: Drew) -> Taken {
    Taken { drew, copied: true }
}

/// The window's own buffer drawn with its frame and the frame's ground
/// where the client's pixels go: what a pin is shown over.
fn ground(guest: &mut Guest, p: Plan) -> bool {
    let scene = &mut guest.scene;
    let (w, h, stride) = p.shape;
    if p.a.framed() {
        let frame = &mut scene.win.frame;
        frame.client_shape = None;
        // Every row a whole stride: what paint.rs reads of the client's.
        let rows = (stride as usize).checked_mul(h as usize);
        let Some(client) = rows.and_then(|n| frame.client_room(n)) else { return false };
        fill_ground(client, (w, h, stride));
        frame.client_shape = Some(p.shape);
        frame.chrome = None;
        return redraw(scene) == Drew::Shown;
    }
    let all = whole((w, h));
    let drawn = draw(scene, p.bytes, all, |_, out| {
        fill_ground(out, (w, h, stride));
        Some(all)
    });
    drawn == Drew::Shown
}

fn fill_ground(px: &mut [u8], (w, h, stride): Shape) {
    let ground = frame_bg().to_le_bytes();
    for row in px.chunks_mut(stride as usize).take(h as usize) {
        let n = (w as usize * 4).min(row.len());
        for p in row[..n].chunks_exact_mut(4) {
            p.copy_from_slice(&ground);
        }
    }
}

/// `buffer` drawn whole into the window's own buffers by the copy path.
fn copied_whole(guest: &mut Guest, surface: u32, buffer: u32) -> Drew {
    guest.scene.win.frame.chrome = None;
    super::super::show::present(guest, surface, buffer)
}

/// Back to the window's own buffers: `buffer` drawn whole into them by the
/// copy path, then the layer told to draw no content. The buffer the layer
/// drew is given back once the compositor lets it go; one the compositor
/// still draws (the clear refused) stays the window's until a later commit
/// clears it.
fn leave(guest: &mut Guest, surface: u32, buffer: u32) -> Drew {
    let drew = copied_whole(guest, surface, buffer);
    let (Some(port), Some(on), Some(held)) =
        (lookup_port(b"compositor"), guest.scene.win.out, guest.scene.win.content)
    else {
        return drew;
    };
    let (w, h) =
        (guest.scene.win.shape.map_or(0, |s| s.0), guest.scene.win.shape.map_or(0, |s| s.1));
    let (changed, view) = ((0, 0, w, h), (0, 0, 0, 0));
    let c = Content { on, handle: 0, at: (0, 0), opaque: false, changed, view };
    let rid = guest.scene.next_serial();
    if drew == Drew::Shown && layer_content(port, rid, &c).is_ok() {
        guest.scene.win.content = None;
        guest.scene.releasing.push(Held { since: now(), ..held });
    }
    drew
}

/// The client destroyed `buffer`: a window its layer draws it on is drawn
/// into its own buffers from it first, while it is there to read, and its
/// pin is given up. It is given back to nobody: it is gone.
pub fn buffer_gone(guest: &mut Guest, buffer: u32) {
    let scene = &guest.scene;
    let windows = core::iter::once(&scene.win).chain(scene.behind.iter());
    let on = |w: &&super::super::super::scene_window::Window| {
        w.content.is_some_and(|h| h.buffer == buffer)
    };
    if let Some(id) = windows.filter(on).find_map(|w| w.window) {
        if super::super::super::window_front::select_window(guest, id) {
            let _ = give_back(guest);
        }
    }
    guest.scene.releasing.retain(|h| h.buffer != buffer);
    super::pins::unpin(&mut guest.scene, guest.pid, buffer);
}
