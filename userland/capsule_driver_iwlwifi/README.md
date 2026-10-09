# capsule_driver_iwlwifi

## Role

`capsule_driver_iwlwifi` is the Intel Wi-Fi PCIe hardware capsule. It owns the
Intel wireless device claim, BAR0 mapping, interrupt grant, DMA grants,
firmware selection and load, and, on the SO platforms (AX211, and AX201 on an
SO platform), the firmware bring-up, passive scanning, joining WPA2 and WPA3
networks and carrying their data for `driver.iwlwifi0`.

```text
Settings / setup / net_core (nonos_wifi_client)
        |   Wi-Fi control family: status, scan, connect, disconnect, link
        v
driver.iwlwifi0 -- brokered MMIO/IRQ/DMA --> Intel Wi-Fi PCIe device
        |   the MLME, SAE, supplicant, CCMP and station checks of
        |   nonos_wifi_core (shared with driver.rtl8821ce0)
        ^
net_core -- link protocol (NNET): link status, MAC, Ethernet frames
```

The capsule is not an IP stack, socket layer, DHCP client, or network profile
manager. Those layers stay above the hardware driver.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Crypto`, `Driver`, `DeviceEnum`, `Mmio`,
`Irq`, and `Dma`:

```text
CAPSULE_REQUIRED_CAPS = 0xF8038
```

The driver reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`, `MkIrqBind`, and `MkDmaMap`. The kernel validates the signed
manifest, routes IPC, brokers grants, and revokes every grant on capsule exit.

Only `net.core` and the Wi-Fi panels (`app.settings` in any of its three
windows, and `app.setup_wizard`) may send to `driver.iwlwifi0`: the kernel
holds the endpoint to them (`src/services/registry/held.rs`), by name and by
pid. That covers all three request families, the older NIWF one included.

Two limits of this grant shape the driver:

- The broker maps at most 64 pages (256 KiB) per DMA grant for a network-class
  device (`dma_page_limit_for_class`). The legacy staging grant is 64 pages,
  and the gen3 path spreads its memory over several grants of at most that
  size (control region, receive buffers, and the firmware sections planned
  first-fit by `gen3::plan`). The first join maps one more, the 160 KiB
  transmit region (`gen3::txq`: the management and data queues' rings, byte
  count tables and frame buffers), kept for the capsule's life once the
  firmware holds its addresses.
- `Crypto` is there for `CryptoRandom` alone: the station address is drawn
  fresh every boot (`server/radio/address.rs`, through `nonos_mac`), as are a
  join's handshake nonce and SAE secrets. If the kernel gives no randomness
  the interface takes the fixed address 02:00:00:00:00:01 and only the
  passive scan runs.
- The mask does not hold `Debug` (no console). Without it the `[iwlwifi]`
  console lines are refused by the kernel; the status reply carries the same
  facts.

## Interface contract

Three request families arrive on the one service inbox: the Wi-Fi control
family, net_core's link protocol, and this driver's own.

The Wi-Fi control family (`nonos_wifi_client`, tag `0x57494649`):

| Operation | Input | Output |
|---|---|---|
| status (4) | none | stage byte, then step, detail, CSR_HW_REV, CSR_HW_RF_ID, sweeps, stalls, frames, beacons |
| scan (3) | none, radio up | sweeps, frames, beacons, then the network list |
| connect (1) | `[ssid_len][ssid][pass_len][pass][flags]` | status code, sent, received, data, EAPOL, probe, deauthentications, to us, state, AKM, AP status (`server/join_wire.rs`) |
| disconnect (2) | none | 0 |
| link (5) | none | `[associated][bssid 6][ssid_len][ssid][akm]` |
| connect, disconnect, link when the radio cannot join | any | -38 |
| scan with the radio down, any other op | any | -38 |

The radio can join when it is up, a station address was drawn, and the
firmware runs every join command at the layout this driver encodes
(`gen3::station::ids`). The connect codes are the RTL8821CE's (-1 to -12,
named in `nonos_wifi_client::driver::join_text`); -1 also covers a join
command the firmware failed.

