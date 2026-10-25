// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The run path asks the tool bound before each spawn it can make: the Linux
//! personality's two names, and every embedded tool. Only init's self-test
//! runs a tool as the kernel, outside any caller's bound.

#[test]
fn the_run_path_checks_before_every_spawn() {
    let src = include_str!("../../../../src/userspace/tool_capsules/run.rs");
    let code: String = src.lines().map(|l| l.split("//").next().unwrap()).collect();
    let code: String = code.split_whitespace().collect();
    let at = |s: &str| code.find(s).unwrap_or_else(|| panic!("run.rs lost `{s}`"));
    let linux = at("super::bound::check(LINUX_MANIFEST_BYTES,LINUX_CAPS,name)?;");
    assert!(linux < at("super::linux_terminal::run(argv)"));
    assert!(linux < at("run_qwen_for_caller(argv)"));
    assert!(at("ifname==LINUX_TOOL||name==QWEN_TOOL{") < linux);
    let tool = at("lettool=embedded(name)?;super::bound::check(tool.manifest,tool.caps,name)?;spawn(&tool,argv)");
    assert!(tool < at("pubfnrun_embedded_for_kernel("), "run_named checks before its spawn");
    assert_eq!(code.matches("bound::check(").count(), 3);
}

/// The one unbounded run is the kernel's own, for init's boot self-test:
/// built only with that feature, called only from it, never by MkToolRun.
#[test]
fn only_init_s_self_test_takes_the_kernel_run() {
    let run = include_str!("../../../../src/userspace/tool_capsules/run.rs");
    let gated = "#[cfg(feature = \"nonos-tool-selftest\")]\npub fn run_embedded_for_kernel(";
    assert!(run.contains(gated), "the kernel run exists only in a self-test build");
    let syscall = include_str!("../../../../src/syscall/microkernel/tool_run.rs");
    assert!(syscall.contains("tool_capsules::run_named(") && !syscall.contains("for_kernel"));
    let init = include_str!("../../../../src/userspace/init/entry.rs");
    let selftest = init.split("#[cfg(feature = \"nonos-tool-selftest\")]\nfn run_tool_selftest()").nth(1);
    let body = selftest.expect("init's self-test").split("\n}\n").next().unwrap();
    assert!(body.contains("run_embedded_for_kernel(service, argv)"));
    assert_eq!(init.matches("run_embedded_for_kernel").count(), 1, "init calls it there only");
}
