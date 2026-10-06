# nonos.toml, read and checked, turned into what one build needs: the kernel's
# cargo features, the bootloader's policy and the store's contents.
#
# A profile is a kind of image, and it sets floors, not defaults: a nonos.toml
# that asks a profile for less than its floor fails to evaluate. The kernel
# features are derived from Cargo.toml, never listed by hand, so a profile can
# name nothing the kernel does not have, and what a profile takes out is taken
# out of the binary, not switched off at run time.
#
# The boot menu's Standard, Hardened, Safe, Air-Gapped and Recovery choices are
# a different thing: a run-time posture any image offers. The build profiles
# decide what the image can ever do; the boot menu narrows it further.
{ lib, root }:
let
  kernelFeatures = (builtins.fromTOML (builtins.readFile (root + "/Cargo.toml"))).features;

  # Every feature a list of features turns on, by Cargo.toml.
  closure =
    fs:
    let
      step = acc: f:
        if builtins.elem f acc || !(kernelFeatures ? ${f}) then acc
        else builtins.foldl' step (acc ++ [ f ]) kernelFeatures.${f};
    in
    builtins.foldl' step [ ] fs;

  # A feature list with some features taken out of everything it turns on: a
  # feature that would bring one in is replaced by its own members, recursively,
  # and the unwanted ones are dropped.
  without =
    drop: fs:
    lib.concatMap (
      f:
      if builtins.elem f drop then [ ]
      else if lib.any (d: builtins.elem d (closure [ f ])) drop then without drop kernelFeatures.${f}
      else [ f ]
    ) fs;

  # Features the kernel's code tests by name (cfg(feature = "...")), so they
  # can only be on as themselves, never replaced by their members.
  markers = [ "microkernel-setup-wizard" "microkernel-input-probe" ];

  # First-boot setup is the only thing that ever records the choice to keep
  # anything (policy field Persistent), and with the installer it is what lets
  # an image write itself to a disk.
  installFeatures = [ "microkernel-setup-wizard" "nonos-capsule-setup-wizard" "nonos-capsule-install" "nonos-capsule-install-cli" ];

  # Everything that reaches a network: the drivers, the stack, and the
  # programs whose only job is to go online.
  networkFeatures = [
    "nonos-capsule-driver-virtio-net" "nonos-capsule-driver-e1000" "nonos-capsule-driver-rtl8139"
    "nonos-capsule-driver-rtl8169" "nonos-capsule-driver-iwlwifi" "nonos-capsule-driver-rtl8821ce"
    "nonos-capsule-net-l2" "nonos-capsule-net-ip" "nonos-capsule-net-udp" "nonos-capsule-net-dhcp"
    "nonos-capsule-net-tcp" "nonos-capsule-net-dns" "nonos-capsule-net-ntp" "nonos-capsule-net-core"
    "nonos-capsule-net-sockets" "nonos-capsule-net-nym" "nonos-capsule-net-anon" "nonos-capsule-socks5"
    "nonos-capsule-browser" "nonos-capsule-app-store" "nonos-capsule-market" "nonos-capsule-installer"
    "nonos-capsule-model-fetch" "nonos-capsule-shield"
  ];

  # Lets service capsules write to the serial console, which a hardened image
  # must not (Cargo.toml says why).
  debugFeatures = [ "capsule-serial-debug" ];

  # The bootloader's policies, weakest first. Development compiles in the F12
  # override and is never sealed for release.
  loaders = [ "dev-qemu" "standard-qemu" "standard" "production" ];
  level = l: lib.lists.findFirstIndex (x: x == l) (throw "nonos.toml: loader \"${l}\" is not one of ${toString loaders}") loaders;

  profiles = {
    standard = {
      about = "the full system for real hardware: every driver, the desktop, first-boot setup and the installer";
      for = "a person's own machine, every day";
      privacy = "amnesic until the person chooses to install; only a program granted the network capability can open a socket, and the Nym mixnet client is there for those that use it; capsules may write to the serial console for diagnosis";
      kernel = [ "microkernel-full-gui" ];
      loader = "standard";
      drop = [ ];
    };
    hardened = {
      about = "the full system with Secure Boot and a TPM required at boot and no serial console for capsules";
      for = "a machine that may be seized or tampered with";
      privacy = "standard, and the loader refuses to start without Secure Boot and a TPM to measure into; no capsule can write to a console anyone could read";
      kernel = [ "microkernel-full-gui" ];
      loader = "production";
      drop = debugFeatures;
    };
    airgapped = {
      about = "hardened, with no network driver, stack or online program compiled in; installs only from offline media";
      for = "keys and documents that must never touch a network";
      privacy = "hardened, and no code that could reach a network exists in the image: no driver, no stack, no program that goes online";
      kernel = [ "microkernel-full-gui" ];
      loader = "production";
      drop = debugFeatures ++ networkFeatures;
    };
    qemu = {
      about = "the desktop for virtual machines, without the drivers only real hardware has";
      for = "trying NONOS, and CI's boot smoke";
      privacy = "the standard posture, inside a virtual machine whose host sees everything";
      kernel = [ "microkernel-desktop-gui" "microkernel-setup-wizard" ];
      loader = "standard-qemu";
      drop = [ ];
    };
    dev = {
      about = "the qemu image with the loader's development policy and path-only attestation; the seal refuses it for release";
      for = "working on NONOS itself";
      privacy = "none promised: the loader's development override is compiled in, and the gates admit capsules on their path alone, without the STARK proof";
      kernel = [ "microkernel-desktop-gui" "microkernel-setup-wizard" "nonos-dev-attest" ];
      loader = "dev-qemu";
      drop = [ ];
    };
    core = {
      about = "the microkernel and its base capsules, no desktop";
      for = "kernel work and the smallest image that boots";
      privacy = "amnesic; no network and no desktop, so nothing to install or keep";
      kernel = [ "microkernel-capsules" ];
      loader = "standard-qemu";
      drop = [ ];
    };
  };

  defaults = {
    profile = "standard";
    # A development twin of the profile: its kernel features unchanged, with
    # path-only attestation and the loader's development policy, so a test
    # image is enrolled in seconds. Named <profile>-dev and never sealed for
    # release.
    dev = false;
    smp = true;
    install = true;
    rollback_index = 1;
    features = [ ];
    # The NONOS package mirror the in-tree tools install from; empty, none.
    linux_packages = "";
    store = { linux = true; media = true; };
  };

  resolve =
    raw:
    let
      c = lib.recursiveUpdate defaults raw;
      unknown = lib.subtractLists (builtins.attrNames defaults ++ [ "loader" ]) (builtins.attrNames raw);
      p = profiles.${c.profile} or (throw "nonos.toml: profile \"${c.profile}\" is not one of ${toString (builtins.attrNames profiles)}");
      loader = if c.dev then "dev-qemu" else raw.loader or p.loader;
      drop = p.drop ++ lib.optionals (!c.install) installFeatures;
      features = without drop (p.kernel ++ lib.optional c.smp "nonos-smp" ++ c.features
        ++ lib.optional (c.dev && !(builtins.elem "nonos-dev-attest" p.kernel)) "nonos-dev-attest"
        ++ lib.optional (loader != "dev-qemu") "nonos-release");
      enabled = closure features;
      missing = builtins.filter (f: !(kernelFeatures ? ${f})) features;
      leaked = builtins.filter (f: builtins.elem f enabled) drop;
      base = p.kernel ++ lib.optional c.smp "nonos-smp" ++ c.features;
      # A marker the profile needs, whose members bring in something the
      # profile takes out, cannot be kept: the kernel's Cargo.toml has to stop
      # tying the two together first.
      tangled = builtins.filter (m: !(builtins.elem m drop) && builtins.elem m (closure base)
        && lib.any (d: builtins.elem d (closure [ m ])) drop) markers;
    in
    assert lib.assertMsg (unknown == [ ]) "nonos.toml: unknown keys ${toString unknown}";
    assert lib.assertMsg (missing == [ ]) "nonos.toml: Cargo.toml has no feature ${toString missing}";
    assert lib.assertMsg (leaked == [ ]) "nonos.toml: profile ${c.profile} cannot carry ${toString leaked}";
    assert lib.assertMsg (c.dev || level loader >= level p.loader) "nonos.toml: profile ${c.profile} needs at least loader ${p.loader}, not ${loader}";
    assert lib.assertMsg (builtins.isBool c.dev) "nonos.toml: dev is true or false";
    assert lib.assertMsg (builtins.isInt c.rollback_index && c.rollback_index >= 1) "nonos.toml: rollback_index is an integer from 1";
    assert lib.assertMsg (builtins.isString c.linux_packages && builtins.match "[A-Za-z0-9.-]*(:[0-9]+)?" c.linux_packages != null)
      "nonos.toml: linux_packages is name:port, or empty";
    # Path-only attestation belongs to the development loader, whose loader
    # takes the same path-only kernel trailer; no other image may carry it.
    assert lib.assertMsg (!(builtins.elem "nonos-dev-attest" enabled) || loader == "dev-qemu")
      "nonos.toml: nonos-dev-attest admits capsules without their STARK proof and needs loader dev-qemu";
    # Test 5 proves the pinned wallet vectors at boot: a development test,
    # never in an image any other loader boots.
    assert lib.assertMsg (!(builtins.elem "nonos-capsule-shield-vectors" enabled) || loader == "dev-qemu")
      "nonos.toml: nonos-capsule-shield-vectors is Test 5, for development images only (dev = true)";
    c // {
      inherit features enabled loader;
      # What the profile takes out, closed over Cargo.toml: none of it may be
      # in the kernel, and the receipt checks the kernel's bytes for it.
      taken_out = lib.unique (lib.concatMap (d: closure [ d ]) drop);
      blocked =
        if tangled == [ ] then null
        else "profile ${c.profile} takes out ${toString (builtins.filter (d: lib.any (m: builtins.elem d (closure [ m ])) tangled) drop)}, "
          + "and Cargo.toml's ${toString tangled} brings it back in; until the kernel's features separate the two, "
          + "this profile builds only with install = false";
      inherit (p) about for privacy;
      name = c.profile + lib.optionalString (c.dev && c.profile != "dev") "-dev" + lib.optionalString (!c.install) "-live";
      release = loader != "dev-qemu";
      network = lib.any (f: builtins.elem f enabled) networkFeatures;
      installer = builtins.all (f: builtins.elem f enabled) [ "microkernel-setup-wizard" "nonos-capsule-install" ];
    };
in
{
  inherit profiles resolve closure;

  # What `make profiles` prints: each profile, what it is for, and what it
  # promises about privacy, from the definitions above.
  describe = lib.concatStrings (lib.mapAttrsToList (n: p: ''
    ${n}
      ${p.about}
      for:     ${p.for}
      privacy: ${p.privacy}
      loader:  ${p.loader}

  '') profiles);
  file = builtins.fromTOML (builtins.readFile (root + "/nonos.toml"));
}
