use ndarray::prelude::*;
use std::collections::VecDeque;
use argmax::Argmax;
use std::slice;


#[cxx::bridge]
mod ffi {
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        unsafe fn auction_ffi(problem: *const f64, m: usize, n: usize, eps: f64, max_iterations: usize, assignment: *mut i32);
    }
}

unsafe fn auction_ffi(problem_ptr: *const f64, m: usize, n: usize, eps: f64, max_iterations: usize, assignment_ptr: *mut i32) {
    let problem = ArrayView::from_shape_ptr((m, n), problem_ptr);
    let assignment = auction(&problem, eps, max_iterations);
    let assignment_slice = slice::from_raw_parts_mut(assignment_ptr, n);
    for (k, a) in assignment.iter().enumerate() {
        assignment_slice[k] = match *a {
            Assignment::Assigned(a) => a as i32,
            Assignment::Unassigned => -1,
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Assignment {
    Assigned(usize),
    Unassigned,
}

impl PartialEq<usize> for Assignment {
    fn eq(&self, other: &usize) -> bool {
        let this = match self {
            Self::Assigned(a) => a,
            Self::Unassigned => return false,
        };
        this == other
    }
}

pub fn auction(problem: &ArrayView2<f64>, eps: f64, max_iterations: usize) -> Vec<Assignment> {
    use Assignment::{Assigned, Unassigned};
    
    let (m, n) = problem.dim();
    let mut unassigned_queue: VecDeque<_> = (0..n).collect();
    let mut assigned_tracks: Vec<Assignment> = vec![Unassigned; n];
    let mut prices = vec![0f64; m];

    let mut curr_iter = 0;
    
    while let Some(t_star) = unassigned_queue.pop_front() {
        if curr_iter > max_iterations {
            break;
        }
        let (i_star, val_max) = problem.column(t_star)
            .into_iter()
            .zip(prices.iter())
            .map(|(reward, &price)| reward - price).argmax().unwrap();
        let prev_owner = assigned_tracks.iter().position(|&e| e == i_star);
        assigned_tracks[t_star] = Assigned(i_star);
        
        if let Some(prev_owner) = prev_owner {
            // The item has a previous owner
            assigned_tracks[prev_owner] = Unassigned;
            unassigned_queue.push_back(prev_owner);
        }
        
        let y = problem[(i_star, t_star)] - val_max;
        prices[i_star] += y + eps;
        curr_iter += 1;
    }

    assigned_tracks
}