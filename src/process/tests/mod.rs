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

pub mod address_space_types_tests;
pub mod core_types_tests;
pub mod fd_types_tests;
mod runners;
pub mod scheduler_types_tests;
pub mod thread_group_tests;
pub mod userspace_types_tests;

use crate::test::framework::TestSuite;

pub fn run_all() -> bool {
    let mut suite = TestSuite::new("process");
    runners::thread_group::register(&mut suite);
    runners::core_types::register(&mut suite);
    runners::address_space::register(&mut suite);
    runners::fd_types::register(&mut suite);
    runners::scheduler::register(&mut suite);
    runners::userspace::register(&mut suite);
    suite.run()
}
