use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "wasm")]
use wasmer::ValueType;

/// Allows for getting a slice of u8 out of wasm memory
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WASMString {
    pub ptr: u32, // *const u8
    pub len: u32,
}

#[cfg(feature = "wasm")]
unsafe impl ValueType for WASMString {}

/// C-Compatible representation of a system stage
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SystemStage {
    Pre,
    Tick,
    SendPackets,
    CleanUp,
}

#[cfg(feature = "wasm")]
unsafe impl ValueType for SystemStage {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HostSystem {
    pub stage: SystemStage,
    pub len: u32,
    pub name: *const [u8],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WASMSystem {
    pub stage: SystemStage,
    pub len: u32,
    pub name: u32, // *const [u8]
}

#[cfg(feature = "wasm")]
unsafe impl ValueType for WASMSystem {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HostSystems {
    pub len: u32,
    pub systems: *const [HostSystem],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WASMSystems {
    pub len: u32,
    pub systems: u32, // *const [WASMSystem]
}

#[cfg(feature = "wasm")]
unsafe impl ValueType for WASMSystems {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HostPluginRegister {
    pub name: FFIString,
    pub version: FFIString,
    pub systems: HostSystems,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WASMPluginRegister {
    pub name: WASMString,
    pub version: WASMString,
    pub systems: WASMSystems,
}

#[cfg(feature = "wasm")]
unsafe impl ValueType for WASMPluginRegister {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FFIString {
    pub ptr: *const str,
    pub len: usize,
}

impl From<String> for FFIString {
    fn from(string: String) -> Self {
        let as_str_boxed = string.into_boxed_str();

        Self {
            len: as_str_boxed.len(),
            ptr: Box::into_raw(as_str_boxed),
        }
    }
}

impl From<&str> for FFIString {
    fn from(str: &str) -> Self {
        let as_str_boxed: Box<str> = str.into();

        Self {
            len: as_str_boxed.len(),
            ptr: Box::into_raw(as_str_boxed),
        }
    }
}

#[cfg(feature = "wasm")]
pub type WasmOwned<T> = ManuallyDrop<T>;

#[cfg(not(feature = "wasm"))]
pub type WasmOwned<T> = T;

#[cfg(feature = "wasm")]
pub type SendHost<T> = ManuallyDrop<T>;

#[cfg(not(feature = "wasm"))]
pub type SendHost<T> = T;
