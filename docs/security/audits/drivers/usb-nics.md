# USB NICs

nonos_usbnet and the five USB Ethernet capsules on it. The five are in no image and have no
spawn entry, so every defect here is latent until they ship. Their held-table findings are in
kernel-grants-and-ipc.md, the xHCI side of their contract in xhci.md. Notation: README.md.

## nonos_usbnet: partial

Shared core: driver.xhci0 client, descriptor walk, root-port scanner and the NNET frame server to net.core. Paths under userland/nonos_usbnet/src/ unless they start with userland/.

- **high** `xhci/bulk.rs:58`: driver.xhci0 does not serve OP_BULK_IN_POLL (0x0014), so every receive fails with E_INVAL. -> D10
- **high** `xhci/control.rs:58`: driver.xhci0 refuses control OUT data stages, so every register write and class command with data fails. -> D10
- **high** `nnet/answer.rs:43`: op 6 is OP_STATS here and OP_RX_BATCH in net.core, so net.core never asks for a received frame. -> D11
- **medium** `say/line.rs:61`: every bind logs the full station MAC, the root port and the VID:PID (live once the mask gains Debug). -> D11 (C4)
- **low**, was medium (net.core bounds each device call at 8 ms, so the stall stays inside this capsule) `run/serve.rs:47`: all device work runs on the one serving thread under 12 s call timeouts, retried forever. -> wave 4
- **medium** `nnet/stats.rs:50`: any empty RX poll resets the failure count, so a dead device is held forever. -> wave 4
- **medium** `xhci/control.rs:40`: short control answers are invisible because driver.xhci0 reports wLength as the actual length, so fields a device never sent read as 0. -> wave 4 (the fix is in xhci, near D10)
- **medium** `userland/usbnet_proofs/src/lib.rs:27`: the xHCI client, run loop and scanner are untested, and MockBus serves ops the real controller does not. -> D10, D11 in part (contract tests)
- low: `xhci/bulk.rs:47` bulk_out slices by the caller's length without checking BULK_MAX.

## capsule_driver_ax88179: partial

ASIX AX88179/AX88178A USB 3.0 Gigabit, a port of Linux ax88179_178a. Paths under userland/capsule_driver_ax88179/src/ unless they start with userland/.

- **high** `ax/station.rs:34`: the factory MAC from AX_NODE_ID goes on the wire and into the log, and a blank chip is refused rather than given a random address. -> D11
- **high** `ax/power.rs:34`: it cannot bind or receive on the shipped stack: the first register write is a control OUT with data, receive needs OP_BULK_IN_POLL, and op 6 collides. -> D10, D11
- **medium** `userland/capsule_driver_ax88179/Capsule.mk:19`: the mask 0x200018 grants InputSource (post and drain of the keystroke ring), not Debug as its comment says; the same in all five. The input verifier rated this low. -> D11
- **medium** `ax/link_poll.rs:49`: each PHY change runs link_reset's control loop on the serving thread, and a device can retrigger it every second. -> wave 4
- **medium** `ax/link_poll.rs:42`: a failed PHY read leaves a stale link-up that never counts toward device loss. -> wave 4
- low: `ax/transfer.rs:52` dropped aggregates and errored frames not counted; `ax/bulkin.rs:36` 4 KiB aggregation caps receive far below gigabit.

## capsule_driver_cdc_ecm: partial

USB CDC-ECM devices (dongles in ECM mode, tethering, QEMU usb-net configuration 1). Paths under userland/capsule_driver_cdc_ecm/src/ unless they start with userland/.

