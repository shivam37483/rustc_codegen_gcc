// Compiler:
//
// Run-time:
//   status: 1

#![feature(no_core, start)]
#![no_std]
#![no_core]

extern crate mini_core;
use mini_core::*;

#[start]
fn main(mut argc: isize, _argv: *const *const u8) -> isize {
    1
}
