# nonos_edid

The identification data a monitor hands over, parsed without allocation:
VESA E-EDID 1.4, its CTA-861 extension and DisplayID 2.0 sections carried
in extension blocks. `no_std`, no dependencies. The loader and every
display capsule read a panel through this crate, and `edid_proofs` runs
the same source on the host.

## API

- `Edid::parse(&[u8]) -> Result<Edid, Error>`: the base block's header and
  checksum must hold (`Error::Short`, `Header`, `Checksum`). The extension
  count is byte 126, or the HDMI Forum EEODB's when block 1 carries one,
  cut to the blocks actually handed over. Extensions are checked one by
  one as they are read; a bad one is skipped.
- Identity: `manufacturer`, `product`, `serial`, `serial_text`, `name`,
  `version`, `digital`.
- `native()`: the base block's first detailed timing and the image size,
  exactly as the loader has read it. `size_mm()`, and `physical_mm()`
  packed `w | h << 16` for the display wire's CAPS.
- `preferred()`: the base block's first detailed timing, else a DisplayID
  type VII timing marked preferred, else a CTA extension's first one.
- `for_each_timing(&mut dyn FnMut(Timing))`: every timing named anywhere,
  base block first: established I/II and III, standard (with 0xFA), CVT
  three byte codes, detailed timings, then per extension CTA detailed
  timings and VICs, or DisplayID type VII. A `Timing` without `detail`
  names a picture and a rate only; `nonos_modes` computes its timing.
- `range_limits()`, including the 1.4 offset flags and CVT's clock trim.
- `max_tmds_khz()`: the largest of the HDMI VSDB, HF-VSDB and HF-SCDB
  rates; 0 when none is given.
- `for_each_audio`, `basic_audio`: CTA short audio descriptors.
- `tile()`: DisplayID tiled display topology.
- `vic(code) -> Option<Vic>`: the CTA-861 VIC table, built at compile time
  from `data/cta861_vics.txt`.

## Sources

Every layout is cited in the file that reads it: VESA E-EDID 1.4,
ANSI/CTA-861-H, HDMI 1.4b and 2.1 (the vendor blocks), VESA DisplayID 2.0
and 1.3. Where a bit placement was not checked against the document it is
marked unverified. `data/cta861_vics.txt` names its own source.
