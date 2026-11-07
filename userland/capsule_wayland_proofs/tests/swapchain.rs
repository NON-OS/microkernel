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

//! A guest's window on two buffers (scene_buffers.rs, present/draw.rs):
//! each commit drawn into the buffer the compositor is not showing, brought
//! up to the picture first, then presented on the window's own layer. The
//! compositor double holds the buffer a present replaced until it composes
//! (desk.rs), as the compositor's release fence does.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::linux::call::{family_ms, set_clock};

const BUFFER_RELEASE: u16 = 0;
const W: i32 = 320;
const H: i32 = 200;
const BLUE: u32 = 0xFF20_40A0;

/// A toplevel's first configure acked and `W` by `H` of blue committed.
fn window() -> Client {
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("swap")]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (W, H), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// Buffer `n` (its own pool) of `argb` attached with the box `b` said
/// damaged, and committed; the events after.
fn commit_box(c: &mut Client, n: u32, argb: u32, b: (i32, i32, i32, i32)) -> Vec<Ev> {
    let (pool, buffer) = (100 + 2 * n, 101 + 2 * n);
    c.painted(SHM, pool, buffer, (W, H), argb);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events()
}

/// Buffer `n` holding `px`, the guest's whole picture, attached with only
/// the box `b` said damaged, as a client that redraws its changes does;
/// the events after.
fn commit_picture(c: &mut Client, n: u32, px: &[u32], b: (i32, i32, i32, i32)) -> Vec<Ev> {
    let (pool, buffer) = (100 + 2 * n, 101 + 2 * n);
    let bytes: Vec<u8> = px.iter().flat_map(|p| p.to_le_bytes()).collect();
    // SAFETY: a new memfd, written and sized, handed to the session with
    // the request.
    let fd = unsafe {
        let fd = libc::memfd_create(c"picture".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, bytes.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, bytes.as_ptr().cast(), bytes.len()), bytes.len() as isize);
        fd
    };
    c.req_fds(SHM, 0, &[A::U(pool), A::I(bytes.len() as i32)], &[fd]);
    c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(W), A::I(H), A::I(W * 4), A::U(0)]);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events()
}

/// Where the guest's pixels start in the shown window, frame and all.
fn content_at(c: &Client) -> (usize, usize) {
    let (w, _, px) = pixels(&c.s.guest).expect("shown");
    let at = px.iter().position(|&p| p == BLUE).expect("the guest's pixels are shown");
    (at % w as usize, at / w as usize)
}

/// The guest's `W` by `H` as the window shows them.
fn content(c: &Client, (x0, y0): (usize, usize)) -> Vec<u32> {
    let (w, _, px) = pixels(&c.s.guest).expect("shown");
    let w = w as usize;
    (0..H as usize)
        .flat_map(|y| (0..W as usize).map(move |x| (y, x)))
        .map(|(y, x)| px[(y0 + y) * w + x0 + x])
        .collect()
}

fn presents() -> Vec<(u64, u64)> {
    desk::with(|d| {
        d.calls
            .iter()
            .filter_map(|c| match c {
                Call::Present(on, to, _) => Some((*on, *to)),
                _ => None,
            })
            .collect()
    })
}

fn released(evs: &[Ev], buffer: u32) -> bool {
    evs.iter().any(|e| e.object == buffer && e.opcode == BUFFER_RELEASE)
}

/// A small generator, so a failure names its seed.
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) % u64::from(n)) as u32
    }
}

#[test]
fn each_commit_alternates_buffers_and_names_the_layer_by_the_one_shown() {
    let mut c = window();
    let first = c.s.guest.scene.win.out.expect("shown");
    for n in 0..6 {
        commit_box(&mut c, n, 0xFF00_0000 | ((n + 1) * 0x10_1010), (0, 0, W, H));
    }
    let p = presents();
    assert_eq!(p.len(), 6, "one present a commit, no damage commit: {p:?}");
    assert_eq!(p[0].0, first, "the first names the layer by its first buffer");
    for pair in p.windows(2) {
        assert_eq!(pair[1].0, pair[0].1, "each names the layer by the buffer shown");
        assert_ne!(pair[0].0, pair[0].1, "and shows the other");
    }
    let handles: std::collections::BTreeSet<u64> = p.iter().flat_map(|&(a, b)| [a, b]).collect();
    assert_eq!(handles.len(), 2, "two buffers, no more: {handles:?}");
    assert!(desk::with(|d| d.calls.iter().all(|c| !matches!(c, Call::Damage(..)))));
}

