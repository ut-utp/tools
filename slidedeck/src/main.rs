
use lc3_shims::memory::FileBackedMemoryShim;

pub mod common;

mod ferris;
mod misc;

fn main() {
    FileBackedMemoryShim::with_initialized_memory("slides.mem", ferris::slide_deck())
        .flush()
        .map_err(|_| std::io::Error::last_os_error()).unwrap();
}
