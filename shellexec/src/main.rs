use virtual_memory::*;

#[derive(argh::FromArgs)]
/// Executes shellcode
struct Args {
    /// path to binary file
    #[argh(positional)]
    binary: std::path::PathBuf,
}

fn main() {
    let args: Args = argh::from_env();

    let buf = std::fs::read(args.binary).expect("failed to read file");
    let len = buf.len();

    let mut memory = VirtualMemory::new(len).expect("failed to allocate rwx memory");
    memory.copy_from_slice(&buf);

    let f: extern "C" fn() = unsafe { std::mem::transmute(memory.as_ptr()) };
    f();
}
