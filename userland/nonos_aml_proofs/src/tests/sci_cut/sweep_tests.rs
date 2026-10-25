// NONOS Operating System (AGPL-3.0-or-later)
//! Every scene's GPE method run uncut, then cut at every step below what it
//! used; then the drain, each delivery's HND cut the same. The model in
//! `mod.rs`, checked on the GPE's register writes before and after the
//! method, its status and enable bits, the armed set, what is held, and
//! the notifications queued and delivered. Floors are our choice.

use super::fixture::scene;
use super::run::{drain, sci, start};
use crate::fadt_enc::GPE0;
use crate::probe::Probe;
use crate::tests::sci_rig::bits;

#[test]
fn a_cut_gpe_method_is_rearmed_and_loses_no_notification() {
    let mut tally = [0usize; 2];
    for seed in 0..20 {
        let sc = scene(seed);
        let (mut rig, mut aml, g) = start(&sc);
        let (_, _, used) = sci(&mut rig, &mut aml, &mut Probe::default(), g, 1_000_000);
        for steps in 1..used {
            let what = format!("seed {seed} cut at {steps}");
            let (mut rig, mut aml, g) = start(&sc);
            let mut probe = Probe::default();
            let (report, [before, after], _) = sci(&mut rig, &mut aml, &mut probe, g, steps);
            let (bit, en) = (1u16 << g, GPE0 + 8);
            let (b, a) = match sc.level {
                true => (vec![(en, 0)], vec![(GPE0, bit), (en, bit)]),
                false => (vec![(en, 0), (GPE0, bit)], vec![(en, bit)]),
            };
            assert_eq!((before, after), (b, a), "{what}: the order");
            assert!(report.disabled.is_empty() && rig.events.armed.contains(&g), "{what}: armed");
            assert_eq!(bits(&rig.regs, g), (false, true), "{what}: status clear, enabled");
            let queued: Vec<u8> = aml.rt.notifications.iter().map(|n| n.1).collect();
            let said = |n: u64| probe.debug.contains(&format!("Integer({n})"));
            let least = [0x80u8, 0x81][..usize::from(said(1)) + usize::from(said(2))].to_vec();
            assert!(
                queued == least || queued == [0x80, 0x81][..least.len() + 1],
                "{what}: {queued:?}"
            );
            let delivered = drain(&mut aml, &mut probe, steps);
            assert!(delivered.starts_with(&queued), "{what}: each once, in order");
            let more = &delivered[queued.len()..];
            assert!(
                more.iter().all(|v| *v == 0x90)
                    && more.len() <= usize::from(queued.contains(&0x80)),
                "{what}"
            );
            assert!(aml.rt.mutexes.is_empty() && aml.rt.global_lock == 0, "{what}: held");
            tally[0] += 1;
            tally[1] += usize::from(!queued.is_empty());
        }
    }
    let floors = [570, 535];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
