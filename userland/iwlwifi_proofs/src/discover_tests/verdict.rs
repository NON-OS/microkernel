// NONOS Operating System (AGPL-3.0-or-later)

//! A machine with an Intel Wi-Fi card this driver does not run is not a
//! machine without one: discovery says which card it is and the driver gives
//! up on it (`EXIT_GAVE_UP`), rather than leaving as if nothing were there
//! (`EXIT_ABSENT`). Only a machine with no Intel Wi-Fi function is absent.

use crate::firmware::carried::runs;
use crate::firmware::generation::name;
use crate::pci_match::Candidate;
use crate::verdict::{not_run_line, verdict, Verdict, LINE_MAX};

fn intel(device: u16) -> Candidate {
    Candidate { is_pci: true, vendor: 0x8086, device, class: 0x02, subclass: 0x80, bar0_mmio: true }
}

#[test]
fn every_intel_wifi_id_is_run_or_given_up_never_absent() {
    for device in 0..=u16::MAX {
        let want = if runs(device) { Verdict::Run(0) } else { Verdict::NotRun(device) };
        assert_eq!(verdict([intel(device)]), want, "{device:#06x}");
    }
}

#[test]
fn only_a_machine_without_an_intel_wifi_function_is_absent() {
    assert_eq!(verdict([]), Verdict::Absent);
    let ethernet = Candidate { subclass: 0x00, device: 0x15F3, ..intel(0) };
    let realtek = Candidate { vendor: 0x10EC, device: 0xC821, ..intel(0) };
    assert_eq!(verdict([ethernet, realtek]), Verdict::Absent);
    assert_eq!(verdict([ethernet, intel(0x2526)]), Verdict::NotRun(0x2526), "a 9260");
    let no_bar = Candidate { bar0_mmio: false, ..intel(0x51F0) };
    assert_eq!(verdict([no_bar]), Verdict::NotRun(0x51F0), "an AX211 with no register BAR");
}

#[test]
fn a_card_that_runs_is_brought_up_whatever_comes_before_it() {
    assert_eq!(verdict([intel(0x2526), intel(0x51F0)]), Verdict::Run(1));
    assert_eq!(verdict([intel(0x24FD), intel(0x2526)]), Verdict::NotRun(0x24FD), "the first");
}

#[test]
fn the_line_names_the_card_whole_and_nothing_else() {
    for device in [0x2526u16, 0x24FD, 0x7FFF] {
        let (buf, len) = not_run_line(device);
        let line = core::str::from_utf8(&buf[..len]).unwrap();
        assert!(len < LINE_MAX && line.ends_with('\n') && line.matches('\n').count() == 1);
        assert!(line.starts_with(&format!("[iwlwifi] pci 8086:{device:04x} ")), "{line}");
        let what = name(device).map_or("an Intel Wi-Fi id this driver does not know", |g| g.what);
        assert!(line.contains(what), "{line}");
        assert!(line.trim_end().ends_with(": present, not run by this driver; given up"), "{line}");
    }
}
