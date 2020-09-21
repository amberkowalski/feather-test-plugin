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
    pub len: usize
}

impl From<String> for FFIString {
    fn from(string: String) -> Self {
        let as_str_boxed = string.into_boxed_str();
        
        Self {
            len: as_str_boxed.len(),
            ptr: Box::into_raw(as_str_boxed)
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

/// Indicates that the contained value is owned by the Host
/// 
/// TODO: Might actually still require freeing
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct HostOwned<T>(pub T);

/// Indicates that the contained value is owned by a WASM module
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct WasmOwned<T>(pub T);

/// Indicates a transfer of ownership from the Host to WASM
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SendWasm<T>(pub T);

/// Indicates a transfer of ownership from WASM to the Host
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SendHost<T>(pub T);

/// Indicates the value is static and no special handling is required
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Static<T>(pub T);