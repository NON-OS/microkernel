// NONOS Operating System (AGPL-3.0-or-later)
//! Every scene's _TMP cut at every step below what it used: a reading the
//! model believes, or nothing, never a wrong number; nothing held after.

use super::fixture::{dsdt, scenes};
use super::reader::{believed, held, limited, Log};
use crate::consts::at;
use crate::probe::Probe;
use crate::run::boot;
use crate::thermal::temperature;

#[test]
fn a_cut_or_lying_temperature_is_no_reading() {
    let (mut cut, mut lies_seen) = (0, 0);
    for (lies, mutex) in scenes().into_iter().flat_map(|l| [(l, false), (l, true)]) {
        let (mut aml, _) = boot(&dsdt(&lies, mutex), 2);
        let tz = at(&aml, "\\_TZ.TZ0").unwrap();
        let tmp = at(&aml, "\\_TZ.TZ0._TMP").unwrap();
        for steps in 1.. {
            let mut log = Log::new();
            let r = temperature(&mut aml, &mut limited(&mut Probe::default(), steps, &mut log), tz);
            assert!(!held(&aml), "{lies:?} {steps}: held");
            let [(id, run)] = log[..] else { panic!("one run") };
            assert_eq!(id, tmp);
            assert_eq!(r, believed(run), "{lies:?} {mutex} {steps}");
            if run.is_some() {
                lies_seen += usize::from(r.is_none());
                break;
            }
            cut += 1;
        }
    }
    println!("_TMP: {cut} cut reads, {lies_seen} finished lies read as nothing");
    assert!(cut >= 890 && lies_seen == 8, "{cut} {lies_seen}");
}
