#![no_std]
#![allow(dead_code)]

mod fisher;
mod splitly;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
