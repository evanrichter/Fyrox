#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = fyrox::resource::fbx::fbx_from_bytes(data);
});
