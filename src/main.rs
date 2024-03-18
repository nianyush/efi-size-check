#![no_main]
#![no_std]

extern crate alloc;
extern crate uefi;
extern crate uefi_services;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use log::info;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileMode};
use uefi::CStr16;

#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();
    info!("Starting EFI size checker...");

    let bs = st.boot_services();

    let mut sfs = bs.get_image_file_system(image).unwrap();
    let mut root = sfs.open_volume().unwrap();

    let mut buf = [0; 4];
    let file_name: &CStr16 = CStr16::from_str_with_buf("ABC", &mut buf).unwrap(); // Ensure this file exists on your ESP with a large size for testing
    let mut buffer_size = 1024 * 1024 * 10; // Start with 1 MiB
    let increment = 1024 * 1024 * 10; // Increase buffer size by 1 MiB in each iteration

    info!("start to check...");

    loop {
        let mut file = match root.open(
            file_name,
            FileMode::Read,
            uefi::proto::media::file::FileAttribute::empty(),
        ) {
            Ok(f) => f.into_regular_file().unwrap(),
            Err(_) => break,
        };

        info!("Reading {} bytes into buffer", format_bytes(buffer_size));

        let mut buffer = Vec::with_capacity(buffer_size);
        unsafe {
            buffer.set_len(buffer_size);
        } // Unsafe due to uninitialized memory

        match file.read(&mut buffer) {
            Ok(_) => {
                info!(
                    "Successfully read {} into buffer",
                    format_bytes(buffer_size)
                );
                buffer_size += increment;
            }
            Err(e) => {
                info!("Failed to read into a {} byte buffer: {:?}", buffer_size, e);
                break;
            }
        }
    }

    st.boot_services().stall(100_000_000);
    Status::SUCCESS
}

fn format_bytes(size: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}
