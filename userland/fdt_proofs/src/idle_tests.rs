// NONOS Operating System (AGPL-3.0-or-later)
//! The tree's idle states: each value as the tree has it, only states the
//! kernel may enter kept, and each CPU's list resolved in `find`'s order.

use crate::arch::fdt::find::idle::IdleState;
use crate::builder::{cells, text};
use crate::idle_tree::{read, standby, tree};

#[test]
fn values_come_from_the_tree_and_lists_follow_find() {
    let mut sleep = standby("cpu-sleep", 2, 0x0001_0000, 900);
    sleep.1.push(("local-timer-stop", vec![]));
    let mut off = standby("off", 3, 0x0000_0002, 50);
    off.1.push(("status", text("disabled")));
    let other = ("x", vec![("compatible", text("arm,other")), ("phandle", cells(&[4]))]);
    let mut short = standby("short", 5, 7, 5);
    short.1.retain(|(k, _)| *k != "arm,psci-suspend-param");
    let nodes = [standby("ret", 1, 0x0000_0001, 100), sleep, off, other, short];
    let cpus = [
        (Some(0), cells(&[1, 2, 3, 4, 5])),
        (None, cells(&[1])),
        (Some(0x100), cells(&[2, 9])),
        (Some(0x101), vec![]),
    ];
    let (states, lists) = read(&tree(Some("psci"), &nodes, &cpus)).unwrap();
    let ret = IdleState {
        phandle: 1,
        param: 1,
        entry_us: 10,
        exit_us: 20,
        residency_us: 100,
        timer_stops: false,
    };
    let sleep =
        IdleState { phandle: 2, param: 0x1_0000, residency_us: 900, timer_stops: true, ..ret };
    assert_eq!(states, vec![ret, sleep]);
    assert_eq!(lists, vec![vec![0, 1], vec![1], vec![]]);
}

#[test]
fn no_psci_entry_method_keeps_nothing() {
    let nodes = [standby("ret", 1, 1, 100)];
    let cpus = [(Some(0), cells(&[1]))];
    for method in [None, Some("spin"), Some("psci2")] {
        let (states, lists) = read(&tree(method, &nodes, &cpus)).unwrap();
        assert!(states.is_empty());
        assert_eq!(lists, vec![Vec::<u8>::new()]);
    }
    assert_eq!(read(&tree(Some("arm,psci"), &nodes, &cpus)).unwrap().0.len(), 1);
}

#[test]
fn a_value_of_the_wrong_length_is_refused() {
    use crate::arch::fdt::{find::idle::Refusal, FdtError};
    let keys = ["phandle", "arm,psci-suspend-param", "entry-latency-us", "exit-latency-us"];
    for key in keys.into_iter().chain(["min-residency-us"]) {
        for bad in [vec![], vec![0; 3], cells(&[1, 2])] {
            let mut node = standby("ret", 1, 1, 100);
            node.1.iter_mut().filter(|(k, _)| *k == key).for_each(|(_, v)| *v = bad.clone());
            let blob = tree(Some("psci"), &[node], &[(Some(0), cells(&[1]))]);
            assert_eq!(read(&blob), Err(Refusal::Tree(FdtError::BadCells)), "{key}");
        }
    }
    let blob = tree(Some("psci"), &[standby("ret", 1, 1, 100)], &[(Some(0), vec![0, 0, 1])]);
    assert_eq!(read(&blob), Err(Refusal::Tree(FdtError::BadCells)));
}
