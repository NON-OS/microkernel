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

//! A desynchronized subsurface over a framed window, as pcmanfm's type-ahead
//! box is: an 800x600 toplevel, a 176x42 child (stride 704) at (624, 563)
//! set desync, committed with damage (4, 4, 168, 34). The window must still
//! show every one of its own pixels where they were, row for row.

use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::session::Session;

const W: u32 = 800;
const H: u32 = 600;
const CW: u32 = 176;
const CH: u32 = 42;
const CHILD: u32 = 0xFF20_8040;

struct Client {
    s: Session,
    events: Vec<u8>,
}

fn string(out: &mut Vec<u8>, s: &str) {
    let n = s.len() as u32 + 1;
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(s.as_bytes());
    out.push(0);
    while !out.len().is_multiple_of(4) {
        out.push(0);
    }
}

fn msg(obj: u32, op: u16, args: &[u8]) -> Vec<u8> {
    let mut m = obj.to_le_bytes().to_vec();
    m.extend_from_slice(&(((8 + args.len() as u32) << 16) | op as u32).to_le_bytes());
    m.extend_from_slice(args);
    m
}

fn words(w: &[u32]) -> Vec<u8> {
    w.iter().flat_map(|v| v.to_le_bytes()).collect()
}

impl Client {
    fn send(&mut self, obj: u32, op: u16, args: &[u8], fds: &[i32]) {
        let m = msg(obj, op, args);
        assert_eq!(self.s.send(&m, fds), m.len());
        self.pump();
    }

    fn pump(&mut self) {
        loop {
            let (b, _) = self.s.recv(1 << 16);
            if b.is_empty() {
                break;
            }
            self.events.extend_from_slice(&b);
        }
    }

    /// Every event as (object, opcode, body), from the start.
    fn parsed(&self) -> Vec<(u32, u16, Vec<u8>)> {
        let (mut v, mut at, e) = (Vec::new(), 0, &self.events);
        while at + 8 <= e.len() {
            let obj = u32::from_le_bytes(e[at..at + 4].try_into().unwrap());
            let so = u32::from_le_bytes(e[at + 4..at + 8].try_into().unwrap());
            let size = (so >> 16) as usize;
            v.push((obj, so as u16, e[at + 8..at + size].to_vec()));
            at += size;
        }
        v
    }

    fn global(&self, iface: &str) -> (u32, u32) {
        for (obj, op, b) in self.parsed() {
            if obj == 2 && op == 0 {
                let name = u32::from_le_bytes(b[0..4].try_into().unwrap());
                let n = u32::from_le_bytes(b[4..8].try_into().unwrap()) as usize;
                if &b[8..8 + n - 1] == iface.as_bytes() {
                    let at = 8 + n.div_ceil(4) * 4;
                    return (name, u32::from_le_bytes(b[at..at + 4].try_into().unwrap()));
                }
            }
        }
        panic!("no global {iface}");
    }

    fn bind(&mut self, iface: &str, version: u32, id: u32) {
        let (name, v) = self.global(iface);
        let mut a = words(&[name]);
        string(&mut a, iface);
        a.extend_from_slice(&words(&[version.min(v), id]));
        self.send(2, 0, &a, &[]);
    }

    fn last_configure(&self, xdg_surface: u32) -> u32 {
        let c = self.parsed().into_iter().rfind(|(o, op, _)| *o == xdg_surface && *op == 0);
        u32::from_le_bytes(c.expect("configured").2[0..4].try_into().unwrap())
    }
}

fn parent_px(x: u32, y: u32) -> u32 {
    0xFF00_0000 | (y * W + x)
}

#[test]
fn a_desync_subsurface_leaves_its_framed_parent_row_for_row() {
    let mut c = Client { s: Session::new(), events: Vec::new() };
    c.send(1, 1, &words(&[2]), &[]);
    c.bind("wl_compositor", 4, 3);
    c.bind("wl_shm", 1, 4);
    c.bind("wl_subcompositor", 1, 5);
    c.bind("xdg_wm_base", 2, 6);
    // One pool: the parent's pixels, then the child's.
    let (parent_len, child_len) = ((W * H * 4) as usize, (CW * 4 * CH) as usize);
    let size = parent_len + child_len;
    // SAFETY: a plain memfd of `size` bytes, mapped shared and written here.
    let fd = unsafe { libc::memfd_create(c"pool".as_ptr(), 0) };
    assert!(fd >= 0);
    // SAFETY: as above.
    unsafe {
        assert_eq!(libc::ftruncate(fd, size as i64), 0);
        let p = libc::mmap(
            core::ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        );
        assert_ne!(p, libc::MAP_FAILED);
        let px = core::slice::from_raw_parts_mut(p as *mut u32, size / 4);
        for y in 0..H {
            for x in 0..W {
                px[(y * W + x) as usize] = parent_px(x, y);
            }
        }
        px[(W * H) as usize..].fill(CHILD);
    }
    c.send(4, 0, &words(&[7, size as u32]), &[fd]);
    c.send(7, 0, &words(&[8, 0, W, H, W * 4, 0]), &[]);
    c.send(7, 0, &words(&[9, parent_len as u32, CW, CH, CW * 4, 0]), &[]);
    c.send(3, 0, &words(&[10]), &[]);
    c.send(6, 2, &words(&[11, 10]), &[]);
    c.send(11, 1, &words(&[12]), &[]);
    c.send(10, 6, &[], &[]);
    let serial = c.last_configure(11);
    c.send(11, 4, &words(&[serial]), &[]);
    c.send(10, 1, &words(&[8, 0, 0]), &[]);
    c.send(10, 9, &words(&[0, 0, W, H]), &[]);
    c.send(10, 6, &[], &[]);
    // The type-ahead box.
    c.send(3, 0, &words(&[13]), &[]);
    c.send(5, 1, &words(&[14, 13, 10]), &[]);
    c.send(14, 1, &words(&[624, 563]), &[]);
    c.send(14, 5, &[], &[]);
    // The parent draws again with its child in place, as GTK does.
    c.send(10, 1, &words(&[8, 0, 0]), &[]);
    c.send(10, 9, &words(&[0, 0, W, H]), &[]);
    c.send(10, 6, &[], &[]);
    c.send(13, 1, &words(&[9, 0, 0]), &[]);
    c.send(13, 2, &words(&[4, 4, 168, 34]), &[]);
    c.send(13, 6, &[], &[]);
    let (ow, oh, out) = pixels(&c.s.guest).expect("a window shown");
    // Where the parent's first pixel landed is where the content starts.
    let at = out.iter().position(|&p| p == parent_px(0, 0)).expect("the window's pixels are shown");
    // Before the fix the rows were copied at the client's stride into the
    // wider frame: (0, 0) landed at the frame's corner and every row slid.
    let (cx, cy) = ((at as u32) % ow, (at as u32) / ow);
    assert!(cx + W <= ow && cy + H <= oh, "the content fits in {ow}x{oh} at ({cx}, {cy})");
    let child = |x: u32, y: u32| (624..624 + CW).contains(&x) && (563..563 + CH).contains(&y);
    let mut wrong = 0;
    for y in 0..H {
        for x in 0..W {
            let got = out[((cy + y) * ow + cx + x) as usize];
            let want = if child(x, y) { CHILD } else { parent_px(x, y) };
            wrong += u32::from(got != want);
        }
    }
    assert_eq!(wrong, 0, "{wrong} of the window's pixels moved or were lost");
}
