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

//! drawing.rs pinned whole: its imports, so `stopped` is held.rs's own; the
//! guard, which only `drawable()` makes and only its drop uncounts; and the
//! steps the model's drawer takes, in its order.

use super::text::code;
use super::text_items::flat;

#[test]
fn drawing_rs_is_the_models_drawer_and_nothing_else() {
    let want = [
        "usecore::ops::Deref;usecore::sync::atomic::{AtomicU32,Ordering::SeqCst};",
        "usesuper::held::stopped;usesuper::state::{framebuffer_state,KernelFramebuffer};",
        "staticDRAWS:AtomicU32=AtomicU32::new(0);",
        "pub(crate)structDrawing(&'staticKernelFramebuffer);",
        "implDerefforDrawing{typeTarget=KernelFramebuffer;",
        "fnderef(&self)->&KernelFramebuffer{self.0}}",
        "implDropforDrawing{fndrop(&mutself){",
        "core::sync::atomic::fence(SeqCst);DRAWS.fetch_sub(1,SeqCst);}}",
        "pub(crate)fndrawable()->Option<Drawing>{ifstopped(){returnNone;}",
        "DRAWS.fetch_add(1,SeqCst);letfb=framebuffer_state().filter(|_|!stopped());",
        "iffb.is_none(){DRAWS.fetch_sub(1,SeqCst);}fb.map(Drawing)}",
        "pub(crate)fnwait_quiet(){whileDRAWS.load(SeqCst)!=0{",
        "crate::smp::serve_shootdowns();core::hint::spin_loop();}}",
    ];
    assert_eq!(flat(&code("kernel_core/init/framebuffer/drawing.rs")), want.concat());
}
