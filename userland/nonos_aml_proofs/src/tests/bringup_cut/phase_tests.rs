// NONOS Operating System (AGPL-3.0-or-later)
//! Each bring-up phase cut at every step below the most any one of its
//! evaluations used. The model: the cut phase fails, not half succeeds,
//! and its line says so; every other phase gives what it gives uncut;
//! nothing is held; the routes the capsule serves are the uncut ones; and
//! an EC device whose _REG the cut stopped is told at the next connect.
//! Floors are our choice, from no source.

use super::fixture::dsdt;
use super::phases::bring_up;
use crate::consts::at;
use crate::platform::routes;
use crate::region::Space;
use crate::rig::Rig;
use crate::tests::platform::run::evaluator;

#[test]
fn a_cut_phase_fails_whole_and_says_so() {
    let full = bring_up(&mut Rig::new(&dsdt()), None);
    assert_eq!(
        (full.reg_failed, full.pic, full.osc, full.ini),
        (0, Some(Ok(())), [true, true], [1, 0, 0, 0])
    );
    let mut r = Rig::new(&dsdt());
    bring_up(&mut r, None);
    let pci = at(&r.aml, "\\_SB.PCI0").unwrap();
    let want = format!("{:?}", routes(&mut r.aml, &mut evaluator(&mut r.probe), pci));
    let mut tally = [0usize; 4];
    for (p, done) in tally.iter_mut().enumerate() {
        for steps in 1..full.used[p] {
            let what = format!("phase {p} cut at {steps}");
            let mut r = Rig::new(&dsdt());
            let d = bring_up(&mut r, Some((p, steps)));
            match p {
                0 => {
                    assert!(d.reg_failed > 0 && d.lines[3].starts_with("_REG failed in"), "{what}")
                }
                1 => assert!(
                    matches!(d.pic, Some(Err(_))) && d.lines[4].contains("faulted at"),
                    "{what}"
                ),
                2 => assert!(d.osc.contains(&false), "{what}"),
                _ => assert!(
                    d.ini[1] + d.ini[3] > 0 && d.lines[5] != full.lines[5],
                    "{what}: {:?}",
                    d.lines
                ),
            }
            let same = [d.reg_failed == 0, d.pic == full.pic, d.osc == full.osc, d.ini == full.ini];
            assert!((0..4).all(|q| q == p || same[q]), "{what}: other phases");
            assert!(r.aml.rt.mutexes.is_empty() && r.aml.rt.global_lock == 0, "{what}: held");
            let pci = at(&r.aml, "\\_SB.PCI0").unwrap();
            assert_eq!(
                format!("{:?}", routes(&mut r.aml, &mut evaluator(&mut r.probe), pci)),
                want,
                "{what}"
            );
            if p == 0 {
                assert!(r.connect(3).is_empty(), "{what}: told at the next connect");
                let ec = at(&r.aml, "\\_SB.PCI0.EC0").unwrap();
                assert!(r.aml.rt.announced.contains(&(ec, Space::EMBEDDED_CONTROL.0)), "{what}");
            }
            *done += 1;
        }
    }
    assert_eq!(tally, [21, 31, 4, 33], "every step of each phase cut: {:?}", full.used);
}
