# I2C-HID touchpads: authority, limits and checks

What the two touchpad capsules may do, what they refuse or leave out, and how
they are checked. What they are is in [I2C-HID touchpads](i2c-hid.md).

## Authority and limits

- `driver.i2c_pci0` holds IPC, Memory, Driver, DeviceEnum and Mmio
  (`CAPSULE_REQUIRED_CAPS`, `userland/capsule_driver_i2c_pci/Capsule.mk:18`). It
  has no Irq, as its engine polls, no DMA and no Debug, so its own
  `driver.i2c_pci:` lines never reach the console.
- `driver.i2c_hid0` holds IPC, Memory and InputSource, and Debug only in a build
  with `capsule-serial-debug` (`CAPSULE_OPTIONAL_CAPS`,
  `userland/capsule_driver_i2c_hid/Capsule.mk:14-17`).
- A transfer writes at most 64 bytes and reads at most 1024
  (`TRANSFER_WRITE_MAX`,
  `userland/capsule_driver_i2c_pci/src/protocol/limits.rs:7-8`).
- `driver.i2c_hid0` looks the controller service up 100 times, 20 ms apart, then
  exits with status 2 (`LOOKUP_ATTEMPTS`,
  `userland/capsule_driver_i2c_hid/src/i2c_client/service.rs:10-11`). After 3
  unanswered calls in a row it pauses its calls for 1 s, doubling to 30 s
  (`UNANSWERED_LIMIT`,
  `userland/capsule_driver_i2c_hid/src/i2c_client/gate.rs:33-35`).
- Refused: touchscreens, which the kernel leaves out of the device table, and a
  touchpad with a 10-bit I2C address, which gets the boot console warning above.
- Not supported: I2C keyboards and other HID-over-I2C devices that are not
  pointers, interrupt-driven transfers and DMA.
- No touch, report or gesture is stored. Each report is decoded, posted and
  dropped. With Debug, the driver writes the first 16 bytes of each of its first
  six raw reports to the console as `[i2chid] frm` lines, so a decode can be
  checked against the wire (`frame_dumps`,
  `userland/capsule_driver_i2c_hid/src/input/poll/read_frame.rs:55-58`).

## How this is checked

- [Proof crates](../../overview/glossary-peer-list.md#proof-crate) at commit bff12b97, all
  passing: `i2c_hid_proofs` runs both drivers' wire code against a modelled
  DesignWare core with a modelled touchpad (51 tests); `i2c_pci_proofs` holds
  the LPSS reset, the clocks and the id tables (35); `i2c_transfer_proofs` holds
  the transfer engine (26); `pinctrl_proofs` holds the GPIO controller ids and pad arithmetic (8 since F 46);
  `input_proofs` holds the gesture decoder (96). Since then `i2c_pci_proofs`
  holds 47, `i2c_transfer_proofs` 50 and `i2c_hid_proofs` 52, with the reach and
  its narrowing, the sweep for a pad with no declared address, and waits on a
  busy machine.
- The touchpad on Intel LPSS has one real-hardware report. Works on an x86_64
  laptop (Intel Gemini Lake, 8 GB), developer hardware report, 6 October 2026;
  the image commit was not recorded. The AMD path and every other Intel family
  have not been run on hardware in this release.

## See also

- [I2C-HID touchpads](i2c-hid.md)
- [Support matrix](../../hardware/MATRIX.md)
- [Reporting a machine](../../hardware/report.md)
