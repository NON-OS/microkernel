# nonos_gfx_wire

The display wire, NVGP v1 (PLAN-coverage section 12), as one `no_std`
crate both ends link: every display driver serves it, the compositor
drives it. Nothing allocates; every decoder refuses a short or malformed
frame with `EINVAL` rather than panicking.

## API

- `Header { op, flags, output, request_id, payload_len }` with `encode` and
  `decode` (magic 0x4E56_4750, version 1, 20 bytes); `Request::decode`
  holds a frame to its declared payload and the 256 byte limit.
- `reply(&Header, status, body, out)` and `Reply::decode(frame, &asked)`,
  which holds a reply to the op, output and id it answers.
- `ops`: HEALTHCHECK, the private range, CAPS 0x10 to CURSOR 0x17, and the
  MODE_SET budget. `errno`: -6, -11, -12, -16, -22, -38, -110, -116.
- Bodies: `Caps` (32 bytes, the flag bits in `body::caps_flags`), `Modes`
  (up to 20 `Mode`s), `State` (32 bytes), `Surface` (32 bytes, ARGB8888),
  `Flush` (1 to 15 `Rect`s), `Cursor` (20 bytes, at most 64 x 64).
- `kind`: 1 virtio-gpu, 2 DISPI, 3 SVGA II, 5 synthvid, and G's 6 boot
  framebuffer, 7 Intel, 8 AMD.
- `serial(pid, counter)`, `pid_of(serial)`.
- `fit(want, current, &Caps, &[Mode])`: section 12's resize rule, over
  `nonos_modes::fit`.
