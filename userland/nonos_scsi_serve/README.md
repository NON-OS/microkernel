# nonos_scsi_serve

The block wire (`NBW1`) served for the disks behind a SCSI host adapter,
once for every SCSI host driver: the units found on the adapter's bus
(`Units::scan`: every target, its REPORT LUNS, each LUN probed through
`nonos_scsi`, numbered by target then LUN), the serve loop and the medium
rule, the six ops mapped onto `nonos_scsi`, and the console lines (never a
serial number). A host driver brings its adapter up, implements
`nonos_scsi::Hba`, and hands it here with its reply endpoint and name.

`src/wire` holds the same items under the same names as `nonos_blk_wire`;
at integration it becomes that crate. Each host driver's proof crate
mounts these files by path under its libc shim.
