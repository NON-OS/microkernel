# nonos_bt_hci_proofs

Host tests for `nonos_bt_hci`. The crate depends on `nonos_bt_hci` and
`nonos_bt_wire` by path and calls their public API. Everything in `src/` is
`#[cfg(test)]`.

`sim/` is a simulated controller behind `UsbPipes`. It answers the
standard commands as the cited sections describe, or with the events a
test scripts for an opcode. It cuts each event into
16-octet interrupt transfers, as a device sends a long event. It records
every command and bulk transfer the host makes. So each test runs the host,
the USB transport and the joiner together.

36 tests:

| File | Tests | What it checks |
|---|---|---|
| `descriptor_tests.rs` | 3 | the Bluetooth interface and its three endpoints found, the voice interface ignored; another class skipped, only alternate setting 0 used; missing endpoints and broken descriptor lengths refused |
| `joiner_tests.rs` | 3 | an event over three transfers; two events in one transfer; ACL joined by its two-octet length, a packet past the bound refused and the buffer cleared |
| `transport_tests.rs` | 3 | a command is class request 0x20/0 with the packet as data; ACL out on bulk, a partial take refused; ACL in joined across bulk transfers |
| `bringup_tests.rs` | 3 | the seven commands in order, what is read back, both event masks byte by byte; shared buffers read with Read Buffer Size, no buffers refused; no LE, a failed Reset, and a silent controller (exactly the budget of pauses) each stop bring-up |
| `flow_tests.rs` | 4 | no command sent while the controller takes none; Command Status for a pending command, and for one that should complete; events met while waiting kept in order; a flood keeps only the newest `QUEUE_MAX` |
| `acl_tests.rs` | 3 | a PDU cut into first and continuing packets; a PDU needing more buffers than free not started, buffers given back by Number of Completed Packets and capped at the total; no ACL before bring-up, no empty PDU |
| `scan_tests.rs` | 3 | scan parameters and enable laid out byte by byte; out-of-range timing refused before anything is sent; an advertising report arrives whole through `next_event` |
| `connect_tests.rs` | 3 | LE Create Connection laid out byte by byte and the link read back; scan or link timing out of range refused unsent; a report met while connecting stays queued, a failed connection is its status |
| `connect_cancel_tests.rs` | 3 | a peer not heard within the budget is cancelled and its Unknown Connection Identifier taken; a cancel after the link came up is disallowed and the link kept, its event before or after the cancel's answer; any other refused cancel is its status |
| `encrypt_tests.rs` | 3 | LE Enable Encryption with Rand and EDIV zero and the LTK least significant octet first, a reserved handle refused; another link's Encryption Change kept; a refusal, and a change that leaves the link clear, are statuses |
| `encrypt_end_tests.rs` | 2 | a link that drops ends the wait with its reason and its Disconnection Complete stays queued; no change within the budget is a timeout |
| `update_tests.rs` | 3 | LE Connection Update laid out byte by byte and taken on Command Status; a refusal is its status; timing out of range or a reserved handle refused unsent |

Each expected byte string is laid out by hand from the cited section of the
Core Specification. None is taken from another project's tests.

Seven of the tests were checked against broken code; each break makes a
test fail:
- sending without a command credit;
- dropping one event-mask bit;
- sending the LTK unreversed;
- not searching the kept events while waiting;
- not keeping the Disconnection Complete that ends an encryption wait;
- taking a disallowed cancel as an error;
- taking another link's Encryption Change.
