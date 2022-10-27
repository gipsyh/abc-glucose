mod bindings;

use bindings::{
    bmcg2_sat_solver_addvar, bmcg2_sat_solver_mark_cone, bmcg2_sat_solver_read_cex,
    bmcg2_sat_solver_set_jftr, bmcg2_sat_solver_set_var_fanin_lit, bmcg2_sat_solver_solve,
    bmcg2_sat_solver_start, bmcg2_sat_solver_start_new_round, bmcg2_sat_solver_stop,
};
use std::{ffi::c_void, ptr::NonNull, slice::from_raw_parts};

pub struct Solver {
    ptr: NonNull<c_void>,
}

impl Solver {
    pub fn new() -> Self {
        let ptr = NonNull::new(unsafe { bmcg2_sat_solver_start() }).unwrap();
        unsafe { bmcg2_sat_solver_set_jftr(ptr.as_ptr(), 2) }
        Self { ptr }
    }

    pub fn add_var(&mut self) -> Var {
        Var(unsafe { bmcg2_sat_solver_addvar(self.ptr.as_mut()) })
    }

    pub fn set_fanin(&mut self, var: Var, fanin0: Lit, fanin1: Lit) {
        debug_assert!(fanin0.0 < fanin1.0);
        unsafe { bmcg2_sat_solver_set_var_fanin_lit(self.ptr.as_mut(), var.0, fanin0.0, fanin1.0) }
    }

    pub fn new_round(&mut self) {
        unsafe { bmcg2_sat_solver_start_new_round(self.ptr.as_mut()) }
    }

    pub fn mark_cone(&mut self, var: Var) {
        unsafe { bmcg2_sat_solver_mark_cone(self.ptr.as_mut(), var.0) }
    }

    pub fn solve_under_assumptions(&mut self, assumptions: &[Lit]) -> Option<&[Lit]> {
        let ret = unsafe {
            bmcg2_sat_solver_solve(
                self.ptr.as_ptr(),
                assumptions.as_ptr() as _,
                assumptions.len() as _,
            )
        };
        if ret == 1 {
            let ret = unsafe { bmcg2_sat_solver_read_cex(self.ptr.as_ptr()) };
            Some(unsafe { from_raw_parts(ret.add(1) as *const Lit, *ret as usize) })
        } else {
            debug_assert!(ret == -1);
            None
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { bmcg2_sat_solver_stop(self.ptr.as_mut()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Var(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lit(i32);

impl From<Var> for Lit {
    fn from(value: Var) -> Self {
        Self(value.0 + value.0)
    }
}

impl Lit {
    pub fn new(var: Var, compl: bool) -> Self {
        Lit(var.0 + var.0 + compl as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lit, Solver};

    #[test]
    fn test() {
        let mut solver = Solver::new();
        let var0 = solver.add_var();
        let var1 = solver.add_var();
        let var2 = solver.add_var();
        solver.set_fanin(var2, var0.into(), var1.into());
        solver.new_round();
        solver.mark_cone(var2);
        let ret = solver.solve_under_assumptions(&[var2.into()]).unwrap();
        assert_eq!(ret[0], Lit(0));
        assert_eq!(ret[1], Lit(2));
    }
}
