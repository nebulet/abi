use super::handle::Handle;

extern {
    // generic handles
    pub fn handle_close(handle: Handle) -> u64;
    pub fn handle_duplicate(handle: Handle, rights: u32) -> u64;

    // processes
    // pub fn wasm_compile(ptr: *mut u8, size: usize) -> u64;
    // pub fn process_create(code_handle: Handle) -> u64;
    // pub fn process_start(proc_handle: Handle) -> u64;
}
