// SPDX-FileCopyrightText: 2024 Ledger SAS
//
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

//extern crate shield;
//use sentry_uapi::systypes::Status;
//use sentry_uapi::*;
use shield::println;
use shield::process::get_process_handle;
use shield::Status;
use core::prelude::rust_2024::Ok;
//use shield::*;

#[cfg(target_os = "none")]
shield::shield_main!();

fn main() {
    println!("Hello, World !");
    match get_process_handle(0x5555 as u32) {
        Ok(handle) => {
            println!("Got task handle");
            //let mut handle: &mut [u8; 4] = &mut [0; 4];
            //TODO: handle bad status
            //let _ = copy_from_kernel(&mut handle[..]);
            //handle.set_persistent();
            //println!("Got shm handle: {:?}", handle);
        }
        _ => println!("No task handle"),
    }
}
