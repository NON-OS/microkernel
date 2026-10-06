// NONOS Operating System (AGPL-3.0-or-later)
//! The driver serves the disk the kernel block layer will accept as NONOS,
//! wherever it sits among the controllers and ports, and falls back to the
//! first port in walk order only when no disk carries NONOS.

use crate::choose::candidate::{Candidate, Choice};
use crate::choose::layout::{holds, starts_with_magic, PLAN_LBA, STORE_LBA, STORE_MAGIC};
use crate::choose::pick::choose;

fn c(controller: usize, port: u8, store: bool, plan: bool) -> Candidate {
    Candidate { controller, port, store, plan }
}

#[test]
fn nonos_disk_on_second_controller_wins_over_first_controller() {
    /*
     * The q35 shape seen on a live boot: the ESP on the first controller in
     * device list order, the NONOS data disk on the second.
     */
    let cands = [c(0, 0, false, false), c(1, 0, true, false)];
    assert_eq!(choose(&cands), Some(Choice { index: 1, fallback: false }));
}

#[test]
fn plan_magic_alone_marks_the_disk() {
    let cands = [c(0, 0, false, false), c(0, 2, false, true)];
    assert_eq!(choose(&cands), Some(Choice { index: 1, fallback: false }));
}

#[test]
fn nonos_disk_on_a_later_port_of_one_controller_wins() {
    let cands = [c(0, 0, false, false), c(0, 1, false, false), c(0, 5, true, true)];
    assert_eq!(choose(&cands), Some(Choice { index: 2, fallback: false }));
}

#[test]
fn several_nonos_disks_pick_lowest_controller_then_port_in_any_order() {
    let cands = [c(1, 0, true, false), c(0, 3, false, true), c(0, 1, true, false)];
    assert_eq!(choose(&cands), Some(Choice { index: 2, fallback: false }));
}

#[test]
fn blank_disks_fall_back_to_first_port_in_walk_order() {
    let cands = [c(1, 0, false, false), c(0, 4, false, false), c(0, 2, false, false)];
    assert_eq!(choose(&cands), Some(Choice { index: 2, fallback: true }));
}

#[test]
fn no_port_came_up_means_no_choice() {
    assert_eq!(choose(&[]), None);
}

#[test]
fn a_structure_past_the_end_of_the_disk_is_not_read() {
    assert!(!holds(STORE_LBA, STORE_LBA));
    assert!(holds(STORE_LBA + 1, STORE_LBA));
    assert!(!holds(PLAN_LBA, PLAN_LBA));
    assert!(holds(PLAN_LBA + 1, PLAN_LBA));
}

#[test]
fn magic_must_open_the_sector() {
    let mut sector = [0u8; 512];
    assert!(!starts_with_magic(&sector, STORE_MAGIC));
    sector[..8].copy_from_slice(b"NONOSTR1");
    assert!(starts_with_magic(&sector, STORE_MAGIC));
    assert!(!starts_with_magic(&sector[1..], STORE_MAGIC));
    assert!(!starts_with_magic(&sector[..7], STORE_MAGIC));
}
