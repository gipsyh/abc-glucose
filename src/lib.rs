mod bindings;

use bindings::bmcg2_sat_solver_start;
use std::{ffi::c_void, ptr::NonNull};

pub struct Solver {
    ptr: NonNull<c_void>,
}

impl Solver {
    pub fn new() -> Self {
        let ptr = NonNull::new(unsafe { bmcg2_sat_solver_start() }).unwrap();
        Self { ptr }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    #[test]
    fn it_works() {
        let solver = Solver::new();
    }
}
