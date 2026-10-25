# The attestation surface page: moved

This page checked v3 (`NZKSTRK1`) trailers, which the NONOS gates no longer
accept: they check v4 trailers with `nox_verify`. It is replaced by the proof
verifier's page, `nonos-proof-verify/web`, which runs `nonos-proof-verify`
itself compiled to WebAssembly, so the page, the command line and the gates
check a trailer the same way. Build it with `nix build .#proof-verify-web`, or
see `nonos-proof-verify/README.md`.
