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

//! What every one of these services promises about the frames it is sent,
//! written once. A service is described by a `Spec`: how its decode is called
//! and what it gives back, the length rule its header follows, whether a
//! refusal echoes the request it refuses, and the reply its loop sends for a
//! refusal, built with the service's own encoders. `check` holds a frame to
//! all of it; `fuzz` and `boundaries` choose the frames.

/// The common header every one of these services reads: magic, version, op,
/// flags, two reserved bytes, request id and payload length.
pub const HDR_LEN: usize = 20;
/// The status word every refusal carries after its header.
pub const STATUS_LEN: usize = 4;

/// What a decode made of a frame.
pub enum Outcome {
    Served { op: u16, flags: u16, id: u32, body: Vec<u8> },
    Refused { op: u16, flags: u16, id: u32, status: i32 },
}

/// How a header's length field relates to the bytes that came.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Len {
    /// The body is exactly the bytes after the header.
    Exact,
    /// The body is the first `length` bytes after the header; more may follow.
    AtLeast,
    /// As `AtLeast`, with the length field capped.
    AtLeastUpTo(u32),
    /// The decode leaves the length to the handlers and hands back no body.
    Unread,
}

pub struct Spec {
    pub name: &'static str,
    pub magic: u32,
    pub len: Len,
    /// A refusal of a whole header names its op, flags and request id; a
    /// refusal of a short frame, or any refusal when false, names zeros.
    pub echo: bool,
    pub last_op: u16,
    /// The statuses a refusal may carry.
    pub statuses: &'static [i32],
    /// The receive buffer the loop hands the decode a slice of; the kernel
    /// truncates a longer message to it.
    pub rx_len: usize,
    pub decode: fn(&[u8]) -> Outcome,
    /// The refusal the loop sends, from the service's own encoders.
    pub refusal: fn(u16, u16, u32, i32) -> Vec<u8>,
}

fn le16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

/// What must hold for any frame. Returns whether it was served.
pub fn check(spec: &Spec, buf: &[u8]) -> bool {
    let name = spec.name;
    let whole = buf.len() >= HDR_LEN;
    let (op, flags, id, len) =
        if whole { (le16(buf, 6), le16(buf, 8), le32(buf, 12), le32(buf, 16)) } else { (0, 0, 0, 0) };
    let after = buf.len().saturating_sub(HDR_LEN);
    let length_ok = match spec.len {
        Len::Exact => len as usize == after,
        Len::AtLeast => len as usize <= after,
        Len::AtLeastUpTo(max) => len <= max && len as usize <= after,
        Len::Unread => true,
    };
    let should_serve = whole && le32(buf, 0) == spec.magic && le16(buf, 4) == 1 && length_ok;
    match (spec.decode)(buf) {
        Outcome::Served { op: o, flags: f, id: i, body } => {
            assert!(should_serve, "{name} served a frame it should refuse: {} bytes", buf.len());
            assert_eq!((o, f, i), (op, flags, id), "{name}: the request is what the header names");
            let want: &[u8] =
                if spec.len == Len::Unread { &[] } else { &buf[HDR_LEN..HDR_LEN + len as usize] };
            assert_eq!(&body[..], want, "{name}: the body is what the length names");
            true
        }
        Outcome::Refused { op: o, flags: f, id: i, status } => {
            assert!(!should_serve, "{name} refused a frame it should serve: {} bytes", buf.len());
            assert!(spec.statuses.contains(&status), "{name}: status {status}");
            let want = if whole && spec.echo { (op, flags, id) } else { (0, 0, 0) };
            assert_eq!((o, f, i), want, "{name}: the refusal names the request it answers");
            let reply = (spec.refusal)(o, f, i, status);
            assert_eq!(reply.len(), HDR_LEN + STATUS_LEN, "{name}: a header and a status");
            assert_eq!((le32(&reply, 0), le16(&reply, 4)), (spec.magic, 1), "{name}: reply magic");
            assert_eq!((le16(&reply, 6), le16(&reply, 8), le32(&reply, 12)), (o, f, i), "{name}: reply names");
            assert_eq!(le32(&reply, 16), STATUS_LEN as u32, "{name}: reply length");
            assert_eq!(le32(&reply, HDR_LEN) as i32, status, "{name}: reply status");
            false
        }
    }
}

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

