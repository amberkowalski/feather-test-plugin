#[cfg(feature = "host")]
pub use host::*;

#[cfg(feature = "host")]
use module::*;

#[cfg(not(feature = "host"))]
pub use module::*;

pub mod module {
    use std::ops::Deref;

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginBox<T: Copy>(pub T);

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginRef<T: Copy>(pub T);

    impl<T: Clone + Copy> Deref for PluginBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: Clone + Copy> Deref for PluginRef<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    /// A type that allows Strings to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginString {
        ptr: *const u8,
        len: usize,
    }

    impl PluginString {
        pub unsafe fn from_borrow(string: &str) -> Self {
            Self {
                ptr: string.as_ptr(),
                len: string.len(),
            }
        }
    }

    impl From<String> for PluginBox<PluginString> {
        fn from(string: String) -> Self {
            let as_str_boxed = Box::new(string.as_str().as_bytes());

            PluginBox(PluginString {
                len: as_str_boxed.len(),
                ptr: Box::into_raw(as_str_boxed) as *const u8,
            })
        }
    }

    impl From<&str> for PluginBox<PluginString> {
        fn from(str: &str) -> Self {
            let as_str_boxed: Box<[u8]> = Box::from(str.as_bytes());

            PluginBox(PluginString {
                len: as_str_boxed.len(),
                ptr: Box::into_raw(as_str_boxed) as *const u8,
            })
        }
    }

    /// A type that allows slices to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginSlice<T: Copy + Clone> {
        len: usize,
        elements: *const [T],
    }

    impl<T> From<&[T]> for PluginBox<PluginSlice<T>>
    where
        T: Clone + Copy,
    {
        fn from(from: &[T]) -> PluginBox<PluginSlice<T>> {
            let as_box: Box<[T]> = from.into();

            PluginBox(PluginSlice {
                len: as_box.len(),
                elements: Box::into_raw(as_box),
            })
        }
    }

        /// A type that allows slices to be sent over FFI.
        #[repr(C)]
        #[derive(Copy, Clone, Debug)]
        pub struct PluginSliceAlloc<T: Copy + Clone> {
            len: usize,
            elements: *const [T],
        }
    
        impl<T> From<&[T]> for PluginBox<PluginSliceAlloc<T>>
        where
            T: Clone + Copy,
        {
            fn from(from: &[T]) -> PluginBox<PluginSliceAlloc<T>> {
                let as_box: Box<[T]> = from.into();
    
                PluginBox(PluginSliceAlloc {
                    len: as_box.len(),
                    elements: Box::into_raw(as_box),
                })
            }
        }

    /// A type that allows system definitions to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginSystem {
        pub stage: crate::SystemStage,
        pub name: PluginBox<PluginString>,
    }

    /// A type that defines a plugin's properties.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginRegister {
        pub name: PluginBox<PluginString>,
        pub version: PluginBox<PluginString>,
        pub systems: PluginBox<PluginSliceAlloc<PluginSystem>>,
    }

    impl Into<*const PluginBox<PluginRegister>> for PluginRegister {
        fn into(self) -> *const PluginBox<PluginRegister> {
            let boxed_self = Box::from(PluginBox(self));

            Box::into_raw(boxed_self)
        }
    }
}

#[cfg(feature = "host")]
pub mod host {
    use std::marker::PhantomData;
    use wasmer::ValueType;
    use wasmer::{WasmPtr, NativeFunc, Memory, Array};
    use std::alloc::{dealloc, Layout};
    use std::ops::Deref;

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginBox<T: ValueType>(pub T);

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginRef<T: ValueType>(pub T);

    impl<T: ValueType> Deref for PluginBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    unsafe impl<T> ValueType for PluginBox<T> where T: ValueType {}

    impl<T: ValueType> Deref for PluginRef<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    unsafe impl<T> ValueType for PluginRef<T> where T: ValueType {}

