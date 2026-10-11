# nonos_modes

Display timings computed one way by every display driver. `no_std`; its
only dependency is `nonos_edid`, whose `Detailed` is the timing type here.

## API

- `dmt(width, height, hz, reduced) -> Option<Dmt>` and `dmt::all()`: the
  VESA DMT progressive modes, from `data/dmt.txt`, built into a table at
  compile time (a malformed line stops the build).
- `cvt(width, height, refresh_mhz, Blanking) -> Option<Detailed>`: CVT 1.2
  standard blanking, reduced blanking v1 and v2, progressive, in exact
  integer arithmetic so each floor lands where the standard puts it.
- `compute(width, height, refresh_mhz, Blanking)`: the DMT mode where one
  matches, else CVT. `resolve(&nonos_edid::Timing)`: an EDID timing made
  whole: its own detail, or the DMT or CVT timing its source names.
- `clock_ok(&t, max_khz)`, `in_range(&t, &RangeLimits)`: the pixel clock,
  and every rate against the monitor's range limits descriptor.
- `carries(Link, pixel_khz, bpp)`: DisplayPort (8b/10b to HBR3, 128b/132b
  for UHBR) and TMDS (deep colour scales the character rate) capacity.
- `fit(want, current, Option<&Range>, modes)`: the display wire's rule for
  the size to ask for: nothing for a zero or current size; a range adapter
  clamped to 640x480..max, its width alignment and its surface budget; a
  list adapter's largest mode inside the wanted size.

## Sources

VESA DMT 1.0 r13 (`data/dmt.txt` names its source and how it is checked),
VESA CVT 1.2, E-EDID 1.4, DisplayPort 1.4a and 2.0, HDMI 1.4b, and the
display wire in PLAN-coverage section 12.
