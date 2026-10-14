# capsule_driver_i2c_pci: on real hardware

What a boot on real silicon has to show before the capsule is called done.
The summary is in [README.md](README.md).

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
- Every LPSS I2C controller is found (PCI or ACPI); the one whose bus answers
  the touchpad's address is bound.
- On firmware that fills the touchpad's address in at run time, the sweep finds
  a pad at 0x15, 0x2C or 0x10 on the controller the declaration names, and the
  HID driver binds it.
- The SCL counts give a fast-mode bus within spec on the real input clock.
- When no controller comes up, the pass is retried and then given up on, with
  every controller released in between.

## See also

- [SETUP.md](SETUP.md)
- [RELEASE.md](RELEASE.md)