    pub trait WasmFree: ValueType {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>);
    }

    /// A type that allows Strings to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginString {
        pub ptr: u32,
        pub len: u32,
    }

    impl PluginString {
        pub fn to_string(&self, memory: &Memory) -> Option<String> {
            let ptr: WasmPtr<u8, Array> = WasmPtr::new(self.ptr);

            Some(ptr.get_utf8_string(memory, self.len)?.to_owned())
        }
    }

    unsafe impl ValueType for PluginString {}

    impl WasmFree for PluginBox<PluginString> {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            let type_size = std::mem::size_of::<u8>() as u32;
            let type_align = std::mem::align_of::<PluginString>() as u32;

            // Adjust the layout to free the entire slice
            let free_size = type_size * self.len;

            free_func.call(WasmPtr::new(self.ptr), free_size as u32, type_align as u32).unwrap();

        }
    }

    /// A type that allows slices to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginSlice<T: ValueType> {
        pub len: u32,
        pub elements: u32, // *const [T]
        _marker: PhantomData<T>,
    }

    unsafe impl<T> ValueType for PluginSlice<T> where T: ValueType {}

    // Known Bug: things that are `WasmFree` wont get freed inside the FFISS

    impl<T> WasmFree for PluginBox<PluginSlice<T>> where T: ValueType {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            let type_size = std::mem::size_of::<T>() as u32;
            let type_align = std::mem::align_of::<T>() as u32;

            // Adjust the layout to free the entire slice
            let free_size = type_size * self.len;

            free_func.call(WasmPtr::new(self.elements), free_size as u32, type_align as u32).unwrap();
        }
    }

    /// A type that allows slices to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginSliceAlloc<T: ValueType + WasmFree> {
        pub len: u32,
        pub elements: u32, // *const [T]
        _marker: PhantomData<T>,
    }

    unsafe impl<T> ValueType for PluginSliceAlloc<T> where T: ValueType + WasmFree {}

    // Known Bug: things that are `WasmFree` wont get freed inside the FFISS

    impl<T> WasmFree for PluginBox<PluginSliceAlloc<T>> where T: ValueType + WasmFree {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            // Start by runnning WasmFree on the slice elements
            let slice = unsafe { std::slice::from_raw_parts(self.elements as *const T, self.len as usize) };

            for element in slice {
                element.free(&free_func);
            }

            let type_size = std::mem::size_of::<T>() as u32;
            let type_align = std::mem::align_of::<T>() as u32;

            // Adjust the layout to free the entire slice
            let free_size = type_size * self.len;

            free_func.call(WasmPtr::new(self.elements), free_size as u32, type_align as u32).unwrap();
        }
    }

    /// A type that allows system definitions to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginSystem {
        pub stage: crate::SystemStage,
        pub name: PluginBox<PluginString>,
    }

    unsafe impl ValueType for PluginSystem {}

    impl WasmFree for PluginSystem {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            self.name.free(free_func)
        }
    }

    /// A type that defines a plugin's properties.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PluginRegister {
        pub name: PluginBox<PluginString>,
        pub version: PluginBox<PluginString>,
        pub systems: PluginBox<PluginSliceAlloc<PluginSystem>>,
    }

    unsafe impl ValueType for PluginRegister {}

    impl WasmFree for PluginBox<PluginRegister> {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            self.name.free(free_func);
            self.version.free(free_func);
            self.systems.free(free_func);
        }
    }

    impl PluginBox<PluginRegister> {
        pub fn free_ptr_to(ptr: WasmPtr<u8>, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            let type_size = std::mem::size_of::<Self>() as u32;
            let type_align = std::mem::align_of::<Self>() as u32;

            free_func.call(WasmPtr::new(ptr.offset()), type_size, type_align).unwrap()
        }
    }
}

/// C-Compatible representation of a system stage
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SystemStage {
    Pre,
    Tick,
    SendPackets,
    CleanUp,
}
