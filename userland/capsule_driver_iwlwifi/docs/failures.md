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

The 7265, 8265, 9260 and AX200 parts are found and refused by the gen3 radio
before it maps any DMA; their firmware and the legacy load path are deleted.
