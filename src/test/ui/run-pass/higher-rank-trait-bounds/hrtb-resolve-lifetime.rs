// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// A basic test of using a higher-ranked trait bound.

// pretty-expanded FIXME #23616

trait FnLike<A,R> {
    fn call(&self, arg: A) -> R;
}

type FnObject<'b> = for<'a> FnLike<&'a isize, &'a isize> + 'b;

fn main() {
}