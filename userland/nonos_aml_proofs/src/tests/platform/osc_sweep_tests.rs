// NONOS Operating System (AGPL-3.0-or-later)
//! F-110: _OSC against hostile firmware. Each seeded answer is held to a
//! model (ACPI 6.5, 6.2.11; recalled, unverified): not a buffer of at
//! least the words sent is refused as Shape; an unknown UUID, an unknown
//! revision, then a failure bit, refuse in that order; otherwise the
//! status word is kept and each capability word is cut to the one sent.
//! The query carries the query bit, the commit asks no more than was asked,
//! offered and allowed, and NONOS ends holding no control it did not claim.

use super::osc_sweep_gen::{answer, ask, roll};
use crate::load::Aml;
use crate::object::Object;
use crate::platform::{offer_then_claim, PCI_HOST_UUID, QUERY};

#[path = "osc_sweep_model.rs"]
mod model;
use model::{model, named, words};

#[test]
fn osc_never_ends_holding_what_was_not_claimed() {
    let (mut ok, mut refused, mut cut) = (0, 0, 0);
    for seed in 1..=4000u64 {
        let s = &mut seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let (asked, claim) = ask(s);
        let answers = [answer(s, asked.len()), answer(s, asked.len())];
        let (mut sent, mut calls) = (Vec::new(), 0usize);
        let mut fw = |_: &mut Aml, _, args: Vec<Object>| {
            sent.push(words(&args[3]).unwrap());
            calls += 1;
            answers[calls - 1].clone()
        };
        let got =
            offer_then_claim(&mut Aml::default(), &mut fw, 0, (&PCI_HOST_UUID, 1), &asked, &claim);
        let mut query = asked.clone();
        query[0] = QUERY;
        let want = model(&query, &answers[0]).and_then(|offered| {
            let commit: Vec<u32> = (0..asked.len())
                .map(|i| if i == 0 { 0 } else { asked[i] & offered[i] & claim[i] })
                .collect();
            model(&commit, &answers[1]).map(|granted| (offered, granted, commit))
        });
        assert_eq!(sent[0], query, "seed {seed}: the query");
        match (&got, &want) {
            (Ok((o, g)), Ok((wo, wg, commit))) => {
                assert_eq!((o, g), (wo, wg), "seed {seed}");
                assert_eq!(sent[1], *commit, "seed {seed}: the commit");
                assert!((1..g.len()).all(|i| g[i] & !(asked[i] & claim[i]) == 0), "seed {seed}");
                cut +=
                    usize::from(words(answers[1].as_ref().unwrap()).unwrap()[1..g.len()] != g[1..]);
                ok += 1;
            }
            (Err(e), Err(w)) => {
                assert_eq!(named(e), *w, "seed {seed}");
                refused += 1;
            }
            _ => panic!("seed {seed}: {got:?} against {want:?}"),
        }
        let _ = roll(s, 2);
    }
    assert!(ok >= 1000 && refused >= 1000 && cut >= 500, "{ok} ok, {refused} refused, {cut} cut");
}
