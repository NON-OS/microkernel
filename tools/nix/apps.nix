# The flake's programs. Each runs on the host, outside the sandbox, with the
# flake's tools first on PATH, so `nix run` behaves the same on every machine.
#
#   nix run .#seal   ek's step: enroll, sign and pack the artifacts into a
#                    bootable image, with ek's keys (tools/nonos-seal)
#   nix run .#qemu   boot a sealed image under QEMU (tools/nonos-qemu)
#   nix run .#receipt, .#check-report   what make build and make check print
{ pkgs, lib, hostTools, shell }:
let
  tools = shell.tools ++ [ hostTools pkgs.nix ];
  env = ''
    export PATH=${lib.makeBinPath tools}:$PATH
    export NONOS_IN_FLAKE=1 NONOS_PYTHON=python3 NONOS_HOST_TOOLS=${hostTools}
    export OVMF=${shell.OVMF} OVMF_VARS=${shell.OVMF_VARS}
    root=$(git rev-parse --show-toplevel 2>/dev/null) || { echo "run from inside the NONOS checkout" >&2; exit 1; }
    cd "$root"
  '';
  # The two reports need only Python and git, not the whole shell.
  report = name: script: {
    type = "app";
    program = toString (pkgs.writeShellScript "nonos-${name}" ''
      # nix itself is the one that ran this, already on PATH.
      export PATH=${lib.makeBinPath [ pkgs.python3 pkgs.git pkgs.coreutils ]}:$PATH
      root=$(git rev-parse --show-toplevel 2>/dev/null) || { echo "run from inside the NONOS checkout" >&2; exit 1; }
      cd "$root"
      exec ${script} "$@"
    '');
    meta.description = "NONOS ${name}";
  };
  app = name: script: {
    type = "app";
    program = toString (pkgs.writeShellScript "nonos-${name}" ''
      ${env}
      exec ${script} "$@"
    '');
    meta.description = "NONOS ${name}";
  };
in
{
  seal = app "seal" "python3 tools/nonos-seal";
  # Booting needs QEMU, the software TPM and the firmware, never the seal's
  # host tools, so a change to those does not hold up a boot.
  qemu = {
    type = "app";
    program = toString (pkgs.writeShellScript "nonos-qemu" ''
      export PATH=${lib.makeBinPath (shell.tools ++ [ pkgs.nix ])}:$PATH
      export OVMF=${shell.OVMF} OVMF_VARS=${shell.OVMF_VARS}
      root=$(git rev-parse --show-toplevel 2>/dev/null) || { echo "run from inside the NONOS checkout" >&2; exit 1; }
      cd "$root"
      exec python3 tools/nonos-qemu "$@"
    '');
    meta.description = "NONOS qemu";
  };
  # make build's receipt and make check's report (tools/nonos-receipt,
  # tools/nonos-check-report): the build saying what it proved.
  receipt = report "receipt" "python3 tools/nonos-receipt";
  check-report = report "check-report" "python3 tools/nonos-check-report";
}
