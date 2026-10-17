// NONOS Operating System (AGPL-3.0-or-later)
//! find_reader over seeded configurations, sound and then told one lie:
//! it finds what the rule in desc_sweep/model.rs finds from the bytes,
//! and what each lie must leave.

use crate::ccid::find_reader;
use crate::desc_sweep::build::{raw, sound};
use crate::desc_sweep::model::{rule, Found};
use crate::exchange_sweep::rng::{rounds, Rng};
use crate::reader::class;

const LIES: [&str; 9] =
    ["sound", "foreign", "short", "missing", "level", "no bulk", "alt only", "vendor", "alt first"];

#[test]
fn every_reader_is_found_as_its_descriptors_say() {
    let mut told = [0u32; 9];
    for seed in 1..rounds(20_000) {
        let r = &mut Rng::new(seed);
        let (lie, mut b) = (r.range(0, LIES.len() - 1), sound(r));
        let c = b.class.clone();
        let word = |at: usize| u32::from_le_bytes(c[at..at + 4].try_into().unwrap());
        let mut want: Found = Some(Ok((b.iface[2], c[4], word(40), (word(44) as usize).min(4096))));
        match LIES[lie] {
            "foreign" => {
                (b.iface[2], b.hid) = (1, Some(class(9, 0x0004_0000, 0x777)));
                if r.range(0, 1) == 0 {
                    (b.class, want) = (vec![], Some(Err("no CCID descriptor")));
                } else if let Some(Ok(w)) = want.as_mut() {
                    w.0 = 1;
                }
            }
            "short" | "missing" | "alt only" => {
                let n = if LIES[lie] == "short" { r.range(2, 53) } else { 0 };
                b.class.truncate(n);
                if n > 0 {
                    b.class[0] = n as u8;
                }
                if LIES[lie] == "alt only" {
                    b.alt = Some(c.clone());
                }
                want = Some(Err("no CCID descriptor"));
            }
            "level" => {
                let f = word(40) & !0x6_0000;
                b.class[40..44].copy_from_slice(&f.to_le_bytes());
                want = Some(Err("TPDU or character level reader"));
            }
            "no bulk" => {
                b.eps.remove(r.range(0, 1));
                b.iface[4] -= 1;
                want = Some(Err("no bulk pair"));
            }
            "vendor" => {
                b.vendor = vec![0xC5; r.range(54, 255)];
                (b.vendor[0], b.vendor[1]) =
                    (b.vendor.len() as u8, [0x24, 0x22, 0xFF][r.range(0, 2)]);
            }
            "alt first" => b.alt_first = true,
            _ => {}
        }
        told[lie] += 1;
        let raw = raw(&b);
        let got = find_reader(&raw)
            .map(|f| f.map(|r| (r.interface, r.max_slot, r.features, r.max_message)));
        assert_eq!(got, rule(&raw), "seed {seed}: {}: the rule", LIES[lie]);
        assert_eq!(got, want, "seed {seed}: {}", LIES[lie]);
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 2_000, "{lie} was told only {n} times");
    }
}
