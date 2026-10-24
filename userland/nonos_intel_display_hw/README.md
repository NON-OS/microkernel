# nonos_intel_display_hw

The contract every Intel display engine generation's code shares, as a leaf
crate (`no_std`, depending only on `nonos_edid`):

- `access`: `Mmio` (32-bit registers and 64-bit GGTT entries by byte offset
  into BAR0), `Clock` (monotonic microseconds, `None` when unreadable),
  `Expiry` and `Until` (a clock deadline, never a poll count), and the
  `rmw` and `field` helpers;
- `error`: `Error`, why the engine refused or gave up; every check runs
  before the first write;
- `pipe`: `Pipe` and `Transcoder`;
- `modeset`: `Output` and `ModeSet`, the trait a generation's mode setting
  implements: `set`, `off`, `hpd_poll`.

`capsule_driver_intel_display` re-exports the first three as
`engine::access`, `engine::error` and `engine::pipe`, and implements Gen9's
mode setting itself. A later generation's mode setting lives in its own
crate and builds against this one, so neither crate depends on the other.

Proved through `intel_display_proofs`, which mounts the capsule's engine
over a register model of the display engine.
