# virtio_gpu_proofs

Host proofs for the virtio-gpu driver (`capsule_driver_virtio_gpu`). The crate
includes the shipping driver source with `#[path]`. The driver reaches its
registers and its queue through bounds-checked views (`nonos_virtio::Mmio`),
so windows in host memory (`nonos_devmodel`) stand in for the common, notify
and device capability regions and for the queue's DMA grant, and a part on its
own thread consumes the ring the way a device does. A `nonos_libc` shim keeps
the server's replies and debug lines for the tests to read, keeps each
release and PCI command write, showing the test the device at that moment,
and makes DMA grants in host memory, kept per thread so a test sees which
are still held; it has no `MkExit`. A clock counts each question a wait asks
as a millisecond, and yields through the shim, where a host model can hang.

## What it proves

80 `#[test]` functions, and one check held when the crate builds:

- the bring-up on the shared transport: the status handshake, the features
  taken against those offered (ACCESS_PLATFORM whenever offered, never
  assumed), where the part is told its rings are, the ring cut to a power of
  two the region holds, refusals left FAILED, the doorbell bound
  (`modern_tests`, `modern_refusal_tests`, `modern_queue_tests`,
  `notify_bound_tests`), Bus Master only once a device a predecessor left
  live is read back reset with its queue on this grant, DRIVER_OK after it,
  and never for a device refused at any step (`master_tests`), the device
  configuration offsets (`config_tests`), only the modern id matched
  (`discovery_tests`), and discovery reading at least 128 device records
  (`discover.rs`, at build);
- the control queue: every command a two-descriptor chain, the answer read
  from the descriptor the part named, an answer out of turn (another
  descriptor, or the used index moved by two) or no answer by the deadline
  ending in a device reset, and a part that wakes late never running a stale
  chain with the next command's bytes (`queue_tests`, `late_tests`,
  `queue_limit_tests`), the wire bytes of each command (`reply_tests`,
  `reply_refusal_tests`, `wire_tests`, `wire_scanout_tests`);
- the scanout mode, the device's own from 16x16 up and within one display
  grant, judged by the same grant as the wire's MODES (`mode_tests`);
- a stall on the EDID or 3D probe at bring-up: the queue restarted on the
  same grant and the outputs still taken on the 2D path, every command
  under DRIVER_OK (`probe_stall_tests`);
- **the display wire (G-77, `gpu/`, `wire_sweep/`):** a host model answers
  the ring on the driver's own thread through the yield each wait makes. It
  keeps resources with their own pixels and backing and what each scanout
  shows, sees the ring wiped by a restart as the reset it is, and can be cued
  to stop answering (for a while or for good), to refuse a command, or to
  kill the driver, at any command. It proves:
  - the takeover of every enabled scanout;
  - CAPS, MODES, STATE and the host's display events;
  - flushes;
  - a mode set wrong at every command shows a whole picture, old or new (or
    nothing, with the driver `Down`, when the host never answers again), and
    the host never reads pages the driver gave back;
  - a surface the host keeps waits in limbo and goes when the host answers,
    or when a recovery has reset the host;
  - a silent owner's blank wrong at every command leaves none of its frame
    once a transfer landed, and is finished on later wakes;
  - the shared wire sweep, run on the driver's own server;
- a stall recovered in place (`gpu/recover_tests`, `gpu/recover_down_tests`,
  `gpu/recover_row_tests`): the queue restarted on its grant and every
  output shown again on its own surface from the kept pages, nothing given
  back; a try the host stalls on followed by the next; a host that never
  answers again leaving the driver `Down` after three tries, holding the
  claim and every grant, its outputs LOST and dark under a new generation,
  FLUSH refused `ENXIO`, reported on one turn only; `Down` at the fourth
  stall after three recoveries in a row without SERVED_MS served, and a
  device that serves that long forgiven every time; `quiesce` resetting the
  device before it is given back (`teardown_tests`);
- in the shipping text, the device reset before its claim is released, DMA
  unmapped by the driver only where the host can no longer read it (limbo,
  a surface never made, limbo after a reset), bus mastering only after the
  reset (`order_tests`), nothing in the server releasing the device or
  leaving but on a DGON, every turn settling the queue before it reads a
  frame, and a failed registration releasing the device only after a reset
  (`recover_order_tests`).

## What it does not prove

The broker calls, the receive loop, the virgl probe's pixels (its readback
grant stays zeroed, so it never reports 3D), pixels on a screen, and anything
the model's reading of the 2D commands gets wrong (it is recalled, not
checked against the specification).

## Run

```sh
cd userland/virtio_gpu_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
