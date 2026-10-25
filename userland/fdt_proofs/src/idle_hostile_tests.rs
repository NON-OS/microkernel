// NONOS Operating System (AGPL-3.0-or-later)
//! Idle-state trees past what is kept, and damaged ones: refused whole, never
//! cut short, and never a panic or an index past the table.

use crate::arch::fdt::find::idle::{Refusal, MAX_PER_CPU, MAX_STATES};
use crate::builder::cells;
use crate::idle_tree::{read, standby, tree};

#[test]
fn more_states_than_kept_is_refused() {
    let names: Vec<String> = (0..=MAX_STATES).map(|i| format!("s{i}")).collect();
    let nodes: Vec<_> = names.iter().zip(1..).map(|(n, ph)| standby(n, ph, 1, 100)).collect();
    let cpus = [(Some(0), cells(&[1]))];
    assert_eq!(read(&tree(Some("psci"), &nodes, &cpus)), Err(Refusal::TooManyStates));
    assert!(read(&tree(Some("psci"), &nodes[..MAX_STATES], &cpus)).is_ok());
}

#[test]
fn a_cpu_naming_more_than_kept_is_refused() {
    let names: Vec<String> = (0..=MAX_PER_CPU).map(|i| format!("s{i}")).collect();
    let nodes: Vec<_> = names.iter().zip(1..).map(|(n, ph)| standby(n, ph, 1, 100)).collect();
    let all: Vec<u32> = (1..=MAX_PER_CPU as u32 + 1).collect();
    let blob = tree(Some("psci"), &nodes, &[(Some(0), cells(&all))]);
    assert_eq!(read(&blob), Err(Refusal::TooManyForCpu));
    let blob = tree(Some("psci"), &nodes, &[(Some(0), cells(&all[..MAX_PER_CPU]))]);
    assert_eq!(read(&blob).unwrap().1, vec![(0..MAX_PER_CPU as u8).collect::<Vec<_>>()]);
}

#[test]
fn single_byte_damage_never_panics_or_points_past_the_table() {
    let mut sleep = standby("sleep", 2, 0x1_0000, 900);
    sleep.1.push(("local-timer-stop", vec![]));
    let nodes = [standby("ret", 1, 1, 100), sleep];
    let cpus = [(Some(0), cells(&[1, 2])), (Some(1), cells(&[2]))];
    let blob = tree(Some("psci"), &nodes, &cpus);
    let mut copy = blob.clone();
    for at in 40..blob.len() {
        for value in [0x00, 0xff, blob[at] ^ 0x01, blob[at] ^ 0x80] {
            copy[at] = value;
            if let Ok((states, lists)) = read(&copy) {
                assert!(states.len() <= MAX_STATES);
                for list in lists {
                    assert!(list.len() <= MAX_PER_CPU);
                    assert!(list.iter().all(|&i| usize::from(i) < states.len()));
                }
            }
        }
        copy[at] = blob[at];
    }
}
