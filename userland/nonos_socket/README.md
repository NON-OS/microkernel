# nonos_socket

`nonos_socket` is the client side of `net.sockets` for a `no_std` capsule: a
TCP stream to a host name, driven by request and reply over IPC. It touches no
device and no packet. `nonos_route_link`, `capsule_model_fetch` and
`capsule_terminal` use it for the Direct network.

## What it offers

- `TcpStream::connect(host, port)` looks up `net.sockets`, opens an IPv4 stream
  socket (`OP_SOCKET`), and asks the service to resolve and connect the name
  (`OP_CONNECT_HOST`). A failed connect closes the handle before it returns.
- `write_all` sends in as many `OP_SEND` calls as the data needs. `read`
  returns what has arrived; zero means nothing was ready within the 200 ms
  receive timeout, not that the peer closed.
- Dropping a `TcpStream` sends `OP_CLOSE`, so a handle cannot outlive the code
  that opened it.
- `lookup(name)` returns a service's port, or zero when it is not registered.
- The free functions `open`, `connect_host`, `send`, `recv` and `close` are the
  same calls on a raw handle.

Every reply is checked before it is believed: a frame shorter than the 20 byte
header, a wrong magic (`0x4E534B54`) or a non-zero status is an error, and a
receive copies the least of what the header claims, what arrived and what the
caller has room for. Errors are `SocketError`: `NoService`, `Protocol`,
`Refused`, `BadHost` (empty or longer than 253 bytes) and `TooLarge` (a body
past one 1536 byte frame).

Timeouts: 2 s for most calls, 9 s for a connect, 200 ms for a receive.

## What it does not do

- No datagrams, listeners or mixnet sockets: only `OP_SOCKET` with kind 1.
- No route: it always goes through `net.sockets`, which is a direct
  connection. A caller that follows the chosen network uses
  `nonos_route_link` instead, which calls this crate only for Direct.
- It resolves nothing itself; `net.sockets` asks `net.dns`.

## Capabilities

A library adds no capability of its own. The capsule using it needs IPC to
make the calls and Network, which the kernel requires of every sender to
`net.sockets`.

See [the network stack](../../docs/handbook/network/stack.md).
