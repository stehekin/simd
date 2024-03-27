#![feature(portable_simd)]

// source: https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html
use std::simd::{f32x4, Simd};

fn add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
  let size = xs.len() as isize;
  let chunks = size / 4;

  let p_x = xs.as_mut_ptr();
  let p_y = ys.as_ptr();

  // Chunks.
  let simd_p_x = p_x as *mut f32x4;
  let simd_p_y = p_y as *const f32x4;

  for i in 0 .. chunks {
    unsafe {
      *simd_p_x.offset(i) += *simd_p_y.offset(i);
    }
  }

  // Remaining elements.
  for i in (4 * chunks)..size {
    unsafe{
      *p_x.offset(i) += *p_y.offset(i);
    }
  }
}


fn main() {
    let mut x = vec![0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0];
    add_assign(&mut x, &y);
    println!("{:?}", x);
}