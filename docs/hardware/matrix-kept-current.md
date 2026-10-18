# Hardware support matrix: How the matrix is kept current; See also

Part of [Hardware support matrix](MATRIX.md).

## How the matrix is kept current

No tool writes this page. Before the commit in its footer moves, the NONOS team checks every row against the tree by hand:

1. Each row's ids are read again from the tables listed under Sources. A new id, a new driver or a driver that enters or leaves the image becomes a row change.
2. The state follows the code: what the kernel spawns at boot (`src/userspace/init/spawn_plan`), what the driver refuses and why, and what is missing.
3. Proof crate counts and results come from the flake checks for that exact commit. A failing check is shown as failing, with its cause.
4. Real hardware enters the How verified column only with a hardware report made as [Reporting a machine](report.md) describes. A report that cannot name its image commit says so, as the one above does.

A machine is listed by its chips and their ids, never by brand or model.

## See also

- [Reporting a machine](report.md)
- [Drivers](../drivers/README.md)
- [Input drivers](../drivers/input/README.md)
- [Audio](../drivers/audio.md)
- [Installation requirements](../install/requirements.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
