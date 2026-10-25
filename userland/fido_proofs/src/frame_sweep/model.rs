// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound host hears on one channel, stated apart from the
//! assembler (CTAP 2.x, 11.2.4 and 11.2.9, as recalled, unverified):
//! reports under 7 bytes or on another channel are nothing. An opening
//! (bit 7 of byte 4) of KEEPALIVE with one byte of status 1 or 2 is heard;
//! any other KEEPALIVE is nothing. ERROR with one byte is its code heard;
//! of another length it breaks what was being put together. Any other
//! opening longer than 7609 bytes, or opening while a message is under way,
//! breaks it; else it begins a message of that length. A continuation
//! carrying the next sequence number adds its bytes; any other breaks. A
//! message is heard once it holds its length, the bytes past it dropped.

use crate::ctaphid_parts::heard::Heard;

pub fn heard(reports: &[Vec<u8>], cid: [u8; 4]) -> Vec<Heard> {
    let (mut out, mut msg): (Vec<Heard>, Option<(u8, usize, Vec<u8>, u8)>) = (Vec::new(), None);
    for p in reports {
        if p.len() < 7 || p[..4] != cid {
            continue;
        }
        let n = u16::from_be_bytes([p[5], p[6]]) as usize;
        let broke = |msg: &mut Option<_>, out: &mut Vec<Heard>| {
            *msg = None;
            out.push(Heard::Broken);
        };
        if p[4] & 0x80 == 0 {
            match msg.as_mut() {
                Some((_, _, _, seq)) if *seq == p[4] => *seq += 1,
                _ => {
                    broke(&mut msg, &mut out);
                    continue;
                }
            }
        } else {
            match (p[4], n, p.get(7)) {
                (0xBB, 1, Some(&s)) if s == 1 || s == 2 => out.push(Heard::Keepalive(s)),
                (0xBB, ..) => {}
                (0xBF, 1, Some(&c)) => out.push(Heard::Error(c)),
                (0xBF, ..) => broke(&mut msg, &mut out),
                (_, n, _) if n > 7609 || msg.is_some() => broke(&mut msg, &mut out),
                (cmd, n, _) => msg = Some((cmd, n, Vec::new(), 0)),
            }
            // Only an opening that began a message carries its bytes.
            if !matches!(&msg, Some((_, _, got, 0)) if got.is_empty())
                || matches!(p[4], 0xBB | 0xBF)
            {
                continue;
            }
        }
        let Some((cmd, need, got, _)) = msg.as_mut() else { continue };
        let data = if p[4] & 0x80 != 0 { &p[7..] } else { &p[5..] };
        got.extend_from_slice(&data[..(*need - got.len()).min(data.len())]);
        if got.len() == *need {
            out.push(Heard::Message(*cmd, core::mem::take(got)));
            msg = None;
        }
    }
    out
}
