// SPDX-FileCopyrightText: 2024 Ledger SAS
//
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate shield;
use shield::println;
use sentry_uapi::*;

#[cfg(target_os = "none")]
shield::shield_main!();

fn main() {
    println!("Hello, World !");
    match sentry_uapi::syscall::get_shm_handle(0xF00 as u32) {
    Status::Ok => (),
    any_err => return (any_err),
}
}
