// NONOS Operating System (AGPL-3.0-or-later)
//! `admit` against the ruling written out, over generated ranges on a small
//! page grid: well formed, physically inside one reserved-memory region,
//! and clear of every held span and of each other; all or nothing.

use crate::memory::iommu::reserved::{admit, Refusal, ReservedRange};

const P: u64 = 0x1000;

fn rule(new: &[ReservedRange], held: &[(u64, u64)], regions: &[(u64, u64)]) -> Result<(), Refusal> {
    for (i, r) in new.iter().enumerate() {
        let (Some(ie), Some(pe)) = (r.iova.checked_add(r.len), r.phys.checked_add(r.len)) else {
            return Err(Refusal::Malformed);
        };
        if r.len == 0 || r.iova % P != 0 || r.phys % P != 0 || r.len % P != 0 {
            return Err(Refusal::Malformed);
        }
        if !regions.iter().any(|&(b, s)| b.checked_add(s).is_some_and(|e| b <= r.phys && pe <= e)) {
            return Err(Refusal::OutsideReservedMemory);
        }
        let over = |s: u64, e: u64| (r.iova..ie).any(|a| a >= s && a < e);
        let earlier = new[..i].iter().any(|o| over(o.iova, o.iova + o.len));
        if held.iter().any(|&(s, e)| over(s, e)) || earlier {
            return Err(Refusal::Overlap);
        }
    }
    Ok(())
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) % bound
    }
    /// A page multiple below `n` pages, now and then not on a page boundary.
    fn pick(&mut self, n: u64) -> u64 {
        if self.next(16) == 0 {
            self.next(n * P)
        } else {
            self.next(n) * P
        }
    }
}

#[test]
fn admit_follows_the_ruling() {
    let mut r = Lcg(40);
    let mut admitted = 0;
    for _ in 0..30_000 {
        let new: Vec<ReservedRange> = (0..r.next(4))
            .map(|_| ReservedRange { iova: r.pick(12), phys: r.pick(12), len: r.pick(4) })
            .collect();
        let held: Vec<(u64, u64)> = (0..r.next(3))
            .map(|_| {
                let s = r.pick(12);
                (s, s + r.pick(4))
            })
            .collect();
        let regions: Vec<(u64, u64)> = (0..r.next(3)).map(|_| (r.pick(12), r.pick(8))).collect();
        let got = admit(&new, &held, &regions);
        assert_eq!(got, rule(&new, &held, &regions), "{new:x?} {held:x?} {regions:x?}");
        admitted += got.is_ok() as u32;
    }
    assert!(admitted > 1_000, "hardly any admitted: {admitted}");
}
