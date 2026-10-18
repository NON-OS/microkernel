// NONOS Operating System (AGPL-3.0-or-later)
//! F-110: _OSI against hostile strings, under every personality. The model
//! (ACPI 6.5, 5.7.2; recalled, unverified): a string is answered Ones only
//! when it is, byte for byte, a row of the table marked answered; a
//! Windows release only among the first `windows` such rows. A string
//! near a row (a letter's case changed, a byte added or cut, a prefix, a
//! NUL or a space) is not that row, so it is Zero: firmware never learns
//! of support NONOS did not decide to claim.

use super::osi;
use crate::exec::{rows, Group, Personality};

fn model(s: &[u8], windows: usize) -> u64 {
    let mut seen_windows = 0;
    for r in rows() {
        let shown = r.group != Group::Windows || seen_windows < windows;
        seen_windows += usize::from(r.group == Group::Windows);
        if r.string == s && r.answered && shown {
            return u64::MAX;
        }
    }
    0
}

fn near(s: &[u8], seed: &mut u64) -> Vec<Vec<u8>> {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    let mut out = vec![s.to_vec(), [s, b" "].concat(), [s, b"\0"].concat(), [b"x", s].concat()];
    out.push(s.iter().map(u8::to_ascii_uppercase).collect());
    if !s.is_empty() {
        out.push(s[..s.len() - 1].to_vec());
        let mut flip = s.to_vec();
        let i = (*seed % s.len() as u64) as usize;
        flip[i] ^= 0x20;
        out.push(flip);
    }
    out
}

#[test]
fn osi_answers_only_its_own_rows_byte_for_byte() {
    let others: [&[u8]; 8] = [
        b"",
        b"Linux",
        b"Darwin",
        b"FreeBSD",
        b"Windows 2099",
        b"Windows",
        b"Microsoft Windows NT",
        b"Module Device ",
    ];
    let (mut yes, mut no, mut seed) = (0, 0, 0x2545_F491_4F6C_DD1Du64);
    let total = rows().filter(|r| r.group == Group::Windows).count();
    for windows in 0..=total {
        let p = Personality { windows, ..Personality::DEFAULT };
        let strings: Vec<Vec<u8>> = rows()
            .flat_map(|r| near(r.string, &mut seed))
            .chain(others.iter().map(|o| o.to_vec()))
            .collect();
        for s in strings {
            let want = model(&s, windows);
            assert_eq!(
                osi(p, &s),
                want,
                "{:?} with {windows} Windows rows",
                String::from_utf8_lossy(&s)
            );
            *(if want != 0 { &mut yes } else { &mut no }) += 1;
        }
    }
    assert!(yes >= 100 && no >= 1000, "{yes} answered, {no} not");
}