#[test]
fn partial_commits_on_alternate_buffers_always_show_the_whole_picture() {
    let mut partial = 0;
    for seed in 1..=12u64 {
        let mut c = window();
        let origin = content_at(&c);
        let mut model = vec![BLUE; (W * H) as usize];
        let mut rng = Rng(seed);
        for n in 0..24 {
            let (x, y) = (rng.below(W as u32 - 1) as i32, rng.below(H as u32 - 1) as i32);
            let (w, h) =
                (1 + rng.below((W - x) as u32) as i32, 1 + rng.below((H - y) as u32) as i32);
            let argb = 0xFF00_0000 | (rng.below(0x00FF_FFFF) + 1);
            for yy in y..y + h {
                for xx in x..x + w {
                    model[(yy * W + xx) as usize] = argb;
                }
            }
            desk::with(|d| d.calls.clear());
            commit_picture(&mut c, n, &model, (x, y, w, h));
            // Read and presented as the box alone: the rest of the buffer
            // drawn in is what the other buffer brought it up to.
            let boxed = desk::with(|d| {
                d.calls.iter().any(
                    |c| matches!(c, Call::Present(_, _, a) if (a.2, a.3) == (w as u32, h as u32)),
                )
            });
            partial += usize::from(boxed);
            let shown = content(&c, origin);
            let wrong = shown.iter().zip(&model).filter(|(a, b)| a != b).count();
            assert_eq!(wrong, 0, "seed {seed} commit {n}: {wrong} pixels not the picture");
        }
        assert!(desk::with(|d| d.held.is_empty()), "every buffer replaced was released");
    }
    assert!(partial >= 12 * 20, "{partial} of {} commits presented as their box", 12 * 24);
}

#[test]
fn a_commit_finding_no_buffer_keeps_the_guests_until_it_is_drawn() {
    let mut c = window();
    let origin = content_at(&c);
    set_clock(1_000);
    desk::with(|d| d.hold = true);
    // The first commit takes the free buffer; the one shown is held.
    let evs = commit_box(&mut c, 0, 0xFF80_8080, (0, 0, W, H));
    assert!(released(&evs, 101), "drawn at once, given back at once");
    // The next finds both shown or held: not drawn, not given back.
    let evs = commit_box(&mut c, 1, 0xFF40_C040, (0, 0, W, H));
    assert!(!released(&evs, 103), "the client must not write what is still to be shown");
    assert!(content(&c, origin).iter().all(|&p| p == 0xFF80_8080), "the frame before stays");
    assert!(c.s.guest.scene.win.owing.is_some());
    // Due after a refresh; nothing is free until the compositor composes.
    let due = c.s.tick().expect("a wake for the debt");
    assert!(due <= 16, "{due} ms");
    set_clock(1_016);
    c.s.tick();
    assert!(c.s.guest.scene.win.owing.is_some(), "still nothing free");
    desk::compose();
    set_clock(2_000);
    c.s.tick();
    let evs = c.events();
    assert!(c.s.guest.scene.win.owing.is_none());
    assert!(released(&evs, 103), "given back once drawn");
    assert!(content(&c, origin).iter().all(|&p| p == 0xFF40_C040), "the frame owed is shown");
    desk::with(|d| d.hold = false);
}

#[test]
fn a_later_commit_takes_an_owed_ones_place_and_gives_it_back() {
    let mut c = window();
    let origin = content_at(&c);
    desk::with(|d| d.hold = true);
    commit_box(&mut c, 0, 0xFF80_8080, (0, 0, W, H));
    let evs = commit_box(&mut c, 1, 0xFF40_C040, (0, 0, W, H));
    assert!(!released(&evs, 103));
    desk::compose();
    // The next commit draws; the one kept is given back, never drawn.
    let evs = commit_box(&mut c, 2, 0xFFC0_4040, (0, 0, W, H));
    assert!(released(&evs, 103) && released(&evs, 105), "both given back: {:?}", evs.len());
    assert!(c.s.guest.scene.win.owing.is_none());
    assert!(content(&c, origin).iter().all(|&p| p == 0xFFC0_4040));
    desk::with(|d| d.hold = false);
}

