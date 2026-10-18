// NONOS Operating System (AGPL-3.0-or-later)
//! Before the kernel holds a sector, a READ whose phase was lost answers
//! E_AGAIN (server/handlers/block.rs), and the kernel's disk selection asks
//! again. A stick that loses every READ's CSW must not keep it asking for
//! ever: three failed requests let it go (scan/keep.rs), and its port then
//! waits, answering E_NODEV, so the refusals come in short runs.

use crate::answers::passes_check;
use crate::clock::{advance, at};
use crate::fixture::{answers_left, ready_unit};
use crate::protocol::{E_AGAIN, E_NODEV, E_TIMEDOUT};
use crate::scan::Medium;
use crate::serving::read;
use crate::sticks::bound;
use crate::xhci::{setup, Answer};

#[test]
fn a_stick_losing_every_reads_csw_before_it_is_held_is_not_refused_for_ever() {
    // The kernel asks every 200 ms for 15 minutes.
    let mut server = bound(1000);
    let t0 = at();
    let (mut said, mut run_start, mut longest) = (vec![], None::<u64>, 0u64);
    while at() - t0 < 900_000 {
        if matches!(server.scanner.medium, Medium::Ready(_)) {
            let lost = [Answer::Data(vec![0; 512]), Answer::Data(vec![0; 13])];
            setup(|d| d.answers = lost.into_iter().chain(passes_check(1000)).collect());
        } else if answers_left() == 0 {
            setup(|d| d.answers = ready_unit(1000).into());
        }
        let status = read(&mut server);
        if status == E_AGAIN || status == E_TIMEDOUT {
            let start = *run_start.get_or_insert(at());
            longest = longest.max(at() - start);
        } else {
            run_start = None;
        }
        if said.last() != Some(&status) {
            said.push(status);
        }
        advance(200);
    }
    assert!(said.contains(&E_NODEV), "never E_NODEV: {said:?}");
    assert!(longest < 60_000, "refused for {longest} ms in a row");
}
