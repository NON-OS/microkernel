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

//! One desktop per thread: a display of a set size, a window manager that
//! places a window where it is asked, a compositor that counts what it is
//! told, and a clipboard of text. Every call is kept, in order, so a proof
//! reads back what the server asked of the desktop.

use std::cell::RefCell;

#[path = "desk_parts.rs"]
mod parts;
pub use parts::{layer_content_parts, present_layer_parts};

pub const COMPOSITOR: u32 = 10;
pub const WM: u32 = 11;
pub const INPUT_ROUTER: u32 = 12;
pub const CLIPBOARD: u32 = 13;

/// One call the server made on the desktop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    Damage(u32, u32, u32, u32),
    /// A swapchain present: the handle its layer showed, the handle it
    /// shows now, and the box that changed, in the buffer's pixels.
    Present(u64, u64, (u32, u32, u32, u32)),
    Submit(u64, u32, u32, u32, u32),
    /// A keyed layer: handle, x, y, width, height, window, the handle it
    /// replaces.
    Layer(u64, u32, u32, u32, u32, u32, u64),
    RemoveLayer(u64),
    /// A popup opened: its window, its parent, where it was placed.
    PopupOpen(u32, u32, (u32, u32, u32, u32)),
    Remove,
    Open(u32, u32, u32, u32, u32),
    Close(u32),
    Move(u32, u32, u32),
    Resize(u32, u32, u32),
    Maximize(u32, (u32, u32, u32, u32), bool),
    Minimize(u32),
    Restore(u32),
    Focus(u32),
    Raise(u32),
    Subscribe(u32),
    Cursor(u64, u8),
    /// The layer showing the first handle draws the second over its own,
    /// at the corner, the box that changed with it (OP_LAYER_CONTENT).
    /// The layer, the content, its corner, the box changed, the box drawn.
    Content(u64, u64, (u32, u32), (u32, u32, u32, u32), (u32, u32, u32, u32)),
    /// The boxes the present or content just before was told apart into,
    /// as x, y, width and height (OP_PRESENT_LAYER_PARTS,
    /// OP_LAYER_CONTENT_PARTS).
    Parts(Vec<(u32, u32, u32, u32)>),
}

/// OP_LAYER_CONTENT's request, as app_skeleton sends it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Content {
    pub on: u64,
    pub handle: u64,
    pub at: (u32, u32),
    pub opaque: bool,
    pub changed: (u32, u32, u32, u32),
    pub view: (u32, u32, u32, u32),
}

pub struct Desk {
    pub width: u32,
    pub height: u32,
    pub clipboard_on: bool,
    pub clipboard: Vec<u8>,
    pub calls: Vec<Call>,
    /// The keyed layers the compositor holds, bottom first.
    pub layers: Vec<KeyedLayer>,
    /// Each open window's rect, as the window manager keeps it.
    pub windows: Vec<(u32, (u32, u32, u32, u32))>,
    /// Whether the window manager places popups (OP_POPUP_OPEN).
    pub popups: bool,
    /// The refresh period OP_FRAME_TIME reports, 0 for none.
    pub refresh_ns: u64,
    /// Whether OP_FRAME_TIME reports frames shown: the frames shown so far,
    /// whether something was damaged since the last, and the time the next
    /// frame is shown at. While `covered`, nothing damaged is shown.
    pub frame_times: bool,
    pub covered: bool,
    pub shown: u64,
    pub damaged: bool,
    pub shown_ns: u64,
    /// The window with the keyboard, as the window manager keeps it.
    pub focused: Option<u32>,
    /// Buffers a present took off screen, held until the compositor
    /// composes (`compose`); with `hold` off it composes at once.
    pub held: Vec<u64>,
    /// Each layer's content: the handle the layer shows, and the one drawn
    /// over it.
    pub contents: Vec<(u64, u64)>,
    /// Whether each OP_LAYER_CONTENT taken named its content opaque, in turn.
    pub content_opaque: Vec<bool>,
    /// Every OP_LAYER_CONTENT refused, the rest of the protocol served.
    pub refuse_content: bool,
    pub hold: bool,
    /// Presents refused, as a full held list or a lost layer would be.
    pub refuse: bool,
    /// The display driver OP_SCANOUT_PEER names, 0 for none.
    pub scanout_peer: u32,
}

