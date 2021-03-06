// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass

const A: [u8; 1] = ['h' as u8];
const B: u8 = (&A)[0];
const C: &'static &'static &'static &'static [u8; 1] = & & & &A;
const D: u8 = (&C)[0];

pub fn main() {
    assert_eq!(B, A[0]);
    assert_eq!(D, A[0]);
}
