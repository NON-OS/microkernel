# USB HID and input

The input drivers: usb_hid, i2c_hid, i2c_pci and ps2_input. The kernel's
input capabilities (who may post to and drain the keystroke ring) are in
kernel-grants-and-ipc.md. Index and notation: README.md.

## capsule_driver_usb_hid: partial

IPC-only USB HID class driver above driver.xhci0: boot keyboards and mice, hubs, and a QEMU-tablet decode for any other HID interface. Paths under userland/capsule_driver_usb_hid/src/ unless they start with src/ or userland/.

- **medium**, was high (no reader exists: the endpoint is kernel-only and the kernel never calls the op) `hid/keyboard/push_key.rs:53`: the first 64 key events after boot, with their ASCII bytes, stay in a queue nothing drains. -> D4
- **medium**, was high (no privilege gain: a hostile device can declare a boot mouse anyway) `descriptors/binding.rs:44`: every non-boot HID interface is decoded with the QEMU tablet's fixed layout, so a gamepad or UPS holds buttons down and moves the pointer. -> D4
- **medium** `orchestrator/poll/feed_report.rs:33`: one keyboard and one mouse state are shared by every bound device, so a second keyboard's idle report releases the first one's keys. -> D4
- **medium** `orchestrator/enumerate/forget.rs:39`: unplugging a keyboard with a key held repeats that key for up to 30 s and never posts KEY_UP. -> D4
- **medium** `server/dispatch.rs:25`: keystroke-injection (OP_FEED_*) and keystroke-readback (OP_POLL_KEYS) ops stay in the production build. -> D4
- **medium** `orchestrator/enumerate/attach_child.rs:41`: a flapping or hostile hub stalls all USB input for seconds on every rescan. -> wave 4
- **medium** `descriptors/parse.rs:60`: media and consumer keys and every report-protocol feature are dropped, for want of a report-descriptor parser. -> D4 in part (the parser; binding consumer pages: wave 4)
- **medium** `orchestrator/binding.rs:24`: no keyboard LED output reports, so Caps Lock state is invisible at a password prompt. -> wave 4
- **medium** `server/pump_once.rs:25`: every endpoint is polled over IPC about 1000 times a second, and there is no suspend. -> wave 4
- InputSource also lets this capsule drain every keystroke: see kernel-grants-and-ipc.md, authority_broker.rs:72 (high, was medium, K8).
- low: `orchestrator/poll/drain_endpoint.rs:32` drain loop has no per-pass cap; `xhci/call.rs:53` xHCI replies not matched to their request; `src/userspace/capsule_driver_usb_hid/client/probe.rs:63` kernel mirror client out of sync; `userland/capsule_driver_usb_hid/README.md:244` README contradicts the code; `descriptors/parse.rs:24` file over 75 lines.

## capsule_driver_i2c_hid: partial

HID-over-I2C touchpad class driver over driver.i2c_pci0, with a gesture engine. Paths under userland/capsule_driver_i2c_hid/src/ unless they start with userland/.

- **medium** `input/poll/cycle.rs:56`: a device that is neither touch nor mouse (an I2C keyboard, a sensor hub) is decoded as a mouse by a byte heuristic, so Ctrl clicks and key codes move the pointer. -> wave 4
- **medium** `hid/report_desc/decode/decode_touch.rs:40`: unchecked u32 arithmetic on descriptor-derived bit offsets; wraps in release, panics with overflow checks on. -> wave 4
- **medium** `hid/probe/scan.rs:30`: with no pad found, it writes register pointers to eight bus addresses every 0.5 s forever, which rewrites GPIO expanders at 0x20 and 0x24. -> wave 4 (D2's address bound in i2c_pci would refuse it)
- **medium** `hid/report_desc/parse/walk.rs:48`: the report-descriptor walker mis-lays out legal descriptors (offsets reset per Report ID, no Push/Pop, usage ranges ignored). -> wave 4
- **medium** `server/runner/run.rs:26`: where no GPIO doorbell is mapped, the input register is read every 2 ms forever. -> wave 4
- **medium** `hid/power/wake.rs:44`: no suspend or resume, and a device reset is not detected. -> wave 4
- **medium** `userland/input_proofs/src/hid_touchpad_tests.rs:111`: the report-descriptor parser has no hostile-input or fuzz tests. -> wave 4
- low: `input/poll/read_frame.rs:58` diagnostic sentinels and hex dumps in the production path; `input/gesture/on_touch.rs:26` files over 75 lines; `i2c_client/acpi_hid.rs:67` peer-supplied slave address truncated without validation; `userland/capsule_driver_i2c_hid/README.md:78` README says the doorbell op is not served.

## capsule_driver_i2c_pci: near

DesignWare I2C master on Intel LPSS (and ACPI platform) controllers, held to driver.i2c_hid0. Paths under userland/capsule_driver_i2c_pci/src/.

- **low**, was medium (only a compromised i2c_hid, its sole client, could send the op) `server/handlers/controller.rs:13`: OP_CONTROLLER_INFO always panics (an 8-byte copy into 4 bytes). -> D2
- **medium** `setup/pci.rs:31`: Bus Master is turned on and the LPSS IDMA released on a driver that does no DMA; unconfined, a compromised i2c_pci could DMA anywhere. -> D2, K6 (Bus Master refused without Dma)
- **medium** `server/handlers/transfer.rs:26`: OP_TRANSFER reaches every address on the bus (PMIC, PD controller, EEPROM, I2C TPM), not only the bound touchpad. -> D2
- **medium** `transaction/engine/run.rs:73`: transfers busy-spin a core for their whole duration. -> wave 4
- **medium** `transaction/control/wait_idle.rs:29`: no recovery from a stuck bus; the touchpad stays dead until reboot. -> D2
- **medium** `setup/sequence/run.rs:149`: when firmware declares no device, a blind scan writes to undeclared addresses on every controller. -> D2
- The Irq capability in `userland/capsule_driver_i2c_pci/Capsule.mk:16` lets this bus driver post keystrokes: see kernel-grants-and-ipc.md, authority_broker.rs:61 (medium, K8).
- low: `regs.rs:12` MMIO accessors without SAFETY comments or bounds; `setup/sequence/run.rs:35` files over 75 lines, stale README, truncated comment.

## capsule_driver_ps2_input: near

i8042 keyboard and mouse over brokered PIO, interrupt-driven. Paths under userland/capsule_driver_ps2_input/src/.

- **low**, was high (no reader: the kernel client has no caller, and only code running inside ps2_input could read it) `poll/absorb.rs:43`: a rolling ring keeps the last 256 scancode bytes, about 128 keystrokes. -> D2
- **medium** `mouse/parser.rs:35`: no resync timeout and no reset detection; a lost byte or a mouse reset leaves phantom clicks or a dead pointer. -> wave 4
- **medium** `keymap/translate.rs:30`: the Pause key posts Left Ctrl and Num Lock; Print Screen and Break map to nothing. -> wave 4
- **medium** `poll/absorb.rs:63`: lock keys toggle state but never the LEDs. -> wave 4
- **medium** `init/enable_mouse/detect_wheel.rs:32`: PS/2 touchpads run as bare 3- or 4-byte mice. -> wave 4 (named there)
- low: `ring/state.rs:41` Ring::queued inverted and README misstates overflow; `init/wait.rs:60` bring-up waits busy-spin on PIO syscalls; `poll/mod.rs:18` dead diagnostic code and files over 75 lines.
