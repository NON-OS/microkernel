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
| 0x2725 (discrete AX210, TY) | TY 0x42 | GF | `iwlwifi-ty-a0-gf-a0-86.ucode` | yes |
| 0x7E40, 0x2729 (Meteor Lake, MA) | MA 0x44, step B | GF, step A | `iwlwifi-ma-b0-gf-a0-86.ucode` | yes |
| same | MA, step B | GF with CDB (GF4), step A | `iwlwifi-ma-b0-gf4-a0-86.ucode` | yes |
| 0x272B (BE200, Gl) | Gl 0x47, step C | FM 0x112, step C | `iwlwifi-gl-c0-fm-c0-92.ucode` | yes |
| 0xA840, 0x7740, 0x4D40 (BE201, Bz) | Bz 0x46, step B from the CNVi | FM 0x112, step C | `iwlwifi-bz-b0-fm-c0-92.ucode` | yes |

A known adapter whose image is not in the tree is refused by name (step 4,
detail 0x6000i, i the image number), as is anything else (an SO CDB GF4
module, JF, a blank OTP, an MA MAC of another step or an MA RF of another
step, since Linux names the MA image from both, an MA HR module, an unknown
MAC), rather than started on a firmware it does not run.
The TY and MA images are the AX211 image's build (core83-73) for those MACs:
every command and notification version the driver speaks is the AX211
image's (`gen3_ty_ma_tests::firmware`). The MA images add two debug commands
at version 8, the GF4 image the CDB capability, with which a 5 GHz PHY
context names LMAC 1 (`iwl_mvm_get_lmac_id`); nothing else they list is
read. The discrete AX210 runs as Linux v6.17's `iwl_ty_mac_cfg` has it (v6.12
still gave 8086:2725 the integrated SO transport): discrete in
SOC_CONFIGURATION, and its boot LTR (250 us) written to CSR_LTR_LONG_VAL_AD
before UREG_CPU_INIT_RUN, with no image-loader spin (`iwl_pcie_set_ltr`).
The BE200 and BE201 boot in the BZ family's CSR layout (`gen3::csr`, V2);
a Bz MAC's step is read from CNVI_AUX_MISC_CHIP and ORed into hw_rev, as
Linux's `get_crf_id` does. Every image and PNVM file is linux-firmware's at
bf992df7a95b5e19bb8f8546eb4d7c53ab2598d0 (WHENCE: Redistributable,
LICENCE.iwlwifi_firmware, text in `nonos-bootloader/firmware/intel/LICENSE`),
pinned by sha256 in `scripts/baselines/upstream.sha256` (`firmware::blob`).
The TY, MA and BZ-family paths are host-proven only: never booted on those
parts.
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
of it against a walk of its TLVs (`gen3_pnvm_real_tests.rs`); the TY, MA and
MA GF4 files give their own MAC its section (`gen3_ty_ma_tests::boot`). An HR
radio has no PNVM, and runs on its defaults, as Linux's does. Every parser of
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
