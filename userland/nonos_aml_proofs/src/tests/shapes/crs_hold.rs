// NONOS Operating System (AGPL-3.0-or-later)
//! The three template readers held to the model over every seed.

use super::*;

fn fits(b: &[u8]) -> bool {
    b.len() >= 2 + 4 * usize::from(b[1])
}

/// One per descriptor naming any: an IRQ's lowest, an Extended's first.
fn first(t: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();
    for (large, name, _, b) in frames(t) {
        if !large && name == 4 && (b.len() == 2 || b.len() == 3) {
            let mask = u16::from(b[0]) | u16::from(b[1]) << 8;
            out.extend((mask != 0).then(|| mask.trailing_zeros()));
        } else if large && name == 9 && b.len() >= 2 && b[1] >= 1 && fits(b) {
            out.push(u32::from_le_bytes([b[2], b[3], b[4], b[5]]));
        }
    }
    out
}

#[test]
fn templates_are_their_model() {
    let (mut honest, mut lying) = (0, 0);
    for seed in 0..SEEDS {
        let (t, truth) = template(&mut Rng::new(seed ^ 0xC25));
        let got: Vec<_> = items(&t).iter().map(|i| (i.large, i.kind, i.whole, i.body)).collect();
        assert_eq!(got, frames(&t), "items, seed {seed}: {t:x?}");
        assert_eq!(interrupts(&t), first(&t), "interrupts, seed {seed}: {t:x?}");
        let all = interrupt_entries(&t);
        assert_eq!(all, entries(&t), "entries, seed {seed}: {t:x?}");
        let Some(truth) = truth else {
            lying += 1;
            continue;
        };
        honest += 1;
        assert_eq!(frames(&t).len(), truth.items, "seed {seed}");
        assert_eq!(interrupts(&t), truth.first, "seed {seed}");
        assert_eq!(all.iter().map(|i| i.gsiv).collect::<Vec<_>>(), truth.all, "seed {seed}");
    }
    assert!(honest >= FLOOR && lying >= FLOOR, "{honest} honest, {lying} lying");
}
