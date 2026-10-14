# capsule_driver_i2c_pci: setup and failure

How the capsule picks a controller and the touchpad on it, and how it fails.
The summary is in [README.md](README.md), who may reach which device in
[CONTRACT.md](CONTRACT.md).

## Runtime lifecycle

Startup discovers every Intel LPSS controller, on PCI or as an ACPI serial-bus
record whose source clock the broker carries in the BAR's `aux` word. For each
one it tries, it claims it, maps BAR0, verifies the DesignWare component type
when exposed, masks controller interrupts and clears pending interrupt state.
It keeps the first controller whose bus returns an HID descriptor from one of
the touchpad addresses ACPI declared, else one where a declared device only
acknowledged, else one a candidate's `_CRS` names, else the first that comes
up, releases the others, and serves IPC (`src/setup/sequence/run.rs`).

Only declared addresses are probed for a descriptor, which writes a register
pointer to the device. Intel's reference firmware declares its pad as TPD0
with address 0 and fills the address in at run time, which the kernel cannot
run. On the controller such a declaration names, and only there, the sweep
tries only the three addresses touchpads commonly ship at, 0x15, 0x2C and
0x10, reading one byte from each in that order, which writes nothing
(`src/setup/sequence/sweep.rs`). The first that answers is bound and joins
the reach even if it is not a pad, and the HID driver then writes and reads
it on every probe; so no address where boards put a GPIO expander (0x20,
0x24), a thermal sensor (0x4B, 0x4C) or a touchscreen (0x38) is tried, and
no address after the first answer is touched. A pad elsewhere, or one that
does not acknowledge a plain read (the descriptor probe's notes say some ELAN
parts do not), is not found this way; once the kernel evaluates `_CRS` for
the pad (K13 in the plan), the sweep goes. With nothing
declared, nothing on the bus is probed. Process teardown and broker
revocation release every hardware grant.

## Failure model

Unsupported PCI IDs, missing BAR0, failed grants, or an unreadable controller
window prevent the capsule from serving. With no LPSS I2C controller in the
device list the capsule logs one line and exits `EXIT_ABSENT` (2) before
claiming anything; when controllers are listed but none comes up, the pass
over them is retried on the shared bounded schedule (`nonos_libc::bring_up`:
seven tries, sleeping between them, every controller not kept released in
between), and running out exits `EXIT_GAVE_UP` (6). Transfers are bounded to
small controller-local buffers, carry an explicit timeout, and surface
DesignWare abort state so higher layers can distinguish NACK from bus
failure. A sweep that meets a faulting bus stops at its first address.

Every wait is bounded in elapsed time (`src/transaction/pace.rs`). While the
bus moves, and for about a FIFO's drain time after, it polls back to back;
then it yields between polls and reads the clock after every yield, so on a
busy machine a time slice runs past the deadline once at most; once the bus
has stopped moving for 4 ms it sleeps a millisecond between polls instead.
Two faults in a row (busy or timed out) reset the controller, the LPSS
function reset and the whole bring-up again; the bus is then `Ready` again or
named `Stuck`, said once on the console with Debug, and the reset is
retried after further faults (`src/transaction/recover.rs`).
