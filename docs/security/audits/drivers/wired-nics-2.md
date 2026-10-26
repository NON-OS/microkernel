# Wired NICs, 2 of 2: rtl8139, rtl8169, virtio_net

The Realtek drivers and virtio_net. The Intel drivers, and the notes that
apply to all six, are in wired-nics-1.md. Index and notation: README.md.

## capsule_driver_rtl8139: partial

Realtek RTL8139 10/100 over an I/O-port BAR, flat 32 KiB RX ring, polled, in the image. Paths under userland/capsule_driver_rtl8139/src/.

- **refuted**, was high (no respawn, Bus Master already on, and a failed attempt practically never leaves the receiver on) `setup/sequence.rs:28`: Bus Master is set before the reset. -> D3 reorder, K6
- **medium** `setup/dma.rs:52`: DMA is mapped without DMA32 and checked only at the start address, so a ring can straddle 4 GiB and wrap. -> wave 4
- **medium** `init/reset.rs:22`: the reset wait is a loop of up to 1,000,000 PIO reads, not a deadline. -> wave 4
- **medium** `tx/reclaim.rs:50`: no transmit-hang detection; four stuck slots answer E_AGAIN forever. -> D3
- **medium** `server/handlers/rx_packet.rs:27`: the TX path, runner and handlers are untested. -> wave 4
- low: `tx/send.rs:39` only compiler fences order DMA buffers; `rx/ring_u8.rs:30` unsafe blocks without SAFETY comments; `init/rx_setup.rs:27` multicast accepted with an unprogrammed hash filter, wake bits left; `setup/driver.rs:41` release() order, and a PIO failure after RE|TE would free the live ring (D3).

## capsule_driver_rtl8169: partial

RTL8169/8168/8111/810x and RTL8125, identified by XID, polled, in the image. Paths under userland/capsule_driver_rtl8169/src/.

- **refuted**, was high (no respawn, Bus Master already on, and a failed attempt never leaves RE set) `setup/sequence.rs:32`: Bus Master is enabled before quiesce and reset. -> D3 reorder, K6
- **medium** `rx/recv_one.rs:42`: the RX error-summary bit (RxRES) is never checked, so some errored frames are handed up. -> wave 4
- **medium** `rx/rearm.rs:33`: descriptor publication relies on compiler fences and one aggregate store; correct on x86 only. -> wave 4
- **medium** `hw/init_8168g.rs:24`: no PHY configuration, PHY firmware or ePHY tables, and DASH is left running on 8168ep/8117. -> wave 4
- **medium** `server/handlers/tx_packet.rs:35`: no transmit-hang detection. -> D3
- **medium** `server/handlers/rx_packet.rs:27`: runner and handlers are untested. -> wave 4
- low: `init/rx_setup.rs:55` all multicast accepted, wake bits left; `rx/recv_one.rs:37` unsafe blocks without SAFETY comments; `setup/driver.rs:42` release() order (latent, D3).

## capsule_driver_virtio_net: partial

virtio-net over the legacy or modern transport, 64 RX and 8 TX buffers of 2 KiB, polled, in the image. Paths under userland/capsule_driver_virtio_net/src/ unless they start with userland/. Under a vIOMMU its legacy transport has the virtio_gpu and virtio_rng defect (media.md); K5 refuses that at claim.

- **high**, was critical (the trigger is hypervisor configuration, not an attacker; default QEMU's 256 is unaffected; critical if bhyve or tuned libvirt is a target) `init/program_queue.rs:32`: the legacy path accepts any QueueNumMax, so with 512 or 1024 entries the device's avail and used rings lie past the 12 KiB grant and it reads and writes neighbouring frames. -> D1
- **high** `server/handlers/rx_packet.rs:43`: a frame of 1525 to 2038 bytes panics OP_RX_PACKET and kills the capsule; drivers are never respawned, so networking is down until reboot. -> D1
- **medium** `server/handlers/rx_fill.rs:40`: an oversize frame pushes a batch past net.core's buffer and the whole batch is lost. -> D1 (the same cap in take_one)
- **medium** `tx.rs:60`: TX waits for each frame with a 200,000-yield spin, one frame in flight. -> wave 4
- **medium** `rx.rs:83`: take_one returns a 'static slice over memory the device can write; unsafe is undocumented. -> wave 4
- **medium** `discover/constants.rs:17`: discovery scans only 32 device records. -> wave 4
- **medium** `setup/modern/run.rs:38`: the modern path sets Bus Master before resetting the device (not verified; its wired twins were refuted). -> wave 4, K6
- **medium** `userland/virtio_net_proofs/src/net_tests.rs:182`: tests pin take_one returning 2038-byte frames but never run the handlers. -> D1 in part
- low: `setup/sequence.rs:55` four files over 75 lines; `server/handlers/tx_packet.rs:27` bodies under 14 bytes accepted; `userland/nonos_virtio/src/common/status.rs:37` modern reset bounded by 1000 reads; `main.rs:64` exit path unmaps DMA while the device is live (unreachable).
