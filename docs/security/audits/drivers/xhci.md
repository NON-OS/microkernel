# xHCI

The USB host controller capsule. Its class drivers are in
usb-hid-and-input.md, storage.md (usb_msc) and usb-nics.md. Index and
notation: README.md.

## capsule_driver_xhci: partial

PCI xHCI controllers behind driver.xhci0: root ports only, boot-protocol HID and BOT storage, served to usb_hid0 and usb_msc0. Paths under userland/capsule_driver_xhci/src/ unless they start with userland/.

- **high** `setup/sequence.rs:85`: a bring-up that fails after USBCMD.RUN drops the scratchpads, DCBAA and rings (mk_dma_unmap) before it halts the controller or releases the device, so a running controller can write into freed frames on unconfined hardware, on each of the seven retries. -> D9
- **high** `controller/wait_command_completion.rs:58`: after a command abort, the Command Ring Stopped event is taken as the next command's completion; a device that NAKs the SET_ADDRESS status stage leaves a stale slot that, on controllers handing out the lowest free id, stops all new enumeration until reboot. -> D9
- **medium**, was high (needs a hostile device attached; any later plug, hub rescan or bulk I/O clears it) `controller/poll_interrupt_in/scan.rs:40`: a command completion with no waiter at the event-ring head blocks every interrupt-IN poll, so keyboard and mouse input stop. -> D9
- **medium**, was high (exploiting it needs code execution in a first-party class capsule first) `server/mux.rs:84`: no per-client slot ownership, quota or EP0 filter; either admitted class capsule can read and write the USB disk's sectors, read keyboard reports or disable any slot. The USB NIC verifier found the same at server/dispatch.rs:26. -> D10
- **medium**, was high (a documented, fail-closed gap; the NIC capsules are not built) `server/dispatch.rs:45`: the USB NIC client contract is missing (OP_BULK_IN_POLL, control OUT data, interrupt and bulk endpoints on one slot), so all five USB Ethernet drivers are unusable. -> D10, K7
- **high** (kept as a functional gap) `contexts/input.rs:35`: no hub support; usb_hid already sends OP_ADDRESS_ROUTED and OP_CONFIGURE_HUB and gets E_INVAL, so nothing behind a hub, dock, monitor or laptop-internal hub works. -> D9
- **medium** `server/handlers/recover.rs:55`: per-request DMA buffers and slot contexts are freed even when endpoint recovery or Address Device failed and the controller may still use them. -> wave 4
- **medium** `rings/event/park.rs:59`: the parked-event list holds 8 and evicts silently (an evicted interrupt-IN completion silences that endpoint for good), and Disable Slot does not clear it. -> wave 4
- **medium** `server/handlers/control_transfer/handle.rs:63`: control transfers report wLength as the actual length, and OUT data stages send zeros. -> D10 for OUT data; the actual length: wave 4
- **medium** `server/handlers/interrupt_in.rs:29`: interrupt-IN is capped at 8 bytes, so longer reports babble (NKRO keyboards, report-ID mice, touchpads, 16-byte CDC notifications). -> wave 4
- **medium** `server/runner.rs:42`: the single-threaded server lets one stalling device freeze every USB device for seconds per attempt. -> wave 4
- **medium** `controller/start.rs:19`: no detection of a dead controller (HSE, HCE, all-ones reads) and no re-initialisation. -> wave 4
- **medium** `rings/command/enqueue.rs:23`: the ring-full checks can never fire and neither ring tracks the consumer, so a stuck ring is overwritten after 63 TRBs. -> wave 4
- **medium** `setup/sequence.rs:49`: one global IRQ grant is overwritten by each controller's setup. -> wave 4
- **medium** `server/handlers/address_device.rs:33`: a class capsule that dies or restarts loses its ports until reboot. -> wave 4 (the verifier's D10 fix frees a dead client's slots)
- **medium** `regs/op/portsc_read.rs:19`: PORTSC accesses are not bounded by the mapped window (MaxPorts up to 255). -> wave 4
- **medium** `controller/get_config_descriptor.rs:26`: configuration descriptors are capped at 512 bytes and index 0, control data at 512 bytes. -> wave 4
- **medium** `setup/irq_bind.rs:27`: only MSI-X is bound, so Intel PCH controllers poll; bulk moves one 4 KiB TRB per IPC. -> wave 4
- **medium** `userland/xhci_proofs/src/lib.rs:44`: the IPC handlers, the Mux, the setup failure path and the abort events are untested. -> D9, D10 in part (each reproduces its findings first)
- low: `regs/op/crcr_program.rs:20` 64-bit registers written as one qword; `regs/mmio_read32.rs:17` safe functions dereference raw addresses, 15 unsafe sites without SAFETY comments; `controller/poll_interrupt_in/mod.rs:21` 14 files over 75 lines and code in a mod.rs; `server/handlers/controller_status.rs:59` DCBAA and scratchpad bus addresses sent to class capsules; `contexts/configure_ep.rs:64` interrupt endpoints ignore Mult and MaxBurst; `rings/event/has_event.rs:20` event ordering correct on x86 only.

## Verification

All six highs were re-traced. The verifier of wait_command_completion.rs:58
added the escalation the audit missed (enumeration stops for good on
lowest-free-id controllers). The verifier of contexts/input.rs:35 found the
client side of hub support already shipping in usb_hid, which makes D9's
two new ops the whole fix.
