// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The Admin split, held against the real cap table and the three signed
//! masks as Capsule.mk declares them. Each capsule's calls pass with its mask
//! and fail once its new bit is taken out; none of the three can grant,
//! revoke from another process, end a process it does not parent or open a
//! window; Admin opens none of the split calls; and init's ambient set holds
//! none of the bits.

use super::cap_table::is_allowed;
use crate::capabilities::{bits_to_caps, Capability, CapabilityToken};
use crate::proofs::proofs_9::formal_model::grant_rule;
use crate::syscall::abi::REGISTRY;
use crate::syscall::numbers::SyscallNumber as N;

#[path = "../../../../../src/process/core/table/ambient.rs"]
mod ambient;

const ACPI_MK: &str = include_str!("../../../../capsule_acpi/Capsule.mk");
const POLICY_MK: &str = include_str!("../../../../capsule_policy/Capsule.mk");
const POWER_MK: &str = include_str!("../../../../capsule_power/Capsule.mk");
const ACPI_SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_acpi/src");

/// The ACPI owner's calls: the cap table's AcpiOwner arm, and the dispatch's
/// owner-only list (owner_gate.rs).
#[rustfmt::skip]
const ACPI_OWNER: [N; 25] = [
    N::MkAcpiTable, N::MkAcpiRegion, N::MkAcpiPort, N::MkAcpiPci, N::MkAcpiPower,
    N::MkAcpiEvent, N::MkAcpiSleep, N::MkSmbiosTable, N::MkAcpiCpu, N::MkAcpiHolds,
    N::MkPciProbeFunction, N::MkPciDepart, N::MkAcpiFunctionOf, N::MkAcpiHwError,
    N::MkAcpiPlatformDevice, N::MkAcpiUsbPorts, N::MkAcpiExternalPort, N::MkAcpiReserved,
    N::MkAcpiProximity, N::MkAcpiOscGrant, N::MkAcpiLpit, N::MkAcpiFirstPass, N::MkAcpiSlot,
    N::MkAcpiLoaded, N::MkAcpiThermalLimit,
];

/// Every other kernel call capsule_acpi makes, through libc or by its tag.
#[rustfmt::skip]
const ACPI_OTHER: [N; 15] = [
    N::AdminShutdown, N::MkAcpiDeviceName, N::MkDeviceList, N::MkDeviceClaim, N::MkIrqBind,
    N::MkIrqAck, N::MkIrqPoll, N::MkIpcRecvFrom, N::MkIpcReply, N::MkServiceLookup,
    N::MkThreadGroup, N::MkTimeMonotonic, N::MkFutexWait, N::MkExit, N::MkYield,
];

#[rustfmt::skip]
const POLICY_CALLS: [N; 9] = [
    N::AdminPolicyPush, N::MkIpcRecvFrom, N::MkIpcReply, N::MkServiceLookup,
    N::MkServiceRegister, N::MkTimeMonotonic, N::MkGetPid, N::MkExit, N::MkYield,
];

#[rustfmt::skip]
const POWER_CALLS: [N; 7] = [
    N::AdminReboot, N::AdminShutdown, N::MkIpcRecvFrom, N::MkIpcReply, N::MkIpcCall,
    N::MkTimeMillis, N::MkExit,
];

/// Admin's calls the cap table refuses the three. MkCapRevoke passes the
/// table with any valid token (a process may drop its own bits); its handler
/// asks Admin for any other target (`grant_rule::revoke_decision`).
const ADMIN_ONLY: [N; 2] = [N::MkCapGrant, N::MkSpawnInstance];

fn required(mk: &str) -> u64 {
    let line = mk.lines().find(|l| l.starts_with("CAPSULE_REQUIRED_CAPS"));
    let word = line.and_then(|l| l.split(":=").nth(1)).map(str::trim).unwrap_or("");
    let hex = word.strip_prefix("0x").unwrap_or("");
    u64::from_str_radix(hex, 16).unwrap_or(0)
}

fn token(mask: u64) -> CapabilityToken {
    CapabilityToken::with_caps(&bits_to_caps(mask))
}

fn opens(mask: u64, n: N) -> bool {
    is_allowed(&token(mask), n, false) || is_allowed(&token(mask), n, true)
}

