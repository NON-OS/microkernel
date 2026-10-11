# nonos_scsi

The SCSI command layer every SCSI host driver shares: the CDBs (INQUIRY,
TEST UNIT READY, READ CAPACITY (10) and (16), MODE SENSE (6), READ and
WRITE (10) and (16), SYNCHRONIZE CACHE, REPORT LUNS), sense data in fixed
and descriptor format and the verdict on it (done, try again, or the block
wire status to answer), the INQUIRY, serial page, capacity and LUN list
parsers, and a disk's probe, refresh, reads, writes and flush over the
`Hba` trait, under one retry rule (four sends at most: unit attentions,
aborted commands, BUSY and a resetting bus).

A host driver implements `Hba`: one command to a target and LUN, its data
through the driver's buffer, the status byte and the sense back. Pure and
allocation free; `cargo test` runs every decision on the host. `src/sim.rs`
is a simulated logical unit (QEMU's scsi-hd and scsi-cd) for these tests
and for each host driver's proof crate; it is never part of a driver.