impl Default for Desk {
    fn default() -> Desk {
        Desk {
            width: 1280,
            height: 800,
            clipboard_on: false,
            clipboard: Vec::new(),
            calls: Vec::new(),
            layers: Vec::new(),
            windows: Vec::new(),
            popups: true,
            refresh_ns: 0,
            frame_times: false,
            covered: false,
            shown: 0,
            damaged: false,
            shown_ns: 0,
            focused: None,
            held: Vec::new(),
            contents: Vec::new(),
            content_opaque: Vec::new(),
            refuse_content: false,
            hold: false,
            refuse: false,
            scanout_peer: 0,
        }
    }
}

thread_local! {
    static DESK: RefCell<Desk> = RefCell::new(Desk::default());
}

pub fn with<R>(f: impl FnOnce(&mut Desk) -> R) -> R {
    DESK.with(|d| f(&mut d.borrow_mut()))
}

fn call(c: Call) -> Result<(), &'static str> {
    with(|d| {
        d.damaged |=
            matches!(c, Call::Damage(..) | Call::Submit(..) | Call::Layer(..) | Call::Present(..));
        d.calls.push(c)
    });
    Ok(())
}

pub fn port_of(service: &[u8]) -> Option<u32> {
    match service {
        b"compositor" => Some(COMPOSITOR),
        b"wm" => Some(WM),
        b"input_router" => Some(INPUT_ROUTER),
        b"clipboard" => with(|d| d.clipboard_on).then_some(CLIPBOARD),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameTime {
    pub seq: u64,
    pub at_ns: u64,
    pub refresh_ns: u64,
}

/// OP_FRAME_TIME, as a compositor answers it: the frames shown so far and
/// when the last was, at the refresh the desk says. What was damaged is
/// shown just after the answer, as a compositor composes after it is asked;
/// the next ask finds it. Without frame times or a refresh, it has none.
pub fn frame_time(_port: u32, _request_id: u32) -> Result<FrameTime, &'static str> {
    with(|d| {
        if !d.frame_times && d.refresh_ns == 0 {
            return Err("no frame time");
        }
        let now = FrameTime { seq: d.shown, at_ns: d.shown_ns, refresh_ns: d.refresh_ns };
        if d.frame_times && d.damaged && !d.covered {
            (d.shown, d.damaged) = (d.shown + 1, false);
        }
        Ok(now)
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyedLayer {
    pub surface_handle: u64,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub z: u32,
    pub window_id: u32,
    pub replaces: u64,
}

/// OP_SCENE_SUBMIT_LAYER as the compositor keeps it: the same handle moves
/// in place, `replaces` takes that layer's place in the stack, a new one
/// goes on top; a replace of a layer not held is refused.
pub fn scene_submit_layer(_p: u32, _r: u32, l: &KeyedLayer) -> Result<(), &'static str> {
    let KeyedLayer {
        surface_handle: h, x, y, width: w, height: ht, window_id: id, replaces, ..
    } = *l;
    call(Call::Layer(h, x, y, w, ht, id, replaces))?;
    with(|d| {
        let held = |k: u64| d.layers.iter().position(|o| o.surface_handle == k);
        match (held(h), replaces) {
            (Some(at), _) => d.layers[at] = *l,
            (None, 0) => d.layers.push(*l),
            (None, r) => match held(r) {
                Some(at) => d.layers[at] = *l,
                None => return Err("compositor holds no such layer to replace"),
            },
        }
        Ok(())
    })
}

pub fn scene_remove_layer(_p: u32, _r: u32, handle: u64, _z: u32) -> Result<(), &'static str> {
    call(Call::RemoveLayer(handle))?;
    with(|d| {
        d.layers.retain(|l| l.surface_handle != handle);
        d.contents.retain(|c| c.0 != handle);
    });
    Ok(())
}

/// OP_LAYER_CONTENT: refused for a layer the compositor does not hold, or a
/// content another layer shows or draws; the content it replaces is held
/// until the next frame, as a present's old buffer is.
pub fn layer_content(_p: u32, _r: u32, c: &Content) -> Result<(), &'static str> {
    let refused = with(|d| {
        let elsewhere = d.layers.iter().any(|l| l.surface_handle == c.handle)
            || d.contents.iter().any(|k| k.1 == c.handle && k.0 != c.on);
        let gone = !d.layers.iter().any(|l| l.surface_handle == c.on);
        d.refuse || d.refuse_content || gone || (c.handle != 0 && elsewhere)
    });
    if refused {
        return Err("compositor refused the layer's content");
    }
    call(Call::Content(c.on, c.handle, c.at, c.changed, c.view))?;
    with(|d| {
        d.content_opaque.push(c.opaque);
        let old = d.contents.iter().find(|k| k.0 == c.on).map(|k| k.1);
        d.contents.retain(|k| k.0 != c.on);
        if c.handle != 0 {
            d.contents.push((c.on, c.handle));
        }
        if let Some(old) = old.filter(|&o| o != c.handle) {
            d.held.push(old);
        }
    });
    if !with(|d| d.hold) {
        compose();
    }
    Ok(())
}

pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
}

