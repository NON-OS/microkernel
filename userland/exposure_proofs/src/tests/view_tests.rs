// NONOS Operating System (AGPL-3.0-or-later)
//! What `audit` and `audit --agent` print: the columns say the verdicts,
//! the device column is labelled capability bits, and the authority line
//! names the signer of the ceiling.

use super::model::{entry, kernel_bits, masks};
use crate::command::builtin::audit::agent::agent;
use crate::command::builtin::audit::format::HEADING;
use crate::command::builtin::audit::join::join;
use crate::command::builtin::audit::reach::{reaches, AREAS, DEVICES_NOTE};
use crate::command::builtin::audit::table::line;

fn text(b: &[u8]) -> String {
    String::from_utf8(b.to_vec()).unwrap()
}

#[test]
fn each_column_says_its_verdict() {
    let at: Vec<usize> = ["network", "storage", "devices*"]
        .iter()
        .map(|h| text(HEADING[0]).find(h).unwrap())
        .collect();
    for m in masks().into_iter().take(2_000) {
        let rows = join(&entry(9, m, 255), &[(9, b"app.x", false)]).unwrap();
        let l = text(&line(&rows[0]));
        for (i, area) in AREAS.iter().enumerate() {
            let cell = &l[at[i]..at[i] + 3];
            assert_eq!(cell == "YES", reaches(m, area), "{m:#x}: {l}");
        }
        assert!(l.contains("publisher"));
    }
    let unsigned = join(&[], &[(4, b"init", false)]).unwrap();
    assert!(text(&line(&unsigned[0])).contains("no signed ceiling"));
    assert!(text(DEVICES_NOTE).contains("not devices claimed now"));
}

#[test]
fn an_agent_shows_what_opens_each_area_and_who_signed_it() {
    let k = kernel_bits();
    let m = k["Network"] | k["Dma"] | k["Driver"] | k["IPC"];
    let regs = [entry(5, m, 0), entry(6, 0, 3)].concat();
    let rows =
        join(&regs, &[(5, b"net.core", false), (6, b"net.core", false), (7, b"app.y", false)])
            .unwrap();
    let out: Vec<String> = agent(&rows, b"net.core").iter().map(|l| text(l)).collect();
    assert_eq!(out.iter().filter(|l| l.starts_with("net.core, pid ")).count(), 2);
    assert!(out.iter().any(|l| l.contains("network    YES, through Network")));
    assert!(out.iter().any(|l| l.contains("devices*   YES, through Driver, Dma")));
    assert!(out.iter().any(|l| l.contains("storage    no")));
    assert!(out.iter().any(|l| l.contains("signed by vendor")));
    assert!(out.iter().any(|l| l.contains("signed by developer")));
    assert_eq!(out.last().map(|l| l.as_bytes()), Some(DEVICES_NOTE));
    assert!(agent(&rows, b"nobody").is_empty());
}
