# Media

hda, virtio_gpu, bga, rtsx, and virtio-rng both as a capsule and as the in-kernel driver.
The kernel's entropy plumbing is in kernel-grants-and-ipc.md. Notation: README.md.

## capsule_driver_hda: near

Intel HD Audio playback only (one 48 kHz 16-bit stereo stream), held to audio.server. Paths under userland/capsule_driver_hda/src/ unless they start with userland/.

- **medium**, was high (a window of a few syscalls, and only without an IOMMU) `handles/broker_handles_drop.rs:23`: teardown unmaps the CORB/RIRB frames while the RIRB engine and Bus Master are still on, on the Silent verdict (every HDMI-only controller) and on errors, so a late codec answer lands in a freed frame. -> D8
- **medium** `controller/codec/path.rs:92`: the codec path search has no work budget; a dense or broken widget graph from codec firmware can cost billions of steps at bring-up. -> wave 4
- **medium** `controller/codec/program.rs:170`: input-amp verbs alias for connection index 16 and up, which can mute the chosen path or unmute another input, such as mic loopback to the speakers. -> wave 4
- **medium** `controller/verb.rs:102`: verb waits busy-spin for 100 ms, and the jack poll keeps sending verbs to a dead codec forever. -> wave 4
- **medium** `userland/capsule_driver_hda/README.md:225`: no capture and no hardware volume; capture will need a consent gate and an indicator that do not exist. -> wave 4
- **medium** `userland/hda_proofs/src/lib.rs:32`: setup and teardown order, server handlers, refill and hostile codec graphs are untested. -> D8 in part
- low: `controller/reset.rs:117` stop_engines ignores whether inherited engines stopped; `server/runner/refill.rs:81` 76 unsafe uses, 2 SAFETY comments, DMA as raw u64; `setup/sequence.rs:79` 23 files over 75 lines.

## capsule_driver_virtio_gpu: partial

virtio-gpu 2D scanout for the compositor (one shared primary surface), polled. Paths under userland/capsule_driver_virtio_gpu/src/ unless they start with userland/.

- **refuted**, split from the critical (virtio-gpu has no transitional device and QEMU forces the modern transport, so the path is dead code) `init/legacy.rs:53`: the legacy transport writes an IOVA page number. -> D7 deletes the legacy path
- **medium** `device/virtqueue/submit.rs:48`: the completion wait is a spin and yield count, and a timed-out command stays published while its descriptors and staging page are reused, so a command can run twice. -> wave 4
- **medium** `server/handlers/create_resource.rs:67`: there is no resource destroy; the 64-entry table fills for good, and a full table leaks the host resource just created. -> wave 4
- **medium** `setup/primary_surface/geometry.rs:29`: the device-reported scanout size is only lower-bounded, so a huge mode fails bring-up instead of falling back. -> wave 4
- low: `setup/probe_3d/run.rs:67` 3D probe grant never returned; `device/cmd/submit_3d.rs:31` 3D is a boot self-test whose stated limits do not match the queue; `device/virtqueue/desc.rs:23` 25 unsafe uses, 2 SAFETY comments; `setup/sequence.rs:26` five files over 75 lines.

## capsule_driver_bga: skeleton

Parked Bochs Graphics Adapter source: no Capsule.mk, no image, serves nothing. Paths under userland/capsule_driver_bga/src/ unless they start with userland/.

- **low**, was high (at worst the capsule faults; no effect outside it, and nothing ships it) `setup/sequence.rs:41`: the framebuffer clear writes 3 MiB without checking the mapped BAR0 length. -> D8
- **low**, was high (the documented parked state) `main.rs:33`: once the mode is set it only sleeps holding the claim, with no IPC and no surface. -> wave 4 (named there)
- **medium** `dispi/set_mode.rs:30`: the DISPI mode is written blind: no ID handshake, no read-back, no video-memory check. -> wave 4
- low: `userland/bga_proofs/src/clear_tests.rs:56` the clear is tested against its own span, not the mapping.

## capsule_driver_rtsx: partial, not in the image

Realtek RTS5227/RTS522A SD card reader: identifies a card and reads block 0. Paths under userland/capsule_driver_rtsx/src/ unless they start with userland/.

- **low**, was high (a documented roadmap item; nothing runs) `report.rs:51`: there is no block device; the card is read only for a log line. -> wave 4 (named there)
- **medium** `setup/driver.rs:17`: the DMA regions drop before the device is released and quiesced. -> wave 4
- **medium** `card/read.rs:61`: removing the card mid-transfer is not detected, so the read waits out its 10 s. -> wave 4
- **medium** `engine/wait.rs:30`: hardware waits busy-spin for up to 10 s. -> wave 4
- **medium** `userland/rtsx_proofs/src/lib.rs:22`: the engine, card and setup flows are untested, and the parsers have no hostile fuzzing. -> wave 4
- low: `engine/wait.rs:42` an all-ones BIPR returns Gone without stopping the engines; `log/mod.rs:27` function bodies in a mod.rs.

## capsule_driver_virtio_rng: partial

virtio-rng capsule serving raw bytes to the kernel only (held endpoint); the kernel never calls it. Paths under userland/capsule_driver_virtio_rng/src/ unless they start with userland/.

- **medium**, was high (latent: the endpoint is kernel-only and the kernel never calls it; high once wired into the pool) `fill.rs:39`: after one timeout the queue runs one completion behind and can serve the same bytes to two callers. -> D7
- **medium**, was high (a window of a few syscalls; it faults under VT-d) `setup/driver.rs:44`: release() unmaps the buffer and ring before the device is reset or released, right after a timed-out fill. -> D7
- **medium** `init.rs:55`: legacy bring-up shrinks the device's queue size, which a legacy driver cannot do; a device with more than 16 entries is never served. -> wave 4
- **medium** `fill.rs:22`: the completion deadline is a count of 100,000 yields, not a time. -> wave 4
- **medium** `userland/virtio_rng_proofs/src/tests/queue_tests.rs:61`: the proofs assert the legacy queue shrink and never run a late completion. -> D7 in part
- low: `queue/used.rs:89` raw-pointer Queue is Copy, two conflicting Safety sections, four files over 75 lines.

## kernel src/drivers/virtio_rng: skeleton


- low: `device/random.rs:28` 100,000-spin timeout under the device lock; `device/io.rs:16` files over 75 lines and statics in mod.rs.