pub fn display_info(_port: u32, _request_id: u32) -> Result<DisplayInfo, &'static str> {
    with(|d| Ok(DisplayInfo { width: d.width, height: d.height }))
}

pub fn damage_commit(_p: u32, _r: u32, x: u32, y: u32, w: u32, h: u32) -> Result<(), &'static str> {
    call(Call::Damage(x, y, w, h))
}

pub fn scene_submit(
    _port: u32,
    _request_id: u32,
    surface_handle: u64,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    _z: u32,
) -> Result<(), &'static str> {
    call(Call::Submit(surface_handle, x, y, width, height))
}

pub fn cursor_shape(
    _port: u32,
    _r: u32,
    surface_handle: u64,
    code: u8,
) -> Result<(), &'static str> {
    call(Call::Cursor(surface_handle, code))
}

pub fn scene_remove(_port: u32, _request_id: u32, _owner_pid: u32) -> Result<(), &'static str> {
    call(Call::Remove)
}

#[derive(Clone, Copy)]
pub struct WindowPlacement {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn window_open(
    _port: u32,
    _request_id: u32,
    window_id: u32,
    _kind: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<WindowPlacement, &'static str> {
    call(Call::Open(window_id, x, y, width, height))?;
    set_rect(window_id, |_| (x, y, width, height));
    Ok(WindowPlacement { x, y, width, height })
}

/// The rect the window manager keeps for `window_id`, changed by `to`.
fn set_rect(window_id: u32, to: impl Fn((u32, u32, u32, u32)) -> (u32, u32, u32, u32)) {
    with(|d| match d.windows.iter_mut().find(|w| w.0 == window_id) {
        Some(w) => w.1 = to(w.1),
        None => d.windows.push((window_id, to((0, 0, 0, 0)))),
    });
}

/// A focused window closed gives the keyboard to the newest one left, as the
/// window manager focuses the window under it.
pub fn window_close(_port: u32, _r: u32, window_id: u32) -> Result<(), &'static str> {
    let next = with(|d| {
        d.windows.retain(|w| w.0 != window_id);
        let was = d.focused == Some(window_id);
        if was {
            d.focused = None;
        }
        d.windows.last().map(|w| w.0).filter(|_| was)
    });
    if let Some(next) = next {
        keys_notice(next, true);
    }
    call(Call::Close(window_id))
}

