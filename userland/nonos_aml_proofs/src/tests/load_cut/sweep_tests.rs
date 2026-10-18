// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded table loaded uncut, then cut at every step below what the
//! uncut load used. The model: what a later evaluation sees is the
//! namespace before the load or the one the uncut load made, never a
//! third; an EC device marked told has said its _REG; no _REG says twice;
//! and loading again with a full budget reaches the uncut namespace with
//! every EC device told. Seeds and floors are our choice, from no source.

use super::gen::{scene, Piece};
use super::snap::{load, rig, snap};

#[test]
fn a_cut_load_leaves_the_namespace_before_or_after_never_between() {
    // Cut runs left before; left after; EC devices told on the retry.
    let mut tally = [0usize; 3];
    for seed in 0..300 {
        let sc = scene(seed);
        let mut r = rig(&sc);
        let before = snap(&mut r);
        let (cut, used, _) = load(&mut r, &sc, 1_000_000);
        assert!(!cut, "seed {seed} {sc:?}");
        let after = snap(&mut r);
        assert_ne!(before, after, "seed {seed}: the table loaded");
        let ecs: Vec<usize> = sc
            .pieces
            .iter()
            .enumerate()
            .filter(|(_, p)| matches!(p, Piece::Ec))
            .map(|(i, _)| i)
            .collect();
        for steps in 1..used {
            let what = format!("seed {seed} cut at {steps}");
            let mut r = rig(&sc);
            let (_, _, error) = load(&mut r, &sc, steps);
            let seen = snap(&mut r);
            assert!(seen == before || seen == after, "{what}: {sc:?}");
            if sc.path == 0 && seen == before {
                assert_eq!(error, Some(crate::error::ErrorKind::Budget), "{what}: said so");
            }
            tally[usize::from(seen == after)] += 1;
            for &i in &ecs {
                let said = r
                    .probe
                    .debug
                    .iter()
                    .filter(|d| **d == format!("Integer({})", 0x100 + i))
                    .count();
                let id = crate::consts::at(&r.aml, &format!("\\E{i:02}"));
                let told = id.is_some_and(|id| r.aml.rt.announced.contains(&(id, 3)));
                assert!(
                    said <= 1 && (!told || said == 1),
                    "{what}: E{i:02} told {told} said {said}"
                );
            }
            load(&mut r, &sc, 1_000_000);
            assert_eq!(snap(&mut r), after, "{what}: loading again");
            for &i in &ecs {
                let id = crate::consts::at(&r.aml, &format!("\\E{i:02}")).unwrap();
                assert!(r.aml.rt.announced.contains(&(id, 3)), "{what}: E{i:02} told at last");
                tally[2] += 1;
            }
        }
    }
    let floors = [4700, 1000, 3900];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
