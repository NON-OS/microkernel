# Wired NICs, 1 of 2: e1000, e1000e, igc

The Intel drivers. Realtek and virtio_net are in wired-nics-2.md. Index and
notation: README.md.

Across all six wired drivers the verifiers found that no driver is ever
respawned, and that the kernel's boot pass already sets Bus Master on every
endpoint (kernel-grants-and-ipc.md, bus/pci/assign/device.rs:76, K6). That
refuted the four "Bus Master before reset" highs as live bugs; they remain a
latent ordering hazard that K6 and D3 close together. The USB NIC verifier
adds that net.core's op-6 probe breaks receive on e1000, rtl8139 and rtl8169
(documented as "the receive fault"); D11 fixes it in net.core.

## capsule_driver_e1000: partial

Intel 8254x (QEMU e1000), polled, one ring pair of 32, in the image. Paths under userland/capsule_driver_e1000/src/.

- **medium**, was high (the first attempt works, since the boot pass set Bus Master; there is no respawn) `setup/sequence.rs:38`: the driver never sets Bus Master itself, so after any failed attempt (whose release clears it) the next attempt reports link up and moves no frames. -> D3
- **low**, was medium (unreachable on conforming parts: LPE is clear and every frame fits one buffer) `queue/rx_consume.rs:53`: the EOP tail of a multi-descriptor frame is delivered as a frame of its own. -> wave 4
- **medium** `discover.rs:51`: BAR0 size is not checked against the highest register written. -> wave 4
- **medium** `constants/pci.rs:19`: fiber/SerDes and IGP-PHY parts are claimed without the link setup they need. -> wave 4
- **medium** `server/handlers/tx_packet.rs:45`: no transmit-hang detection; a stalled ring answers E_AGAIN forever, and with no respawn that lasts until reboot. -> D3
- **medium** `server/runner.rs:66`: only single-frame OP_RX_PACKET is served, no batch receive. -> wave 4
- **medium** `server/handlers/rx_packet.rs:57`: the IPC boundary and the RDT hand-back are untested; the same gap is in all six drivers. -> wave 4
- low: `setup/driver.rs:48` release() unmaps DMA before mk_device_release stops Bus Master (latent, D3); `init/reset.rs:55` Wake-on-LAN left as firmware set it.

## capsule_driver_e1000e: partial, not in the image

Intel 82574/82583 and I217 to I219 on PCH MACs, polled. Paths under userland/capsule_driver_e1000e/src/.

- **refuted**, was high (no respawn exists, Bus Master is already on from the boot pass, and a failed attempt stops the engines first) `setup/sequence.rs:34`: Bus Master is enabled before the part is stopped. -> D3 reorder, K6
- **medium** `init/flush_rx.rs:45`: the I219 RX flush turns the receiver on for a millisecond with the ring firmware left, so a frame can land in reclaimed memory. -> wave 4
- **medium** `init/finish.rs:41`: the station MAC is written to the log on every bring-up. -> wave 4
- **low**, was medium (latent: LPE is clear) `queue/rx_consume.rs:52`: the EOP tail of a split frame is delivered as a frame. -> wave 4
- **medium** `init/post_phy_reset.rs:19`: NVM PHY configuration (LCD extended config, OEM bits) and the runtime link-change workarounds are not done. -> wave 4
- **medium** `server/handlers/tx_packet.rs:43`: no transmit-hang detection, on a part known for TX unit hangs. -> D3
- **medium** `server/handlers/rx_packet.rs:51`: IPC handlers and runner are untested. -> wave 4
- low: `userland/capsule_driver_e1000e/Capsule.mk:37` not in the image; `setup/driver.rs:50` release() order (latent, D3).

## capsule_driver_igc: partial, not in the image

Intel I225/I226 2.5GbE, queue 0 only, polled. Paths under userland/capsule_driver_igc/src/.

- **refuted**, was high (as e1000e; a failed attempt also runs stand_down first) `setup/sequence.rs:35`: Bus Master is enabled before the part is stopped. -> D3 reorder, K6
- **medium** `init/announce.rs:33`: the station MAC is written to the log on every bring-up. -> wave 4
- **medium** `discover/scan.rs:45`: BAR0 size is not checked against the highest register written. -> wave 4
- **low**, was medium (latent: LPE is clear) `queue/rx_wb.rs:40`: the EOP continuation of a split frame is delivered as a frame. -> wave 4
- **medium** `init/phy/power_up.rs:23`: the autonegotiation advertisement is left as the PHY or a previous OS set it. -> wave 4
- **medium** `server/handlers/tx_packet.rs:48`: no transmit-hang detection. -> D3
- **medium** `server/handlers/rx_packet.rs:59`: IPC handlers and runner are untested. -> wave 4
- low: `init/stand_down.rs:38` failure stand-down frees grants before the master stops, the one live instance of the pattern (D3); `init/reset/global.rs:50` Wake-on-LAN never cleared.
