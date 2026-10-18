# nonos_blk_wire

`nonos_blk_wire` is the capsule side of the NONOS block wire (magic `NBW1`):
the 20-byte request and reply header, the six ops, the GEOMETRY and IDENTIFY
bodies, the statuses a driver answers with itself, and `medium_rule`, who a
driver serves raw sectors to. `no_std`, no dependencies.

One crate for every disk driver behind the wire (`driver.sdhci0` today, the
UFS driver next). The kernel's client (`src/hardware/block_device/wire`) and
the installer's (`nonos_blk_client`) are held to it in `emmc_proofs` and
`fs_proofs`, through sdhci's `protocol` module, which re-exports it.

What is a driver's own stays in the driver: the endpoint it replies to and
which units it serves.
