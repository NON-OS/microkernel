// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded firmware against the model, for functions 0 to 11: the
//! answer, and the calls made, which must never include a function the
//! mask does not offer. Seeds and floors are our choice, from no source.

use super::dsm_fixture::source;
use super::dsm_gen::{fw, Form};
use super::dsm_model::ask as model;
use crate::consts::at;
use crate::object::Object;
use crate::pci_dsm::ask;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

const SEEDS: u64 = 2000;

#[test]
fn every_firmware_is_asked_only_what_it_offers() {
    // Answered, offered at 3, fell back to 2, refused by the mask, served
    // by an integer mask, and a wrong UUID.
    let mut tally = [0usize; 6];
    for seed in 0..SEEDS {
        let fw = fw(seed);
        let (mut aml, _) = boot(&source(&fw), 2);
        let dev = at(&aml, "\\_SB.DEV0").unwrap();
        for f in 0..12 {
            let mut probe = Probe::default();
            let got = ask(&mut aml, &mut evaluator(&mut probe), dev, f);
            let (want, calls) = model(&fw, f);
            let got = got.map(|o| match o {
                Object::Integer(n) => n,
                o => panic!("seed {seed} f {f}: {o:?}"),
            });
            assert_eq!(got, want, "seed {seed} f {f} {:?}", fw.forms);
            let said: Vec<String> =
                calls.iter().map(|(r, f)| format!("Integer({})", r << 8 | f)).collect();
            assert_eq!(probe.debug, said, "seed {seed} f {f}");
            let asked0 = calls.iter().filter(|c| c.1 == 0).count();
            tally[0] += usize::from(want.is_some());
            tally[1] += usize::from(want.is_some() && calls[0] == (3, 0) && asked0 == 1);
            tally[2] += usize::from(want.is_some() && asked0 == 2);
            tally[3] += usize::from(want.is_none() && asked0 > 0 && calls.len() == asked0);
            let int = calls.last().map(|c| matches!(fw.forms[c.0 as usize], Form::Int(_)));
            tally[4] += usize::from(want.is_some() && int == Some(true));
        }
        tally[5] += usize::from(fw.uuid != crate::pci_dsm::PCI_DSM_UUID);
    }
    let floors = [800, 450, 300, 4000, 350, 650];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