pub use crate::wm_positioner::{Placed, Positioner};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopupSpec {
    pub window_id: u32,
    pub parent_id: u32,
    pub anchor_x: i32,
    pub anchor_y: i32,
    pub anchor_w: u32,
    pub anchor_h: u32,
    pub width: u32,
    pub height: u32,
    pub anchor: u32,
    pub gravity: u32,
    pub adjust: u32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub flags: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopupPlacement {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rel_x: i32,
    pub rel_y: i32,
}

pub const POPUP_GRAB: u32 = 1;

/// OP_POPUP_OPEN, placed by the window manager's own positioner
/// (capsule_wm geometry/positioner.rs) against its parent's rect, on the
/// whole display; refused for a parent it does not hold, or when the desk
/// says the window manager places no popups.
pub fn popup_open(_p: u32, _r: u32, s: &PopupSpec) -> Result<PopupPlacement, &'static str> {
    let (parent, display, places) = with(|d| {
        let parent = d.windows.iter().find(|w| w.0 == s.parent_id).map(|w| w.1);
        (parent, (d.width, d.height), d.popups)
    });
    let Some((x, y, w, h)) = parent.filter(|_| places) else {
        return Err("wm rejected popup_open");
    };
    let rect = |x, y, width, height| crate::wm_rect::Rect { x, y, width, height };
    let positioner = Positioner {
        anchor_x: s.anchor_x,
        anchor_y: s.anchor_y,
        anchor_w: s.anchor_w,
        anchor_h: s.anchor_h,
        width: s.width,
        height: s.height,
        anchor: s.anchor,
        gravity: s.gravity,
        adjust: s.adjust,
        offset_x: s.offset_x,
        offset_y: s.offset_y,
    };
    let placed = crate::wm_positioner::place(
        &positioner,
        rect(x, y, w, h),
        rect(0, 0, display.0, display.1),
    );
    let r = placed.rect;
    let at = (r.x, r.y, r.width, r.height);
    set_rect(s.window_id, |_| at);
    call(Call::PopupOpen(s.window_id, s.parent_id, at))?;
    Ok(PopupPlacement {
        x: r.x,
        y: r.y,
        width: r.width,
        height: r.height,
        rel_x: placed.rel_x,
        rel_y: placed.rel_y,
    })
}

/// The window manager's notice that it closed popup `window_id`
/// (wm.notify_v1, kind 5), put in the project's inbox.
pub fn popup_done_notice(window_id: u32) {
    let mut f = Vec::with_capacity(28);
    f.extend_from_slice(&0x4E57_4D56u32.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    for word in [5, 0, window_id, 0, 0] {
        f.extend_from_slice(&word.to_le_bytes());
    }
    nonos_libc::world::post(WM, f);
}

/// The popup a window manager notice says it closed (wm.notify_v1 kind 5).
pub fn popup_done(msg: &[u8]) -> Option<u32> {
    let word = |at: usize| Some(u32::from_le_bytes(msg.get(at..at + 4)?.try_into().ok()?));
    (msg.len() == 28 && word(0)? == 0x4E57_4D56 && word(8)? == 5).then(|| word(16)).flatten()
}

pub fn window_move(
    _port: u32,
    _r: u32,
    window_id: u32,
    x: u32,
    y: u32,
) -> Result<(), &'static str> {
    set_rect(window_id, |(_, _, w, h)| (x, y, w, h));
    call(Call::Move(window_id, x, y))
}

pub fn window_resize(
    _port: u32,
    _r: u32,
    window_id: u32,
    w: u32,
    h: u32,
) -> Result<(), &'static str> {
    set_rect(window_id, |(x, y, _, _)| (x, y, w, h));
    call(Call::Resize(window_id, w, h))
}

pub fn window_maximize(
    _port: u32,
    _r: u32,
    window_id: u32,
    rect: (u32, u32, u32, u32),
    full_screen: bool,
) -> Result<(), &'static str> {
    set_rect(window_id, |_| rect);
    call(Call::Maximize(window_id, rect, full_screen))
}

pub fn window_minimize(_port: u32, _r: u32, window_id: u32) -> Result<(), &'static str> {
    call(Call::Minimize(window_id))
}

/// A minimised window shown again (present/hide.rs restore).
pub fn window_restore(_port: u32, _r: u32, window_id: u32) -> Result<(), &'static str> {
    call(Call::Restore(window_id))
}

/// Focused, the window that had the keyboard is told it lost it, then the
/// new one that it has it, as the window manager tells them (capsule_wm
/// focus/notice.rs keys_notices).
pub fn window_focus(_port: u32, _r: u32, window_id: u32) -> Result<(), &'static str> {
    if let Some(was) = with(|d| d.focused).filter(|&w| w != window_id) {
        keys_notice(was, false);
    }
    keys_notice(window_id, true);
    call(Call::Focus(window_id))
}

