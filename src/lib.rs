#![feature(const_fn)]

extern "C" {
    fn print(ptr: *const u8, len: usize);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FFIString {
    pub ptr: *const u8,
    pub len: usize,
}

#[repr(C)]
pub struct FFISystem {
    stage: FFISystemStage,
    len: u32,
    name: *const [u8]
}

#[repr(C)]
pub struct FFISystems {
    len: u32,
    systems: *const [FFISystem]
}

#[repr(C)]
pub struct PluginState {
    name: FFIString,
    version: FFIString,
    systems: FFISystems
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FFISystemStage {
    Pre,
    Tick,
    SendPackets,
    CleanUp
}

#[no_mangle]
extern "C" fn __quill_free(ptr: *mut u8) {
    unsafe { drop(Box::from_raw(ptr)) };
}

static PLUGIN_NAME: &'static str = "Testing Plugin";
static PLUGIN_VERSION: &'static str = "1.0.0";

#[no_mangle]
pub extern "C" fn __quill_setup() -> *const PluginState {
    let hello = "Hello from a Plugin!";
    unsafe {
        print(hello.as_ptr(), hello.len());
    }

    Box::into_raw(Box::new(PluginState {
        name: FFIString {
            ptr: PLUGIN_NAME.as_ptr(),
            len: PLUGIN_NAME.len()
        },
        version: FFIString {
            ptr: PLUGIN_VERSION.as_ptr(),
            len: PLUGIN_VERSION.len()
        },
        systems: FFISystems {
            len: 1,
            systems: &[
                FFISystem {
                    stage: FFISystemStage::Tick,
                    len: "test_system".len() as u32,
                    name: "test_system".as_bytes()
                }
            ]
        }
    }))
}

#[no_mangle]
pub extern "C" fn test_system() {
    let to_print = "Plugin just ticked, awesome!";

    unsafe { print(to_print.as_ptr(), to_print.len()) }
}