net_core's link protocol (`nonos_wifi_core::netif`, tag "NNET"): link status,
MAC address, transmit and receive one Ethernet frame. Until a port is open the
link reads down and holds no address.

The driver's own `NIWF` protocol (version 1):

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_DEVICE_INFO` | none | PCI id, hw revision, family |
| `OP_FIRMWARE_INFO` | none | selected firmware name and size |
| `OP_RF_STATE` | none | RF-kill / init flags |
| `OP_DMA_STATE` | none | brokered DMA staging grant metadata |
| `OP_FIRMWARE_STAGE` | none | staged firmware section counts and bytes |
| `OP_ALIVE_WAIT` | none | alive notification status and interrupt word |

It also routes `OP_FIRMWARE_LOAD`, `OP_RX_POLL`, and the older join helpers
from before the shared core ran the join (`OP_MGMT_BUILD`, `OP_BEACON_PARSE`,
`OP_HCMD_ISSUE`, `OP_WPA_PTK`, `OP_EAPOL_VERIFY`, `OP_CCMP`,
`OP_KEY_UNWRAP`, `OP_EAPOL_BUILD`, `OP_SUPPLICANT_START`,
`OP_SUPPLICANT_STEP`, `OP_CONNECT_START`, `OP_CONNECT_MGMT`,
`OP_CONNECT_EAPOL`, `OP_WIFI_TX`, `OP_WIFI_RX`; `src/server/dispatch.rs`).
Nothing in the tree speaks NIWF to them.

Unknown operations reply `E_BAD_OP`. Non-empty bodies on fixed-width requests
reply `E_INVAL`. Once the gen3 radio owns the card, the ops that write it
(`OP_FIRMWARE_LOAD`, `OP_ALIVE_WAIT`, `OP_HCMD_ISSUE`) reply `E_BUSY` (-16).

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
names it, and the doorbell rings (`gen3::pnvm`). No PNVM file is committed
yet, so the doorbell rings with none and the firmware runs on its defaults,
as Linux's does when the file is missing.

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
kernel randomness; the card's factory address is never read); every frame a
join sends is from it. Only when no randomness was given do they carry the
fixed 02:00:00:00:00:01, and then nothing is ever transmitted. Before the
port opens only authentication, association and EAPOL in the clear leave
(`join_tests`). The passphrase is held only while a join runs; the request
buffer that carried it is zeroed after the reply.
The scan list lives in memory only, is aged (a network unheard for three sweeps
is dropped) and is capped at sixteen networks. The capsule stores no SSIDs,
passphrases, scan history, peer MAC history, DHCP leases, or IP state.

## Runtime lifecycle

Startup (`setup`) discovers a supported Intel Wi-Fi PCI function, claims it,
maps BAR0, binds INTx if the line is routed, else one MSI-X vector, else runs
polled, maps the 64-page staging grant, requests MAC access and waits for the
clock, and reads the hardware revision.

Then, before the first request, the gen3 radio comes up once
(`server/radio`): check the register window, take the NIC (prepare, software
reset, APM init), read CSR_HW_REV and CSR_HW_RF_ID, select the firmware, map
the grants, lay out the context info, peripheral scratch, rings, image loader
and firmware sections, kick the self-load, start the CPU (UREG_CPU_INIT_RUN
through the UMAC window), wait up to 2 s for ALIVE (status 0xCAFE), ring the
PNVM doorbell when the part reports a SKU id, then run INIT_EXTENDED_CFG,
NVM_ACCESS_COMPLETE, INIT_COMPLETE, NVM_GET_INFO, antennas, BT, SoC latency,
power, MCC "ZZ", scan config, and the MLD MAC and link contexts, each bounded by
a 2 s reply wait.

The serving loop then waits at most 50 ms for a request and, between
requests, pumps a background passive UMAC scan over the NVM's channels: a
sweep is started, every received beacon or probe response goes through
`nonos_wifi_core` into the shared scan list, the completion carrying this
scan's uid ends it, and the next sweep starts after a 3 s rest. A sweep that
runs past its budget (150 ms per channel plus 2 s) is given up and counted.
Teardown is handled by process exit and broker revocation of the device, MMIO,
IRQ, and DMA grants.

A connect (`server/radio/join.rs`, `firmware/gen3/join`) runs inside the
request, as on the RTL8821CE, in this order:

1. Leave any current network. Map the transmit region on first use.
2. Hunt the network: stop a background sweep in flight (SCAN_ABORT_UMAC),
   run a passive sweep until a beacon or probe response names the network,
   then stop it. Not heard in a whole sweep: code -2.
3. Draw the handshake nonce and SAE secrets. The MLME reads the beacon and
   refuses here, before any firmware context or frame, an open, TKIP,
   Enterprise or HT-only network, a passphrase that cannot be one, and a
   network saved as WPA3 that now offers only WPA2.
4. Put the firmware contexts up in Linux v6.12's MLD order: PHY context on
   the channel and its RLC chains, link pointed at the PHY then activated with
   the ACK rates, the access point as station 0 (`mfp` 1 until authorized),
   its management (TID 15) and data (TID 0) queues, session protection
   (900 ms), waited for until the firmware is on the channel.
5. Exchange frames through the MLME: Open System or SAE (hash-to-element or
   hunting and pecking) authentication, association, then the four-way
   handshake, an unanswered frame resent at each 300 ms of silence up to six
   times within twenty-five waits (7.5 s, so the hunt and the exchange fit the
   client's 20 s connect timeout). On association the MAC context, station entry
   and link take the AID and ERP state; a session that ends early is asked
   for again.
6. Install the pairwise key (index 0, with MFP when negotiated) and the group
   key (its own index), authorize the station entry, cancel the session, and
   open the port.

Any end takes down what was put up, keys first, flushing the queues before
removing them. Management and handshake frames go at the lowest basic rate
and data at the highest basic rate, each with its rate in the command
(`IWL_TX_FLAGS_CMD_RATE`); no rate scaling table is configured. The station
protects every frame it sends (`Ccmp::SoftwareTxHwRx`) and the firmware sends
it as it is (`IWL_TX_FLAGS_ENCRYPT_DIS`); received frames the firmware
decrypted from its key table, and any it did not, both pass the station's
checks (from the BSS, protected, not a replay) before net_core sees them.

While the port is open the background scan rests and each pass of the loop
services the link: the access point's group rekeys are answered and the new
key installed (the old index removed), and a deauthentication or
disassociation from the access point (unprotected only without management
frame protection) closes the port and takes the contexts down. A disconnect
sends a deauthentication (protected under MFP) before the same teardown.
Beacon loss is not acted on: a link whose access point went away silently
stays up until the next disconnect.

Interrupts: the gen3 path polls. It unmasks the ALIVE cause for the boot (as
Linux does) and, once ALIVE is in, masks every device interrupt (CSR_INT_MASK
0, both MSI-X mask registers all ones); the cause registers still latch and the
error checks read them. The interrupt the setup bound is not serviced. To
service interrupts the driver would need its own MSI-X configuration
(`iwl_pcie_conf_msix_hw`: the IVAR tables routing each cause to a vector);
that is not done. The 7265 and 8265 families are MSI-only in Linux (no MSI-X),
so on a machine where their INTx line is not routed they end up polled.

## Failure model

Every setup phase rolls back prior broker grants on failure. With no
supported adapter in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. An adapter that is present but
fails setup, including an APM clock that never comes up, gives back every
grant and is retried on the shared bounded schedule (`nonos_libc::bring_up`:
seven tries, sleeping between them); running out exits `EXIT_GAVE_UP` (6).

A gen3 bring-up that fails keeps serving and reports where it stopped (stage
code for the client, then step and detail word):

| Step | Failure | Stage |
|---|---|---|
| 1 | register window smaller than 0x2810 | DeadMmio (3) |
| 2 | CSR_HW_REV reads all ones | DeadMmio (3) |
| 3 | NIC not ready (detail 1), clock not ready (2), no NIC access (3) | PowerFailed (2) |
| 4 | no bundled firmware: not an AX210-family platform (0x1xxxx, the PCI id), MAC (0x2xxxx), RF (0x3xxxx), CDB (0x40000), MA step (0x5000s), image not in the tree (0x6000i) | NoAirPath (8) |
| 5 | bundled image did not parse or plan | FirmwareFailed (4) |
| 6 | the broker refused a DMA grant | NoDma (5) |
| 7 | boot: start (0x1x), layout (0x2x), no ALIVE cause (0x30), no ALIVE notification (0x4x), status not 0xCAFE (0x5ssss), PNVM (0x6x) | FirmwareFailed (4), or PowerFailed (2) for start errors |
| 8 | up: unanswered command (0x01ggccww: group, command, wait), no INIT_COMPLETE (0x020000ww), bad NVM, bad MCC, unsupported API | FirmwareFailed (4) |
| 9 | the firmware raised its error cause while running | FirmwareFailed (4) |

Every wait is bounded by the uptime clock and by a count of sleeps, so a
stopped clock cannot hang it. A failure after the kick reads the secure-boot
CPU status, stops bus mastering and resets the device, and keeps the grants
mapped so nothing the device may still write to is handed back. A firmware
that fails while running is stopped the same way and the radio stays down;
there is no restart.

## Current implemented surface

Implemented and proven on the host against a modeled device
(`iwlwifi_proofs`):

- Joining WPA2-PSK and WPA3-SAE networks on AX211 (the SO family) and
  carrying data both ways through net_core's link protocol, against the
  modeled firmware and a scripted access point built from the shared core
  (`join_tests`, `serve_tests`). Every firmware command's bytes are pinned to
  the Linux v6.12 structure and API version they encode (`station_cmd_tests`,
  `tx_path_tests`), and the bundled firmware files are held to those
  versions. The layouts were then checked against Linux v6.12's own fw/api
  headers compiled with gcc: all 133 sizes and field offsets match
  (rerun with its `run.sh`).
- Intel Wi-Fi PCI discovery for 7265, 8265, 9260, AX200, AX210 and the SO
  platforms, whatever the INTx line says; INTx, then MSI-X, then polled.
- Brokered claim, MMIO map, IRQ bind and DMA grants within the per-grant cap.
- Gen3 firmware selection, TLV parse, DMA plan and layout, self-load kick, CPU
  start, ALIVE, PNVM doorbell, the post-ALIVE command sequence, NVM channels,
  the MCC reply, and passive UMAC scanning with received management frames
  parsed (CRC and overrun checked, pad and MIC/CRC tail removed, every length
  bounded) into the shared scan list.
- The command queue (TFH TFDs, first TB, doorbell) and the receive queue
  (free and used descriptors, closed index, restock), every index from the
  device masked to its ring and every buffer id checked.
- The Wi-Fi control family status, scan, connect, disconnect and link
  replies, and net_core's link protocol.

Framed but not run on the air:

- The legacy FH load path for 7265, 8265, 9260 and AX200 (staging, section
  upload, ALIVE wait) exists behind the NIWF ops but is not driven at startup,
  and AX200 or older parts report NoAirPath.

## Wire format

Control family requests: `[magic u32][op u16][request id u32][body]`, replies
echo the header. Status reply after the header: `[stage u8][step u8]` then
seven little-endian u32 words: detail, CSR_HW_REV, CSR_HW_RF_ID, sweeps
completed, sweeps stalled, frames received, beacons parsed. Scan reply after
the header: sweeps, frames, beacons (u32 each), then `[count]` and per network
`[signal 0..100][flags][ssid_len][ssid]` (flags: bit 0 secured, bit 1 WPA2,
bit 2 WPA3). The largest reply is 583 bytes. The connect body and the
connect and link replies are laid out in the Interface contract above and in
`server/join_wire.rs`; the client reads them with `nonos_wifi_client`'s own
parsers (proven in `serve_tests`).

net_core link protocol requests use its 20-byte envelope (`NNET`); a frame is
at most 1514 bytes.

NIWF requests use the 20-byte driver envelope; replies begin with a 4-byte
signed status word. All multi-byte integers are little-endian.

## State ownership

`driver.iwlwifi0` owns only hardware-facing Wi-Fi state: PCI identity, broker
grant ids, BAR mapping, IRQ binding, DMA grants, hardware and RF ids, the
firmware's queues and contexts, the current scan list, and while joined the
link's keys, counters and the network's name. The client and panels own scan
presentation, saved networks and join policy.

## Operating rules

- Do not place DHCP, DNS, IP, or socket policy in this capsule.
- Do not import kernel driver or memory internals.
- Do not use inline PIO or architecture assembly.
- Do not persist Wi-Fi profiles or scan history.
- Every DMA access goes through `Region` with its bounds checked; every wait is
  bounded.
- Firmware upload must go through bounded DMA and explicit completion.

## Release target

The hardware chain is:

```text
driver.iwlwifi0 -> nonos_wifi_core -> net.core -> net.ip -> apps
```

## Release evidence

- `iwlwifi_proofs`: 197 tests run by `cargo test` (`src/link_tests.rs` is not
  compiled), host only. The gen3 boot runs end to end
  against `gen3_model` with the bundled so-a0-gf-a0-86 image, and so do WPA2
  and WPA3 joins with data through the transmit and receive rings.
- `nonos_wifi_core_proofs`: 70 tests (the shared scan list among them).
- The iwlwifi layout check: every join command's size
  and field offsets against Linux v6.12's headers, compiled with gcc, `bad=0`.
- No hardware log for this driver is committed.

## Release checklist

- Capsule builds with zero warnings for `x86_64-nonos-user`.
- Static gates confirm brokered MMIO/IRQ/DMA authority and endpoint ownership.
- Kernel profile `microkernel-driver-iwlwifi` resolves signed artifacts.
- Firmware selection picks the expected `.ucode` for the detected MAC and RF.
- Firmware upload and ALIVE pass on supported hardware.
- Passive scan returns bounded results without persisting SSID history.
- A WPA2 and a WPA3 join reach a DHCP lease on supported hardware.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the
parsers, the sequence against a model, and the bounded waits; these do not.
The `[iwlwifi]` lines below print only on a kernel that grants this capsule
`Debug`; without it, read the same facts from the status reply (stage, step,
detail, CSR_HW_REV, CSR_HW_RF_ID, counters) and the Settings Wi-Fi panel.

- Absence and retry: on a machine without the device, no claim and exit
  `EXIT_ABSENT` (2), which the kernel names as `[EXIT] <service> status 2: no
  device present, not started` (`src/process/exit/end_note.rs`). With the
  device present but failing setup, at most seven claim and release rounds,
  then `EXIT_GAVE_UP` (6) and `[EXIT] <service> status 6: device present,
  bring-up failed and was given up`. The capsule holds no core while it waits.
- Setup on an SO laptop: the MAC clock comes ready in setup's legacy APM poll
  (it runs before the gen3 path takes the NIC; if it times out there, setup
  gives up and the gen3 path never runs). The broker log shows the 64-page
  staging grant accepted.
- Identity: `[iwlwifi] pci 0x000051f0 hw_rev 0x00000370 rf_id 0x0010d000` (or
  your platform's values; MAC type 0x37 or 0x43, RF type 0x10D, 0x10C or
  0x10A), then `[iwlwifi] firmware iwlwifi-so-a0-gf-a0-86.ucode` (or hr-b0-84).
- DMA: about ten further grants in the broker log, none refused (else
  `[iwlwifi] no DMA region of N bytes` and stage NoDma).
- ALIVE: `[iwlwifi] firmware alive: status 0x0000cafe umac M.N sku 0x...`.
  If instead `radio not up: the firmware never raised ALIVE (step 7, detail
  0x00000030)`, the line before it gives the secure-boot CPU status, which
  says whether the ROM accepted the context info and the image loader.
- PNVM: with a non-zero SKU the boot waits up to 250 ms for PNVM_INIT_COMPLETE;
  detail 0x61 means it never came. With a PNVM file in the tree, the line
  `platform NVM section ... given` or `no platform NVM section for this SKU`
  says which happened.
- Up: `[iwlwifi] up: 51 channels, tx ant 3, regulatory 0x....`, then
  `[iwlwifi] radio up; passive scanning`. A failure names the group and
  command that went unanswered in its detail word.
- RF-kill: with the switch off, `the hardware RF-kill switch has the radio
  off`; scans are then not expected to complete.
- Scan: `[iwlwifi] first scan complete: N networks from M beacons` within
  about ten seconds; the Settings panel status reads Ready and its scan counters
  climb (sweeps, frames, beacons). Check against the APs in range: a WPA2-only
  AP shows WPA2, a WPA3-only (SAE, H2E or hunting-and-pecking) AP shows WPA3, a
  transition-mode AP shows both, and signals look plausible (-50 dBm or
  stronger reads 100).
- Nothing on the air: a monitor-mode capture next to the laptop sees no probe
  request from it during scans.
- Stalls and failures: `scan gave no completion within its budget` should not
  appear; `radio down: the firmware raised its error cause while running`
  means the firmware asserted.
- Joining available: neither `no randomness for a station address: scanning
  only` nor `joining off: the firmware runs another layout of command ...`
  appears. Before any join net_core logs `[NET-CORE] link probe
  driver.iwlwifi0 down` (no longer `no-answer`).
- WPA2-PSK, from the Settings panel or net_core's autojoin: within a few
  seconds `[iwlwifi] join: port open; sent 4 recv N eapol 2 state 4 refused
  0` (N at least 4), the panel reads Joined, the link op reports associated
  with AKM 2, and net_core binds `driver.iwlwifi0` and takes a DHCP lease;
  ping and name lookups work.
- WPA3-SAE, once with an H2E access point and once with hunting and pecking:
  `[iwlwifi] join: port open; sent 5 ...` (commit, confirm, association,
  messages 2 and 4), AKM 8, then a lease. A transition-mode access point is
  joined with SAE.
- A monitor-mode capture of the joins: every frame from a locally
  administered address that changes each boot; no probe request; Open System
  (algorithm 0) or SAE (algorithm 3) authentication, an association request
  with the RSNE (AKM 2 or 8, MFPC for SAE), EAPOL messages 2 and 4 in the
  clear, then only CCMP-protected data. The access point's log shows the
  station authorized.
- Refusals: a wrong WPA2 passphrase ends with code -6 (state 3, the access
  point's deauthentication with reason 15 or a timeout); a wrong WPA3 password
  with -10; a network saved as WPA3 that now offers only WPA2 with -7 and
  nothing on the air; a hidden network with -2.
- Group rekey: with the access point rekeying the group key every minute, the
  link stays up across several rekeys and the disconnect line counts them.
- Leaving: a disconnect prints `[iwlwifi] left the network: tx ... rekeys N
  dropped 0 stray 0 failed F`, the access point logs a deauthentication with
  reason 3 (protected under WPA3), and the scan counters climb again. Kicking
  the station from the access point prints `link: the access point ended the
  association, reason R`.
- A join that fails in the firmware prints `join: a firmware context did not
  go up: ...`, `join: command 0x... failed` or `join: the firmware raised its
  error cause`; any of these means a command layout the firmware read
  differently, and the line names the command.

## Explicit non-goals today

This slice does not implement open networks, hidden networks, HT, VHT or HE
rates, rate scaling (data goes at the highest basic rate), QoS (WMM),
aggregation, power save, beacon-loss detection, active scanning, 6 GHz
channels, regulatory updates beyond the world domain, firmware restart,
interrupt-driven operation, monitor mode, AP mode, or roaming. The 7265, 8265,
9260/9560 and AX200 parts, and AX201 outside the SO platforms, do not get a
radio from this driver yet; the discrete AX210 and Meteor Lake lack only their
firmware files.

## Verification

- Capsule: `(cd userland/capsule_driver_iwlwifi && cargo build --release
  --target ../x86_64-nonos-user.json -Zbuild-std=core,alloc
  -Zbuild-std-features=compiler-builtins-mem)`
- Proofs: `(cd userland/iwlwifi_proofs && cargo test --release)`,
  `(cd userland/nonos_wifi_core_proofs && cargo test --release)` and
  `(cd userland/wifi_panel_proofs && cargo test --release)`
- Build: `make -B nonos-mk-driver-iwlwifi`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-iwlwifi`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [network stack](../../docs/handbook/network/stack.md).
