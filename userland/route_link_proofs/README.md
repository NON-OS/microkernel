# route_link_proofs

Host proofs for `nonos_route_link`, the client every capsule that follows the
chosen network leaves through. The shipped sources are included by `#[path]`.

- `pick_tests`: the route for every default and every network up or down;
  nothing falls back, and an unreadable default is never Direct.
- `direct_tests` and `ntp_tests`: the rule for contacts that can only leave
  directly, and `net.ntp`'s decision (`capsule_net_ntp/src/decide.rs`) built
  on it.
- `sdk_way_tests`: the SDK's rule for which way a connection leaves.
- `frame_tests`, `answer_tests` and `socks_tests`: the frames sent to
  `net.socks5` and `net.anon`, the reading of their answers, and the SOCKS5 a
  client speaks.
- `tunnel_open_tests`, `tunnel_read_tests` and `tunnel_fault_tests`: the
  tunnel itself, against a proxy written here (`src/fake.rs`) to lose, delay,
  split and garble its answers on purpose.

Run: `cd userland/route_link_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-route_link_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the SOCKS5 bridge and network routes](../../docs/handbook/network/socks5-and-routes.md) and
[proofs](../../docs/handbook/verification/proofs.md).
