// SPDX-FileCopyrightText: 2024 Ledger SAS
//
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate shield;
use sentry_uapi::systypes::Status;
use sentry_uapi::*;
use shield::println;

#[cfg(target_os = "none")]
shield::shield_main!();

fn main() {
    println!("Hello, World !");
    match sentry_uapi::syscall::get_shm_handle(0xF00 as u32) {
        Status::Ok => {
            let mut handle :&mut [u8] = &mut [0];
            copy_from_kernel(&mut handle);
            //handle.set_persistent();
            println!("Got shm handle");
        }
        _ => todo!(),
    }
}