/// The window manager's notice that `window_id` has the keyboard or lost it
/// (abi/wire.toml [wm.notify_v1], kind 4), put in the project's inbox.
pub fn keys_notice(window_id: u32, has: bool) {
    with(|d| match has {
        true => d.focused = Some(window_id),
        false if d.focused == Some(window_id) => d.focused = None,
        false => {}
    });
    let mut f = Vec::with_capacity(28);
    f.extend_from_slice(&0x4E57_4D56u32.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    for word in [4, 0, window_id, has as u32, 0] {
        f.extend_from_slice(&word.to_le_bytes());
    }
    nonos_libc::world::post(WM, f);
}

pub fn window_raise(_port: u32, _r: u32, window_id: u32) -> Result<(), &'static str> {
    call(Call::Raise(window_id))
}

pub fn subscribe(_port: u32, _request_id: u32, kind_mask: u32) -> Result<(), &'static str> {
    call(Call::Subscribe(kind_mask))
}

pub fn clipboard_copy(text: &[u8]) -> Result<(), &'static str> {
    with(|d| d.clipboard = text.to_vec());
    Ok(())
}

pub fn clipboard_paste(out: &mut [u8]) -> Result<usize, &'static str> {
    with(|d| {
        let n = d.clipboard.len().min(out.len());
        out[..n].copy_from_slice(&d.clipboard[..n]);
        Ok(n)
    })
}

pub fn clipboard_clear() -> Result<(), &'static str> {
    with(|d| d.clipboard.clear());
    Ok(())
}

/// OP_PRESENT_LAYER as the compositor keeps it (state/present_rule.rs
/// present_on): the layer showing `on` shows the body's handle; the buffer
/// it showed is held, then released when the compositor composes. Refused
/// for a layer not held, or a buffer another layer shows.
pub fn present_layer(
    _p: u32,
    _r: u32,
    body: &[u8; crate::swap::PRESENT_LAYER_LEN],
) -> Result<(), &'static str> {
    let word = |at: usize| u32::from_le_bytes([body[at], body[at + 1], body[at + 2], body[at + 3]]);
    let wide = |at: usize| u64::from(word(at)) | u64::from(word(at + 4)) << 32;
    let (handle, on) = (wide(0), wide(32));
    let area = (word(16), word(20), word(24), word(28));
    let refused = with(|d| {
        let taken = d.layers.iter().any(|l| l.surface_handle == handle && handle != on);
        d.refuse || taken || !d.layers.iter().any(|l| l.surface_handle == on)
    });
    if refused {
        return Err("compositor rejected present");
    }
    call(Call::Present(on, handle, area))?;
    with(|d| {
        if let Some(l) = d.layers.iter_mut().find(|l| l.surface_handle == on) {
            l.surface_handle = handle;
        }
        // The content stays on the layer, now named by its new handle.
        if let Some(k) = d.contents.iter_mut().find(|k| k.0 == on) {
            k.0 = handle;
        }
        if on != handle {
            d.held.push(on);
        }
    });
    if !with(|d| d.hold) {
        compose();
    }
    Ok(())
}

/// The compositor composed a frame: every buffer it held and no layer
/// shows is released (frame_pacer/release.rs).
pub fn compose() {
    let gone = with(|d| {
        let layers = &d.layers;
        let drawn = |h: u64| {
            layers.iter().any(|l| l.surface_handle == h) || d.contents.iter().any(|k| k.1 == h)
        };
        let (gone, kept): (Vec<u64>, Vec<u64>) = d.held.iter().partition(|h| !drawn(**h));
        d.held = kept;
        gone
    });
    for h in gone {
        let _ = nonos_libc::world::fence_release(h);
    }
}

/// OP_SCANOUT_PEER: the display driver the desk names, none by default.
pub fn scanout_peer(_port: u32, _request_id: u32) -> Result<u32, &'static str> {
    Ok(with(|d| d.scanout_peer))
}
