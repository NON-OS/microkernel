// NONOS Operating System (AGPL-3.0-or-later)
//! Every reader the class drivers bind on, over configurations told one
//! lie: each finds what the model finds in the descriptors the walk owes,
//! so an honest descriptor after a lie is still found where it stands.

use crate::desc::{find_functional, functional, interfaces, notify_ep, union_data};
use crate::hub_status::status_endpoint;
use crate::sweep::build::{config, cs_least};
use crate::sweep::lie::tell;
use crate::sweep::model;
use crate::sweep::rng::{rounds, Rng};

const SUBTYPES: [u8; 6] = [0x00, 0x01, 0x02, 0x06, 0x0F, 0x1A];

#[test]
fn every_reader_finds_what_the_model_finds_after_a_lie() {
    let mut found = 0u32;
    for seed in 1..rounds(20_000) {
        let mut r = Rng::new(seed);
        let built = config(&mut r);
        let t = tell(&mut r, &built);
        let (raw, owed) = (t.descs.concat(), &t.descs[..t.framed]);
        let ifs = interfaces(&raw);
        let hub = status_endpoint(&raw).map(|s| (s.address, s.max_packet, s.interval));
        if !t.ends {
            continue;
        }
        let why = format!("seed {seed}: {:?}", t.lie);
        assert_eq!(ifs, model::interfaces(owed), "{why}");
        assert_eq!(hub, model::status(owed), "{why}");
        for _ in 0..4 {
            let any = r.byte();
            let number = r.pick(&[0, 1, 2, any]);
            let want = model::functional(owed, number);
            assert_eq!(functional(&raw, number), want, "{why}, interface {number}");
            for sub in SUBTYPES {
                let least = cs_least(sub);
                let got = find_functional(&raw, number, sub, least);
                let honest = want.iter().find(|d| d[2] == sub && d.len() >= least);
                assert_eq!(got, honest.map(Vec::as_slice), "{why}, {number} subtype {sub}");
                found += got.is_some() as u32;
            }
            let union = want.iter().find(|d| d[2] == 0x06 && d.len() >= 5).map(|d| d[4]);
            assert_eq!(union_data(&raw, number), union, "{why}, union of {number}");
            assert_eq!(notify_ep(&raw, number), model::notify(owed, number), "{why}");
        }
    }
    assert!(cfg!(miri) || found > 10_000, "functional descriptors found only {found} times");
}
