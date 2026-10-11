# usb_audio_proofs

Host proofs for the USB audio class capsule (`capsule_driver_usb_audio`).
The crate mounts the shipping `uac`, `wire`, queue and answer source with
`#[path]` and runs them on built descriptors and on requests the audio
server's own sink client builds, without a boot and without a device.

## What it proves

27 `#[test]` functions:

- the playback alternate found in QEMU's usb-audio and in a headset with a
  24-bit alternate and a microphone; formats, rates, protocols and
  endpoints that are refused; a descriptor whose length lies
  (`playback_tests`, `walk_tests`);
- the outputs from Output Terminals (`terminal_tests`);
- packet and batch sizes at full and high speed, with the high-bandwidth
  bits, and every batch fitting one isoch request (`timing_tests`);
- the SET_CUR rate bytes and a port's speed (`request_tests`);
- the queue (`queue_tests`);
- every answer, read back with the sink's readers (`answer_tests`,
  `refusal_tests`);
- the op numbers and lengths equal to driver.hda0's (`peer_tests`).

## Run

```sh
cd userland/usb_audio_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
