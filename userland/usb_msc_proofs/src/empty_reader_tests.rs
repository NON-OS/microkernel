// NONOS Operating System (AGPL-3.0-or-later)
//! A modelled empty single-slot card reader. It answers INQUIRY, then fails
//! TEST UNIT READY with MEDIUM NOT PRESENT. It used to be asked six hundred
//! times, about thirty-five seconds with no request answered, and again
//! every five seconds after. Here it is passed on its answer a second after
//! it was configured (a card still starting up says the same), every
//! request in between is answered at once, and the reader is asked again
//! only after a wait that doubles.

use crate::clock::at;
use crate::disk::{test_ready, Unit};
use crate::fixture::{disk, sense, turs};
use crate::protocol::{E_AGAIN, E_NODEV, E_NOMEDIUM};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{idle, read};
use crate::state::State;
use crate::sticks::plug;
use crate::xhci::{device, script, Answer};

/// INQUIRY answered, then TEST UNIT READY failed with `key`, ASC 3Ah, twice.
fn empty_slot(key: u8) -> Vec<Answer> {
    let ok = || Answer::Csw { residue: 0, status: 0 };
    let empty = || [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense(key, 0x3A)), ok()];
    [Answer::Data(vec![0; 36]), ok()].into_iter().chain(empty()).chain(empty()).collect()
}

#[test]
fn an_empty_slot_is_said_on_its_first_answer_under_not_ready_or_unit_attention() {
    for key in [0x02, 0x06] {
        script(empty_slot(key).split_off(2));
        let got = test_ready(&disk(), &mut State::new(), &mut false);
        assert_eq!(got, Ok(Unit::NoMedium), "sense key {key}");
        assert_eq!(turs(), 1);
    }
}

#[test]
fn an_empty_reader_leaves_the_server_answering_and_waits_to_be_asked_again() {
    plug((0..4).flat_map(|_| empty_slot(0x02)).collect());
    let mut server = Server::new();
    let (start, mut probed) = (at(), vec![]);
    while at() - start < 30_000 {
        let status = read(&mut server);
        assert!(status == E_AGAIN || status == E_NODEV, "answered {status}");
        if at() - start > 2_000 {
            assert_eq!(status, E_NODEV, "the search is over: the kernel asks the next disk");
        }
        assert!(idle(&mut server) <= 20, "a step of an empty reader is a few calls");
        if device(|d| d.enabled as usize) > probed.len() {
            probed.push(at() - start);
        }
    }
    let gaps: Vec<u64> = probed.windows(2).map(|w| w[1] - w[0]).collect();
    assert_eq!(gaps.len(), 2, "asked at {probed:?}");
    assert!((5_000..10_100).contains(&gaps[0]), "first wait {gaps:?}");
    assert!((10_000..15_100).contains(&gaps[1]), "then twice that {gaps:?}");
    assert!(matches!(server.scanner.medium, Medium::Absent));
    let report = server.scanner.report;
    assert_eq!((report.stage, report.detail), (Stage::NoMedium, E_NOMEDIUM));
    assert_eq!(device(|d| d.disabled.len()), probed.len(), "every slot given back");
    assert_eq!(turs(), 2 * probed.len(), "two TEST UNIT READYs a probe");
}

#[test]
fn a_card_still_starting_up_is_not_passed_over() {
    let mut answers = empty_slot(0x02);
    answers.truncate(5);
    answers.extend(crate::fixture::ready_unit(1000).split_off(2));
    plug(answers);
    let mut server = Server::new();
    crate::serving::until_ready(&mut server, 2_000);
    assert_eq!(device(|d| d.enabled), 1, "bound on its first probe, while the kernel waits");
}
