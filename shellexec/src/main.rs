use anyhow::{Context, Result};
use std::{fs, mem, path::PathBuf};
use virtual_memory::VirtualMemory;

#[derive(argh::FromArgs)]
/// Executes shellcode
struct Args {
    /// path to binary file
    #[argh(positional)]
    binary: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let buf = fs::read(args.binary).context("failed to read file")?;
    let len = buf.len();

    let mut memory = VirtualMemory::new(len).context("failed to allocate rwx memory")?;
    memory.copy_from_slice(&buf);

    let f: extern "C" fn() = unsafe { mem::transmute(memory.as_ptr()) };
    f();

    Ok(())
}
