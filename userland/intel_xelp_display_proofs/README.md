# intel_xelp_display_proofs

Host proofs for `nonos_intel_xelp_display`. The crate runs over
`src/model`: a register map that logs every read and write, a TRANS_CONF
whose state bit follows its enable bit a set number of reads later (or
never), and clocks that move a fixed step per read or cannot be read at all.

Each test states the PRM's address, field or limit itself, with its page,
rather than reading the crate's constants, so one change cannot move both
sides of a test. What is proved:

- every register of transcoders A to D, and PIPE_SRCSZ of pipes A to D, at
  the address Vol 2c lists; no eDP transcoder;
- the words for CTA-861 1080p and DMT 800x600, worked out by hand;
- each timing limit: the value at the limit taken, one step past refused,
  for DisplayPort and for HDMI;
- a refused timing, or the eDP transcoder, reads and writes nothing; a
  transcoder that reads enabled or still running is never written;
- TRANS_CONF on and off: the bits written, the others kept, the wait's
  length, a state that lands on the last read counted, an enable that never
  lights turned back off, and a dead clock as a timeout;
- a sweep of 200,000 random timings: each refused with nothing written, or
  written with every field decoded back inside Vol 2c's limits.

- the PLL registers and fields at their Vol 2c addresses and bits;
- every row of Vol 12's DisplayPort table giving back its rate by Vol 12's
  own formula, and its P, Q and K as the table writes them;
- Vol 12's worked HDMI example (113.309 MHz), the symbol clock's range, and
  every symbol clock re-derived from the rule, plus each divider's DCO
  bounds and the facts that make three rules unobservable on kHz inputs;
- the PLL sequences' order, a power or lock that never comes leaving the
  PLL down, and the port clock's two writes.

- the combo PHY: register addresses, the procmon table column by column,
  initialisation's values and order (and B refused before A), every swing
  row and lane field, loadgen per lane, IO power and the DDI buffer. The
  model fans group writes out to the lanes and answers group reads with
  junk, so a read of a group address would be seen.

Run with `cargo test`. Letters gpu-2a to gpu-2c list the mutants caught.
