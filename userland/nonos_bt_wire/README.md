# nonos_bt_wire

Bluetooth wire formats for an LE-only host (Decision 21: LE only, HID over
GATT). Written from the Bluetooth Core Specification 5.4. Every fact in the
source cites its volume, part and section.

| Module | What it holds |
|---|---|
| `cursor` | Little-endian field reader and writer (Vol 4, Part E, 5.2). A short buffer is an error, never a panic. |
| `addr` | Device address, public and random types, random address kinds (Vol 6, Part B, 1.3). |
| `hci::command`, `hci::opcode` | Command packets and the opcodes an LE host uses (Vol 4, Part E, 5.4.1 and 7). |
| `hci::event`, `hci::disconnect`, `hci::le_meta`, `hci::reports` | Event packets, Command Complete and Status, Disconnection Complete, Number of Completed Packets, and the LE Meta subevents Connection Complete, Advertising Report and Long Term Key Request (Vol 4, Part E, 5.4.4 and 7.7). |
| `hci::acl`, `hci::boundary` | ACL data packets and the Packet_Boundary flag (Vol 4, Part E, 5.4.2). |
| `l2cap` | The basic frame, LE channel IDs, and joining ACL fragments into PDUs with a bound per link (Vol 3, Part A, 2.1, 3.1 and 7.2). LE signaling commands (Vol 3, Part A, 4), the connection parameter ranges a central accepts (4.20, 4.21; Vol 6, Part B, 4.5.1 and 4.5.2), and `answer_central`, the central's answer to each signaling frame. |

The crate is `no_std`. It has no transport and no state beyond one
reassembler per link. HCI over USB (`nonos_bt_hci`) and SMP (`nonos_bt_smp`)
are built on it, and GATT will be. The proofs live in `nonos_bt_wire_proofs`.
