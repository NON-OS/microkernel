## Firmware

Firmware bytes are linked into the capsule with `include_bytes!` from
`nonos-bootloader/firmware/intel/`, so no filesystem authority is required.

On an AX210-family platform the image is chosen as Linux's
`iwl_drv_get_fw_name` names it: by the MAC type and step in CSR_HW_REV and the
RF type in CSR_HW_RF_ID. The PCI id gives the transport values (cfg/ax210.c).

| PCI ids | MAC | RF | Image | In the tree |
|---|---|---|---|---|
| 0x51F0, 0x51F1, 0x54F0, 0x7A70, 0x7AF0, 0x7F70 (SO) | SO 0x37, SO-F 0x43 | GF 0x10D | `iwlwifi-so-a0-gf-a0-86.ucode` | yes |
| same | SO, SO-F | HR 0x10A, 0x10C | `iwlwifi-so-a0-hr-b0-84.ucode` | yes |
| 0x2725 (discrete AX210, TY) | TY 0x42 | GF | `iwlwifi-ty-a0-gf-a0` | no |
| 0x7E40, 0x2729 (Meteor Lake, MA) | MA 0x44, step B | GF | `iwlwifi-ma-b0-gf-a0` | no |

A known adapter whose image is not in the tree is refused by name (step 4,
detail 0x6000i, i the image number), as is anything else (a CDB GF module, JF,
a blank OTP, an MA MAC of another step, an MA HR module, an unknown MAC),
rather than started on a firmware it does not run. Committing the TY or MA
image at API 86 and naming it in `firmware/blob.rs` is all their boot lacks.
The TLV file is parsed with every length checked (`gen3::ucode`); the image
loader, capabilities, API flags and command version table come from it.

The platform NVM (PNVM) is loaded as Linux's `fw/pnvm.c` loads it, when the
image's PNVM file is in the tree (`firmware::gen3_pnvm`): after ALIVE, if the
part reports a SKU id, the section whose SKU and HW type match is laid out
after the image loader as fragmented payloads with a descriptor array (every
bundled image has `FRAGMENTED_PNVM_IMG`), the peripheral scratch's `pnvm_cfg`
names it, and the doorbell rings (`gen3::pnvm`). The AX211's file
(`nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0.pnvm`) is linux-firmware's
at commit bf992df7a95b5e19bb8f8546eb4d7c53ab2598d0, pinned by sha256 in
`scripts/baselines/upstream.sha256`; `iwlwifi_proofs` reads every SKU section
of it against a walk of its TLVs (`gen3_pnvm_real_tests.rs`). An HR radio has
no PNVM, and the TY and MA files are not carried, so those firmwares run on
their defaults, as Linux's does when the file is missing. Every parser of
what the firmware writes, what the air brings and these two files is fuzzed
(`userland/iwlwifi_proofs/src/gen3_fuzz_tests`).

## Authority

The capsule may enumerate PCI devices, claim one supported Intel Wi-Fi
function, map BAR0, bind the device IRQ, allocate broker-owned DMA grants of
at most 64 pages each, and draw kernel randomness. It has no PIO authority,
no filesystem authority, no network-stack authority and no credential
authority.

## Privacy and persistence

The scan is passive: the scan request sets forced-passive and its probe
parameters are all zero (proven in `gen3_layout_tests`), so no probe request,
no address and no network name is ever transmitted. A join finds its network
the same way, by listening, so a network that hides its name is not joined
and no saved network's name leaves the machine before its own access point
is heard. The MAC and link contexts the firmware needs carry the station
address drawn for this boot (a locally administered unicast address from
kernel randomness; the card's factory address is never read). Before each
join they are added again under the network's own address, from the boot's
address key, and every frame the join sends is from it
(`gen3::join::address`). Only when no randomness was given do they carry the
fixed 02:00:00:00:00:01, and then nothing is ever transmitted. Before the
port opens only authentication, association and EAPOL in the clear leave
(`join_tests`). The passphrase is held only while a join runs; the request
buffer that carried it is zeroed after the reply. A receive buffer is
cleared before it is posted again, and a transmit buffer before each frame.
The scan list lives in memory only, is aged (a network unheard for three sweeps
is dropped) and is capped at sixteen networks. The capsule stores no SSIDs,
passphrases, scan history, peer MAC history, DHCP leases, or IP state.
