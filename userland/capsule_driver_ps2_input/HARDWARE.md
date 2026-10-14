# capsule_driver_ps2_input: on real hardware

What a boot on real machines has to show before the driver is called
hardware-complete. The summary is in [README.md](README.md).

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers,
the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the
  capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller
  present, not started` line needs the Debug capability, which this manifest
  does not grant, so the kernel prints `[EXIT] <service> status 2: no device
  present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup
  error): at most seven claim and release rounds in the broker log over about
  six seconds, then one `device present, bring-up failed` line naming the last
  cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the
  kernel names as `[EXIT] <service> status 6: device present, bring-up failed
  and was given up`. The capsule holds no core while it waits.
- On a legacy-free machine (no i8042) the probe takes one claim and one release,
  and the capsule exits `EXIT_ABSENT` without a retry.
- On an EC-emulated i8042 the keyboard types and, where fitted, the touchpad or
  mouse moves; a wheel mouse scrolls.
- A firmware-disabled aux port is switched back off and the keyboard keeps
  scanning.
- Caps Lock lights its LED and the case follows; Num Lock is lit from start.
- Unplugging and replugging a PS/2 mouse (or a KVM switch) brings it back, wheel
  included, and a key let go meanwhile is released.

## Evidence still required

Before this driver is called hardware-complete: QEMU keyboard scancode
injection reaches the kernel input ring; QEMU AUX pointer injection reaches
`OP_POLL_MOUSE`; teardown logs show PIO, IRQ1, IRQ12, and both device claims
revoked; a real machine with legacy PS/2 input records keyboard and mouse
events; a USB-only machine proves the absence of this capsule does not block
the USB HID path.

## See also

- [README.md](README.md)
- [RELEASE.md](RELEASE.md)
- [LIFECYCLE.md](LIFECYCLE.md)
