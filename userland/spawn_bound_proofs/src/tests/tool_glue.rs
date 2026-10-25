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
//! personality's two names, and every embedded tool.

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
    let tool = at("super::bound::check(tool.manifest,tool.caps,name)?;");
    assert!(tool < at("tool.spawn_with_args(argv)"));
    assert_eq!(code.matches("bound::check(").count(), 3);
}
