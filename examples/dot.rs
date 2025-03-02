// https://gitlab.com/graphviz/graphviz/-/blob/main/dot.demo/dot.c

use std::{convert::TryInto, ffi::CString, ptr::NonNull};

use graphviz_sys::{
    agclose, gvContext, gvFreeLayout, gvLayoutJobs, gvNextInputGraph, gvParseArgs, gvRenderJobs,
};

fn main() {
    unsafe {
        let gvc = gvContext();
        // Be aware that graphviz uses argv[0] to select the layout engine,
        // so this will only work in a binary named `dot` (or another valid engine name)
        let mut args = std::env::args()
            .map(|arg| CString::new(arg).unwrap().into_raw())
            .collect::<Vec<_>>();
        gvParseArgs(gvc, args.len().try_into().unwrap(), args.as_mut_ptr());
        let mut prev: Option<NonNull<_>> = None;
        while let Some(g) = NonNull::new(gvNextInputGraph(gvc)) {
            if let Some(p) = prev {
                gvFreeLayout(gvc, p.as_ptr());
                agclose(p.as_ptr());
            }
            gvLayoutJobs(gvc, g.as_ptr());
            gvRenderJobs(gvc, g.as_ptr());
            prev = Some(g);
        }

        for ptr in args {
            drop(CString::from_raw(ptr));
        }
    }
}
