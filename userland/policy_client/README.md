# nonos_policy_client

`nonos_policy_client` reads the system policy store, `capsule_policy`. A
capsule that applies a setting needs the store's port, one request and reply,
and confidence that the reply answers the question it asked; this crate is
those three, written once. It is `no_std` and uses two syscalls,
`MkServiceLookup` and `MkIpcCallTimeout`, through the libc with its default
features off, so a std capsule that reaches it through the app skeleton still
links with one panic handler.

## What it offers

- `lookup()`: the store's port, or `None` while `policy` has not registered.
- `get_bool`, `get_u8`, `get_i8`, `get_str`: one `OP_GET` for a `Field`, or
  `None` when the store did not answer, refused, or answered with the wrong
  kind.
- `call`: the round trip under them. It waits 200 ms, and returns a reply only
  when it is at least a header long, its status is `E_OK`, its op is the one
  sent, and its stated payload fits what arrived.
- `status`: the kernel hardening record the store forwards (`OP_STATUS`),
  copied only when it arrives whole.
- `Watch`: follows one enumerated field, asking at most once per interval and
  handing back a value only when it changed, for a capsule with no event to
  wait on.

## Who uses it

`nonos_route_link` reads `Field::NetworkRoute` with it to choose the network a
connection leaves by; an unanswered read there is the Nym mixnet, never Direct.
`net.core` reads `Field::Persistent` and `Field::WifiRadio` before Wi-Fi
autojoin. The desktop shell, the PS/2 and USB HID drivers, vfs, the setup
wizard, About, the browser and `nonos_wifi_client` read their own fields.

## What it does not do

- It never writes: there is no `OP_SET` here. Writers build their own request
  with `nonos_policy_proto`.
- It caches nothing; every read is a round trip.

## Capabilities

A library adds no capability of its own. The capsule using it needs IPC.

See [the SOCKS5 bridge and network routes](../../docs/handbook/network/socks5-and-routes.md).
