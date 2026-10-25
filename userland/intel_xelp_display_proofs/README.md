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

Run with `cargo test`; the 40 mutants of the letter gpu-2a are each caught.