fn masks() -> [(&'static str, u64); 3] {
    [("acpi", required(ACPI_MK)), ("policy", required(POLICY_MK)), ("power", required(POWER_MK))]
}

#[test]
fn the_three_masks_hold_their_bit_and_no_admin() {
    let admin = Capability::Admin.bit();
    let (acpi, policy, power) = (required(ACPI_MK), required(POLICY_MK), required(POWER_MK));
    assert_eq!(acpi, 0x0A00_0005_8018);
    assert_eq!(policy, 0x400_0000_0059);
    assert_eq!(power, 0x800_0000_0018);
    for (name, mask) in masks() {
        assert_eq!(mask & admin, 0, "{name} holds Admin");
    }
    assert_ne!(acpi & Capability::AcpiOwner.bit(), 0);
    assert_ne!(acpi & Capability::PowerControl.bit(), 0);
    assert_ne!(policy & Capability::PolicyPush.bit(), 0);
    assert_ne!(power & Capability::PowerControl.bit(), 0);
}

#[test]
fn each_capsule_makes_its_calls_with_its_mask() {
    let acpi = required(ACPI_MK);
    for n in ACPI_OWNER.iter().chain(ACPI_OTHER.iter()) {
        assert!(opens(acpi, *n), "acpi: {n:?}");
    }
    for n in POLICY_CALLS {
        assert!(opens(required(POLICY_MK), n), "policy: {n:?}");
    }
    for n in POWER_CALLS {
        assert!(opens(required(POWER_MK), n), "power: {n:?}");
    }
}

#[test]
fn without_its_new_bit_each_capsule_loses_exactly_the_split_calls() {
    let acpi = required(ACPI_MK);
    let no_owner = acpi & !Capability::AcpiOwner.bit();
    for n in ACPI_OWNER {
        assert!(!opens(no_owner, n), "acpi without AcpiOwner: {n:?}");
    }
    assert!(opens(no_owner, N::AdminShutdown));
    assert!(!opens(acpi & !Capability::PowerControl.bit(), N::AdminShutdown));
    assert!(opens(acpi & !Capability::PowerControl.bit(), N::MkAcpiTable));
    for n in ACPI_OTHER.iter().filter(|n| **n != N::AdminShutdown) {
        assert!(opens(no_owner & !Capability::PowerControl.bit(), *n), "acpi: {n:?}");
    }
    let policy = required(POLICY_MK) & !Capability::PolicyPush.bit();
    assert!(!opens(policy, N::AdminPolicyPush));
    for n in POLICY_CALLS.iter().filter(|n| **n != N::AdminPolicyPush) {
        assert!(opens(policy, *n), "policy: {n:?}");
    }
    let power = required(POWER_MK) & !Capability::PowerControl.bit();
    assert!(!opens(power, N::AdminReboot));
    assert!(!opens(power, N::AdminShutdown));
}

#[test]
fn none_of_the_three_grants_revokes_kills_any_or_opens_a_window() {
    for (name, mask) in masks() {
        for n in ADMIN_ONLY {
            assert!(!opens(mask, n), "{name}: {n:?}");
        }
        let admin = Capability::Admin.bit();
        assert!(!grant_rule::may_grant(mask, admin, Capability::IPC.bit()), "{name} grants");
        assert!(!grant_rule::may_revoke(mask, admin), "{name} revokes");
        for (init, parent, protected) in
            [(false, true, false), (true, false, false), (true, true, true)]
        {
            let other = grant_rule::revoke_decision(false, false, init, parent, protected);
            assert!(!other, "{name} revokes from another process");
        }
        let t = token(mask);
        assert!(!t.can_admin() && !t.is_admin(), "{name}");
        assert!(!t.can_control_processes(), "{name} may end any process");
        assert!(!t.can_spawn_window(), "{name} may open a window");
        let kill_any = Capability::ProcessControl.bit() | Capability::Admin.bit();
        assert_eq!(mask & kill_any, 0, "{name}: sys_kill's test");
    }
}

#[test]
fn each_split_bit_opens_its_calls_and_nothing_else() {
    let owner = Capability::AcpiOwner.bit();
    let push = Capability::PolicyPush.bit();
    let power = Capability::PowerControl.bit();
    for e in REGISTRY.iter().flat_map(|s| s.iter()) {
        let n = e.variant;
        let base = Capability::CoreExec.bit();
        let by_bit = |bit: u64| opens(base | bit, n) && !opens(base, n);
        assert_eq!(by_bit(owner), ACPI_OWNER.contains(&n), "AcpiOwner and {n:?}");
        assert_eq!(by_bit(push), n == N::AdminPolicyPush, "PolicyPush and {n:?}");
        let off = matches!(n, N::AdminReboot | N::AdminShutdown);
        assert_eq!(by_bit(power), off, "PowerControl and {n:?}");
    }
}

#[test]
fn admin_opens_none_of_the_split_calls() {
    let admin = Capability::Admin.bit() | Capability::CoreExec.bit();
    let split = [N::AdminReboot, N::AdminShutdown, N::AdminPolicyPush];
    for n in ACPI_OWNER.iter().chain(split.iter()) {
        assert!(!opens(admin, *n), "Admin opens {n:?}");
    }
    assert!(opens(admin, N::MkCapGrant));
}

#[test]
fn inits_ambient_set_holds_no_split_bit_and_no_admin() {
    let split = Capability::AcpiOwner.bit()
        | Capability::PolicyPush.bit()
        | Capability::PowerControl.bit()
        | Capability::Admin.bit();
    assert_eq!(ambient::AMBIENT_CAPS & split, 0);
    let init = ambient::AMBIENT_CAPS;
    for n in ACPI_OWNER.iter().chain([N::AdminReboot, N::AdminShutdown, N::AdminPolicyPush].iter())
    {
        assert!(!opens(init, *n), "init: {n:?}");
    }
}

fn tags_in(dir: &std::path::Path, out: &mut Vec<[u8; 4]>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for p in entries.flatten().map(|e| e.path()) {
        if p.is_dir() {
            tags_in(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            let text = std::fs::read_to_string(&p).unwrap_or_default();
            for (at, _) in text.match_indices("b\"") {
                let b = text.as_bytes();
                let tag = b.get(at + 2..at + 6);
                let closed = b.get(at + 6) == Some(&b'"');
                if let (Some(t), true) = (tag, closed) {
                    if t.iter().all(u8::is_ascii_uppercase) {
                        out.push([t[0], t[1], t[2], t[3]]);
                    }
                }
            }
        }
    }
}

/// A kernel call the ACPI capsule names by its tag is one of the lists above,
/// so a new one cannot arrive behind a mask that does not open it.
#[test]
fn every_kernel_tag_capsule_acpi_names_is_listed_and_opened() {
    let mut tags = Vec::new();
    tags_in(std::path::Path::new(ACPI_SRC), &mut tags);
    let mut kernel = 0;
    for tag in tags {
        let id = u64::from(u32::from_le_bytes(tag));
        let Some(e) = REGISTRY.iter().flat_map(|s| s.iter()).find(|e| e.id == id) else {
            continue;
        };
        kernel += 1;
        let listed = ACPI_OWNER.contains(&e.variant) || ACPI_OTHER.contains(&e.variant);
        assert!(listed, "capsule_acpi names {} ({:?}), not listed", e.name, e.variant);
        assert!(opens(required(ACPI_MK), e.variant), "{:?}", e.variant);
    }
    assert!(kernel >= ACPI_OWNER.len(), "{kernel} kernel tags found");
}

/// Admin stands in for neither DeviceEnum nor SpawnWindow: the device listing,
/// the ACPI device facts and a new window each need their own bit.
#[test]
fn admin_is_no_device_lister_and_no_launcher() {
    let admin = token(Capability::Admin.bit());
    assert!(!admin.can_device_enum() && !admin.can_spawn_window());
    let enumerate = [N::MkDeviceList, N::MkAcpiDeviceName, N::MkUsbPortFacts, N::MkNumaNodeOf];
    for n in enumerate {
        assert!(!opens(Capability::Admin.bit(), n), "Admin opens {n:?}");
        assert!(opens(Capability::DeviceEnum.bit(), n), "DeviceEnum and {n:?}");
    }
    assert!(!opens(Capability::Admin.bit(), N::MkSpawnInstance));
    assert!(opens(Capability::SpawnWindow.bit(), N::MkSpawnInstance));
}