#[test]
fn a_refused_present_leaves_the_picture_and_the_next_commit_draws_it_whole() {
    let mut c = window();
    let origin = content_at(&c);
    let shown = c.s.guest.scene.win.out;
    desk::with(|d| d.refuse = true);
    commit_box(&mut c, 0, 0xFF80_8080, (0, 0, W, H));
    assert_eq!(c.s.guest.scene.win.out, shown, "the layer still shows its buffer");
    assert!(content(&c, origin).iter().all(|&p| p == BLUE));
    desk::with(|d| d.refuse = false);
    // A partial commit after is drawn whole, so the frame missed shows too.
    commit_box(&mut c, 1, 0xFF40_C040, (0, 0, 8, 8));
    let px = content(&c, origin);
    let green = px.iter().filter(|&&p| p == 0xFF40_C040).count();
    assert_eq!(green, (W * H) as usize, "read whole from the guest's buffer");
    assert!(c.s.guest.scene.win.owing.is_none());
}

#[test]
fn closed_the_window_lets_go_of_both_buffers() {
    let mut c = window();
    commit_box(&mut c, 0, 0xFF80_8080, (0, 0, W, H));
    let handles: Vec<u64> = presents().iter().flat_map(|&(a, b)| [a, b]).collect();
    assert!(handles.iter().all(|&h| nonos_libc::world::surface(h).is_some()));
    c.req(TOPLEVEL, 0, &[]);
    c.req(XDG_SURFACE, 0, &[]);
    c.req(SURFACE, 0, &[]);
    c.events();
    assert!(handles.iter().all(|&h| nonos_libc::world::surface(h).is_none()), "both released");
    assert!(c.s.guest.scene.win.out.is_none());
    let _ = family_ms();
}

/// A control lit under the pointer waits for no buffer: with none free the
/// window owes the repaint, the serve loop is not held up to two refreshes
/// behind a highlight, and the repaint shows once the compositor composes.
#[test]
fn a_control_lit_by_the_pointer_waits_for_no_buffer() {
    use nonos_capsule_wayland_proofs::linux::wayland::pump;
    use nonos_toolkit::decorations::{hit_test_at, DecorationHit};
    let mut c = window();
    set_clock(1_000);
    desk::with(|d| d.hold = true);
    // The free buffer drawn and shown; the one it replaced is held.
    commit_box(&mut c, 0, 0xFF80_8080, (0, 0, W, H));
    let (w, h, _) = pixels(&c.s.guest).expect("shown");
    let q = c.s.guest.scene.win.frame.quarters;
    let close = (0..w)
        .flat_map(|x| (0..h.min(64)).map(move |y| (x, y)))
        .find(|&(x, y)| hit_test_at(w, h, false, x, y, q) == DecorationHit::CloseButton)
        .expect("a close button");
    nonos_libc::world::with(|world| world.longest_wait_ms = 0);
    nonos_capsule_wayland_proofs::input::motion(close.0 as i32, close.1 as i32);
    pump(&mut c.s.guest);
    assert_eq!(nonos_libc::world::with(|world| world.longest_wait_ms), 0, "no fence wait");
    assert_eq!(c.s.guest.scene.win.frame.hover, DecorationHit::CloseButton);
    let owed = c.s.guest.scene.win.owing.map(|o| format!("{:?}", o.what));
    assert_eq!(owed.as_deref(), Some("Repaint"), "the highlight is owed");
    desk::compose();
    set_clock(2_000);
    c.s.tick();
    assert!(c.s.guest.scene.win.owing.is_none(), "drawn once a buffer is free");
    let lit = c.s.guest.scene.win.frame.chrome.map(|k| k.1);
    assert_eq!(lit, Some(DecorationHit::CloseButton), "the frame shown is lit");
    desk::with(|d| d.hold = false);
}
