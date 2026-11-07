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

//! A client's session as a file, and the file played back into a fresh
//! session. A record is what reached the server, in order: the bytes of
//! each read with the length of each descriptor it carried ('R'), and each
//! frame put in its inbox from outside, with its sender ('I'): the router's
//! input, the window manager's notices. Each step carries the server's clock
//! when it came. Played back, the clock is set to it and what came due by
//! then is let go first, each at the time it came due, as the live server's
//! turns did (frame callbacks held for their period); each descriptor is a
//! memfd of its length, which a pool maps as the client's did, of opaque
//! grey (filled).

use std::io::Write;

use crate::session::Session;

/// The file's first bytes, and its version.
pub const MAGIC: &[u8; 8] = b"NWREC\x00\x00\x03";

pub fn read_chunk(bytes: &[u8], fd_lens: &[u64]) -> Vec<u8> {
    let mut out = vec![b'R'];
    out.extend_from_slice(&crate::linux::call::family_ms().to_le_bytes());
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&(fd_lens.len() as u32).to_le_bytes());
    fd_lens.iter().for_each(|l| out.extend_from_slice(&l.to_le_bytes()));
    out.extend_from_slice(bytes);
    out
}

pub fn input_chunk(sender: u32, frame: &[u8]) -> Vec<u8> {
    let mut out = vec![b'I'];
    out.extend_from_slice(&crate::linux::call::family_ms().to_le_bytes());
    out.extend_from_slice(&(frame.len() as u32).to_le_bytes());
    out.extend_from_slice(&sender.to_le_bytes());
    out.extend_from_slice(frame);
    out
}

/// The length of what `fd` names, or zero for a pipe or socket.
pub fn fd_len(fd: i32) -> u64 {
    // SAFETY: fstat of a descriptor the caller holds.
    unsafe {
        let mut st: libc::stat = core::mem::zeroed();
        if libc::fstat(fd, &mut st) != 0 || st.st_mode & libc::S_IFMT != libc::S_IFREG {
            return 0;
        }
        st.st_size as u64
    }
}

/// Appends to a record file as a session goes.
pub struct Recorder(std::fs::File);

impl Recorder {
    pub fn create(path: &str) -> Option<Recorder> {
        let mut f = std::fs::File::create(path).ok()?;
        f.write_all(MAGIC).ok()?;
        Some(Recorder(f))
    }

    pub fn put(&mut self, chunk: &[u8]) {
        let _ = self.0.write_all(chunk);
    }
}

/// One step of a record, and the server's clock when it came.
pub enum Step<'a> {
    Read(u64, &'a [u8], Vec<u64>),
    Input(u64, u32, &'a [u8]),
}

/// The steps of `rec`, or None where it is cut short or not a record.
pub fn steps(rec: &[u8]) -> Option<Vec<Step<'_>>> {
    let mut at = rec.strip_prefix(MAGIC.as_slice())?;
    let word = |b: &[u8]| -> Option<u32> { Some(u32::from_le_bytes(b.get(..4)?.try_into().ok()?)) };
    let mut out = Vec::new();
    while let Some((&tag, rest)) = at.split_first() {
        let ms = u64::from_le_bytes(rest.get(..8)?.try_into().ok()?);
        let rest = &rest[8..];
        let len = word(rest)? as usize;
        match tag {
            b'R' => {
                let n = word(rest.get(4..)?)? as usize;
                let lens_end = 8 + n * 8;
                let lens = rest.get(8..lens_end)?.chunks(8);
                let lens =
                    lens.map(|c| u64::from_le_bytes(c.try_into().unwrap_or([0; 8]))).collect();
                out.push(Step::Read(ms, rest.get(lens_end..lens_end + len)?, lens));
                at = &rest[lens_end + len..];
            }
            b'I' => {
                let sender = word(rest.get(4..)?)?;
                out.push(Step::Input(ms, sender, rest.get(8..8 + len)?));
                at = &rest[8 + len..];
            }
            _ => return None,
        }
    }
    Some(out)
}

/// Play `rec` into a new session, reading out after every step as the
/// client did. The session as it ends and every event byte it sent, or None
/// for a malformed record.
pub fn replay(rec: &[u8]) -> Option<(Session, Vec<u8>)> {
    replay_watched(rec, |_| {})
}

/// The same, with `watch` shown the session after every step.
pub fn replay_watched(rec: &[u8], mut watch: impl FnMut(&Session)) -> Option<(Session, Vec<u8>)> {
    let mut s = Session::new();
    let mut sent = Vec::new();
    for step in steps(rec)? {
        let (Step::Read(ms, ..) | Step::Input(ms, ..)) = step;
        // The live server's own turns between two steps: each held frame
        // answered when it came due, not at the next step.
        let mut now = crate::linux::call::family_ms();
        while let Some(wait) = s.tick().map(|w| w.max(1)).filter(|&w| now + w < ms) {
            now += wait;
            crate::linux::call::set_clock(now);
        }
        crate::linux::call::set_clock(ms);
        s.tick();
        match step {
            Step::Read(_, bytes, lens) => {
                let fds: Vec<i32> = lens.iter().filter_map(|&l| filled(l)).collect();
                let mut at = 0;
                while at < bytes.len() {
                    let n = s.send(&bytes[at..], if at == 0 { &fds } else { &[] });
                    let (bytes, fds) = s.recv(usize::MAX);
                    sent.extend_from_slice(&bytes);
                    close_all(fds);
                    if n == 0 {
                        return None;
                    }
                    at += n;
                }
            }
            Step::Input(_, sender, frame) => {
                nonos_libc::world::post(sender, frame.to_vec());
                let (bytes, fds) = s.recv(usize::MAX);
                sent.extend_from_slice(&bytes);
                close_all(fds);
            }
        }
        watch(&s);
    }
    Some((s, sent))
}

fn close_all(fds: Vec<i32>) {
    // SAFETY: descriptors handed over by the session, each closed once.
    fds.into_iter().for_each(|fd| unsafe {
        libc::close(fd);
    });
}

/// The most of a descriptor filled: a client's pool can be far larger than
/// what it draws in (foot's are 512 MiB), and the rest stays zero.
const FILLED: u64 = 64 << 20;

/// What a client's pixels are played back as: opaque grey, the same in
/// every buffer, so two frames of a window differ only by what the server
/// adds to them (its frame, title, focus, size), not by which of the
/// client's buffers was drawn.
const GREY: u32 = 0xFF80_8080;

/// A memfd of `len` bytes, standing in for a descriptor of that length,
/// its first FILLED bytes opaque grey.
fn filled(len: u64) -> Option<i32> {
    let bytes = GREY.to_le_bytes().repeat(len.min(FILLED) as usize / 4);
    // SAFETY: a new descriptor, written and sized before it is handed over.
    unsafe {
        let fd = libc::memfd_create(c"nonos-replay".as_ptr(), libc::MFD_CLOEXEC);
        if fd < 0 || libc::ftruncate(fd, len as libc::off_t) != 0 {
            return None;
        }
        if libc::pwrite(fd, bytes.as_ptr().cast(), bytes.len(), 0) != bytes.len() as isize {
            libc::close(fd);
            return None;
        }
        Some(fd)
    }
}
