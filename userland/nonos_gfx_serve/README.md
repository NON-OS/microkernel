# nonos_gfx_serve

The display wire's server side for drivers that scan out of their own
memory (PLAN-coverage section 12), written once for bga, the boot
framebuffer and any VRAM device. `no_std`; it depends only on
`nonos_gfx_wire`.

A driver implements two traits:

- `Display`, the device: `caps`, `modes`, `state` (device events read
  here), `set_mode` (program the mode and show the shadow before
  returning), `flush` (make the rectangles visible), `cursor`.
- `Platform`, the kernel: `alloc` from the display pool, `register` a
  surface, `share` it with a pid, `release` it, the region's `bytes`, and
  the driver's `pid`.

`Server::handle(sender, frame, out)` answers every op: CAPS, MODES and
STATE from the device (STATE carrying the current serial); SURFACE makes
the primary on first ask at the mode the device shows and shares it, a new
asker becoming the owner and freeing a primary still waiting for RETIRE;
MODE_SET checks the size (range devices: 640x480 to the maximum, aligned,
within the budget; list devices: a listed mode), makes the new primary,
shows it, shares it and answers it while the old one waits, one at a time
(EBUSY), keeping the old mode on any failure; RETIRE frees the waiting
primary (the current serial is EINVAL, any other ESTALE); FLUSH checks the
serial (ESTALE) and every rectangle (EINVAL); CURSOR hands over the cursor
slot on devices with HW_CURSOR (else ENOSYS). Another output than the
device has is ENXIO; an unknown op ENOSYS. Serials are pid << 16 | counter.

`copy_rects` is the copy a VRAM driver's flush makes, with an optional red
and blue swap for framebuffers laid out red first.
