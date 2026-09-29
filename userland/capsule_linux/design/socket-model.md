# Socket model decision, item 9

`bind`, `listen` and `accept4` are in scope. A machine aimed at the Kali and
BlackArch tool set where nothing can listen is not that machine: reverse shells,
local proxies, and payload servers are the normal case, not an exotic one. So
implement all three.

They are mediated, not free. The rule is the one the rest of this kernel already
uses: a guest does what a capability lets it do, and the capability names the
resource rather than granting a class of operation.

## The capability

Add `NetBind`. It is not implied by the capability that lets a guest make an
outbound connection, and a guest holding neither still gets `connect`.

A holder of `NetBind` carries a bind set: a list of `(interface, port range,
protocol)` triples. `bind` succeeds only when the requested address falls inside
it. Everything else is `EACCES`, with the refused address named in the log.

Follow the shape the service registry already uses. Runtime registration there is
an allowlist of five endpoints rather than a denylist of reserved names, and that
was the right correction. Do the same here: a guest is given what it may bind,
never a list of what it may not.

## Defaults

- A guest with no `NetBind` cannot bind at all. `bind` returns `EACCES`.
- Loopback and external are different resources and are named separately in the
  bind set. A guest allowed to bind `127.0.0.1:8080` is not thereby allowed to
  bind `0.0.0.0:8080`. Most pentest tooling only needs loopback, so that is the
  common grant and the cheap one.
- Port 0, meaning "pick one for me", allocates only from inside the bind set.
- `SO_REUSEADDR` and `SO_REUSEPORT` do not let a guest take a port another guest
  holds. Two guests never share a bound port.
- `listen` on an unbound socket is `EINVAL`, as on Linux. It is not an implicit
  bind.
- `accept4` returns a socket inheriting the listener's confinement. The accepted
  fd carries no authority the listener did not have.

## Out of scope, refused by name

Raw sockets, `AF_PACKET`, and anything that reaches the link layer. A guest that
can forge frames is not confined by anything above it. Refuse with `EPERM` and a
named reason in the log, not `ENOSYS`, so the refusal reads as a decision rather
than a gap.

## What to prove

1. A guest without `NetBind` cannot bind, for every address.
2. A guest with a bind set cannot bind outside it, including via port 0.
3. Two guests cannot hold the same port, with `SO_REUSEPORT` set on both.
4. An accepted socket carries no authority the listener lacked.
5. A refused bind is logged with the address it asked for.

Write 1 and 2 as theorems if the decision function is pure enough to extract.
The rest are guest-suite tests in the shape the existing ten use.

## Order

Do this after the network block is lifted and items 5 to 8 are closed, because a
listener with no package to run behind it proves nothing.