pub fn frame(spec: &Spec, op: u16, id: u32, len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&spec.magic.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&0x0102u16.to_le_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&id.to_le_bytes());
    f.extend_from_slice(&len.to_le_bytes());
    f.extend_from_slice(body);
    f
}

/// Seeded frames biased toward the length field, the op, the magic and the
/// version, cut to the receive buffer the way the kernel cuts them.
pub fn fuzz(spec: &Spec, seed: u64, rounds: usize) {
    let mut s = seed | 1;
    let body_max = (spec.rx_len - HDR_LEN).min(1024);
    let (mut served, mut refused) = (0usize, 0usize);
    for _ in 0..rounds {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (body_max as u64 + 9)) as usize;
        let body: Vec<u8> = (0..body_len).map(|_| xorshift(&mut s) as u8).collect();
        let len = match r % 8 {
            0 => body_len as u32 + 1,
            1 => (body_len as u32).wrapping_sub(1),
            2 => xorshift(&mut s) as u32,
            3 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => body_len as u32,
        };
        let op = match (r >> 8) % 4 {
            0 => xorshift(&mut s) as u16,
            _ => (xorshift(&mut s) % (u64::from(spec.last_op) + 2)) as u16,
        };
        let mut f = frame(spec, op, xorshift(&mut s) as u32, len, &body);
        f[8..10].copy_from_slice(&(xorshift(&mut s) as u16).to_le_bytes());
        match (r >> 16) % 16 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f[4 + (xorshift(&mut s) % 2) as usize] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            3 => f.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8),
            _ => {}
        }
        f.truncate(spec.rx_len);
        if check(spec, &f) {
            served += 1;
        } else {
            refused += 1;
        }
    }
    let name = spec.name;
    assert!(served > rounds / 10, "{name}: the generator reaches the dispatch: {served}");
    assert!(refused > rounds / 10, "{name}: the generator reaches the refusals: {refused}");
}

/// The named set: empty, one byte, one short of a header, a bare header,
/// length fields over, under and at their maximum, every op, the largest
/// frame the receive buffer holds, a wrong magic and a wrong version.
pub fn boundaries(spec: &Spec) {
    let name = spec.name;
    let bare = frame(spec, 1, 7, 0, &[]);
    for cut in [0, 1, HDR_LEN - 1] {
        assert!(!check(spec, &bare[..cut]), "{name}: {cut} bytes");
    }
    assert!(check(spec, &bare), "{name}: a bare header");
    if spec.rx_len >= HDR_LEN + 4 {
        let over = check(spec, &frame(spec, 1, 9, 5, &[1, 2, 3, 4]));
        assert_eq!(over, spec.len == Len::Unread, "{name}: a length one past the bytes");
        let under = check(spec, &frame(spec, 1, 9, 3, &[1, 2, 3, 4]));
        assert_eq!(under, spec.len != Len::Exact, "{name}: a length one short of the bytes");
    }
    let max = check(spec, &frame(spec, 1, 11, u32::MAX, &[]));
    assert_eq!(max, spec.len == Len::Unread, "{name}: the largest length field");
    for op in (0..=spec.last_op + 1).chain([u16::MAX]) {
        assert!(check(spec, &frame(spec, op, u32::from(op), 0, &[])), "{name}: op {op}");
    }
    let room = spec.rx_len - HDR_LEN;
    let big = frame(spec, 1, 13, room as u32, &vec![0x5A; room]);
    let fits = match spec.len {
        Len::AtLeastUpTo(cap) => room as u64 <= u64::from(cap),
        _ => true,
    };
    assert_eq!(check(spec, &big), fits, "{name}: the largest frame the buffer holds");
    if let Len::AtLeastUpTo(cap) = spec.len {
        // The cap itself is served and one past it refused, with the bytes
        // present either way.
        for len in [cap, cap + 1] {
            let f = frame(spec, 1, 19, len, &vec![0x33; len as usize]);
            assert!(f.len() <= spec.rx_len, "{name}: the receive buffer holds a capped body");
            assert_eq!(check(spec, &f), len == cap, "{name}: a length field of {len}");
        }
    }
    let mut bad = frame(spec, 2, 15, 0, &[]);
    bad[0] ^= 0xFF;
    assert!(!check(spec, &bad), "{name}: a wrong magic");
    let mut bad = frame(spec, 2, 17, 0, &[]);
    bad[4] = 2;
    assert!(!check(spec, &bad), "{name}: a wrong version");
}