- **high** `ecm/bind.rs:57`: the device-supplied iMACAddress is the station address, with no randomization. -> D11
- **high** `ecm/transfer.rs:39`: it would bind, but can never receive (no OP_BULK_IN_POLL, op 6 collision). -> D10, D11
- **medium** `ecm/link.rs:52`: link is always reported up and NETWORK_CONNECTION is never read (also ncm/link.rs:57, rndis/link.rs:63); the 16-byte speed notification also exceeds xhci's 8-byte interrupt-IN cap. -> wave 4
- **low**, was medium (smoltcp drops the malformed frame on its own length checks) `ecm/frame.rs:42`: an oversized bulk-IN transfer is cut to 1514 bytes and delivered. -> wave 4
- **medium** `userland/capsule_driver_cdc_ecm/Capsule.mk:18`: mask grants InputSource, not Debug. -> D11
- **medium** `userland/cdc_ecm_proofs/src/frame_tests.rs:26`: seven tests and no hostile or random input. -> wave 4
- low: `ecm/transfer.rs:29` send trusts the caller's frame length.

## capsule_driver_cdc_ncm: partial

USB CDC-NCM devices (USB-C adapters and docks in NCM mode, tethering), NTB16. Paths under userland/capsule_driver_cdc_ncm/src/ unless they start with userland/.

- **high** `ncm/start.rs:59`: the device-supplied iMACAddress is the station address. -> D11
- **high** `ncm/setup.rs:49`: a device offering NTBs over 4 KiB fails to bind (SET_NTB_INPUT_SIZE carries OUT data), and receive cannot work. -> D10, D11
- **medium** `ncm/link.rs:57`: link is always reported up. -> wave 4
- **medium** `ncm/setup.rs:53`: when SET_NTB_INPUT_SIZE is refused, an input size below 2048, or 0, is kept, so the device stays bound and delivers nothing. -> wave 4
- **medium** `userland/capsule_driver_cdc_ncm/Capsule.mk:18`: mask grants InputSource, not Debug. -> D11
- low: `ncm/ntb_in.rs:65` no cap on datagrams per NTB; `userland/cdc_ncm_proofs/src/hostile_chain_tests.rs:40` fuzz uses small byte values and short blocks.

## capsule_driver_rndis: partial

RNDIS devices (Android and Windows Mobile tethering, QEMU usb-net configuration 2). Paths under userland/capsule_driver_rndis/src/ unless they start with userland/.

- **medium**, was high (tethering frames end at the phone, and the address is usually gadget-generated) `rndis/configure.rs:36`: the permanent address is the station address. -> D11
- **high** `rndis/control.rs:58`: INITIALIZE goes out as a control OUT with data, so every bind fails on the shipped xhci. -> D10
- **medium** `rndis/link.rs:63`: link is always reported up, and media-connect indications are dropped. -> wave 4
- **medium** `userland/capsule_driver_rndis/Capsule.mk:18`: mask grants InputSource, not Debug. -> D11
- low: `rndis/reply.rs:42` device keepalives never answered; `rndis/init.rs:54` small MaxTransferSize refused instead of a lower MTU; `rndis/transfer.rs:53` dropped messages not counted.

## capsule_driver_rtl8153: partial

Realtek RTL8153/8153B USB 3.0 Gigabit in vendor mode, a port of Linux r8152. Paths under userland/capsule_driver_rtl8153/src/ unless they start with userland/.

- **high** `r8153/nic.rs:31`: the link is read only inside recv, which net.core never calls before the link is up, so the adapter can never be bound, even after every xhci fix. -> D11
- **high** `r8153/up/mac.rs:34`: the factory MAC from PLA_BACKUP is written to PLA_IDR and used on the wire. -> D11
- **high** `r8153/ocp/phy.rs:38`: bring-up dies at the first PHY window write, a control OUT with data. -> D10
- **medium** `r8153/up/power_on.rs:36`: bring-up can block the serving thread about 35 s per attempt, repeated forever. -> wave 4
- **medium** `userland/capsule_driver_rtl8153/Capsule.mk:19`: mask grants InputSource, not Debug. -> D11
- low: `r8153/watch.rs:48` link loss only logged, RX and TX left on; `r8153/transfer.rs:54` frames past 32 per transfer, and 1515 to 1518-byte frames, dropped uncounted.
