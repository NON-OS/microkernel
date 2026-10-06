# nonos_policy_proto

`nonos_policy_proto` is the wire format of the system policy store,
`capsule_policy`, and the one place every reader and writer of a setting takes
its numbers from. It is `no_std`, has no dependencies and makes no calls:
bytes on the wire and the tables that name them. `nonos_policy_client` reads
the store with it; Settings, setup, the desktop shell, `net.core`, the browser,
the terminal, `nonos_route_link` and others name fields with it.

## The service

- `POLICY_SERVICE_NAME` is `policy`, on `POLICY_SERVICE_PORT` 4108 with reply
  port 4109 (`src/service.rs`).
- Ops (`src/ops.rs`): `OP_GET` 1 and `OP_SET` 2 address one field; `OP_STATUS` 3
  asks for the kernel hardening record and uses no field.
- Header (`src/hdr.rs`), 12 bytes little endian: op `u16`, field `u32`, kind
  `u8`, a zero byte, status `u16`, payload length `u16`.
- Kinds (`src/kind.rs`): bool, u8, i8, str and bytes. Status codes
  (`src/errno.rs`): `E_OK`, `E_NOT_FOUND`, `E_WRONG_KIND`, `E_BAD_LEN`, `E_INVAL`,
  `E_ACCES`.

## Fields

`Field` (`src/field.rs`) numbers each setting; the high byte is its `Category`:
`0x01xx` user settings, `0x02xx` kernel hardening flags, `0x03xx` identity
(host name, domain, user name, the Qwen tier). `kind_of`, `max_of`,
`str_max_of`, `label_of` and the label tables say how each is typed, bounded
and shown.

`Field::NetworkRoute` (`0x0124`) is the network the system's own traffic
leaves through by default (`src/route.rs`): `NYM` 0, `ANYONE` 1, `DIRECT` 2.
`known` says whether a value is one this build knows; `nonos_route_link`
reads an unknown value as the mixnet. `Field::Persistent` and `Field::WifiRadio` are the
two `net.core` reads before it joins a saved Wi-Fi network.

## Other tables

- `apps`: the bit for each optional app and whether it is switched off
  (`Field::AppsOff`), shared with the kernel's mirror.
- `setup_record`: the answers first-boot setup keeps on a machine that keeps
  state, with fixed lengths per version, written answers first and marker last
  so a half-finished save restores nothing.

## Tests

`tests/` checks the app switches against the kernel's mirror and the setup
record's versions, refusals and route field. `userland/policy_proofs` and
`userland/route_link_proofs` use the crate on the host.

See [the SOCKS5 bridge and network routes](../../docs/handbook/network/socks5-and-routes.md)
for how the route field is read.
