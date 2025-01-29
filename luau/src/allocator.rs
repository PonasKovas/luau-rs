use std::{
    alloc::Layout,
    ffi::c_void,
    hint::unreachable_unchecked,
    num::NonZero,
    ptr::{null_mut, NonNull},
};

/// A trait for allocators that luau can use inside the VM
///
/// You can introduce arbitrary limits or control using this
pub trait LuauAllocator {
    /// Called to allocate `size` number of bytes of memory, must return a pointer to that memory
    /// return `None` to signal failure
    unsafe fn alloc(&mut self, size: NonZero<usize>) -> Option<NonNull<()>>;
    /// Called to resize already allocated memory, `ptr` is the old pointer,
    /// `old_size` is the original size, `new_size` is the new requested size
    /// must returns a pointer to the new allocated memory of size `new_size`
    /// return `None` to signal failure
    unsafe fn realloc(
        &mut self,
        ptr: NonNull<()>,
        old_size: NonZero<usize>,
        new_size: NonZero<usize>,
    ) -> Option<NonNull<()>>;
    /// Called to deallocate memory at `ptr` of size `size`
    unsafe fn free(&mut self, ptr: NonNull<()>, size: NonZero<usize>);
}

// Default allocator:
/////////////////////

/// Default allocator that just puts a simple constant limit on maximum memory that
/// luau is allowed to allocate.
#[derive(Debug, PartialEq, Clone)]
pub struct LuauAllocatorDefault {
    /// Memory constraint on the VM in bytes. Unconstrained if `None`.
    pub memory_limit: Option<usize>,

    used_memory: usize,
}

impl LuauAllocatorDefault {
    /// Creates a new default allocator with the given memory limit in bytes.
    pub fn new(memory_limit: Option<usize>) -> Self {
        Self {
            memory_limit,
            used_memory: 0,
        }
    }
}

impl Default for LuauAllocatorDefault {
    /// Default has no limit
    fn default() -> Self {
        Self::new(None)
    }
}

impl LuauAllocator for LuauAllocatorDefault {
    unsafe fn alloc(&mut self, size: NonZero<usize>) -> Option<NonNull<()>> {
        let size = size.get();

        // check if within limits
        if let Some(limit) = self.memory_limit {
            if self.used_memory + size > limit {
                // oopsie
                // no allocation for you :(
                return None;
            }
        }

        // allocate with rust's global allocator
        let layout = get_layout(size)?;
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut ();

        self.used_memory += size;

        NonNull::new(ptr)
    }
    unsafe fn realloc(
        &mut self,
        ptr: std::ptr::NonNull<()>,
        old_size: NonZero<usize>,
        new_size: NonZero<usize>,
    ) -> Option<std::ptr::NonNull<()>> {
        let old_size = old_size.get();
        let new_size = new_size.get();

        let size_change: isize = new_size as isize - old_size as isize;
        // its UB for this to ever wrap but i just didnt find a different method to add usize + isize
        let new_used_memory = self.used_memory.wrapping_add_signed(size_change);

        // check if within limits
        if let Some(limit) = self.memory_limit {
            if new_used_memory > limit {
                // oopsie
                // no allocation for you :(
                return None;
            }
        }

        // reallocate with rust's global allocator
        let layout = get_layout(old_size)?;
        let ptr =
            unsafe { std::alloc::realloc(ptr.as_ptr() as *mut u8, layout, new_size) } as *mut ();

        self.used_memory = new_used_memory;

        NonNull::new(ptr)
    }
    unsafe fn free(&mut self, ptr: std::ptr::NonNull<()>, size: NonZero<usize>) {
        let size = size.get();

        // deallocate with rust's global allocator
        let layout = get_layout(size).unwrap();
        unsafe { std::alloc::dealloc(ptr.as_ptr() as *mut u8, layout) };

        self.used_memory -= size;
    }
}

fn get_layout(size: usize) -> Option<Layout> {
    // its not really documented what alignment luau expects so we just give the maximum
    // just like malloc does
    let alignment = align_of::<libc::max_align_t>();

    Layout::from_size_align(size, alignment).ok()
}

fn raw<A: LuauAllocator>() -> luau_sys::vm::lua_Alloc {
    unsafe extern "C" fn raw_alloc<A: LuauAllocator>(
        alloc: *mut c_void,
        ptr: *mut c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut c_void {
        let allocator = unsafe { (alloc as *mut A).as_mut().unwrap_unchecked() };
        let ptr = ptr as *mut ();

        match (NonNull::new(ptr), NonZero::new(osize), NonZero::new(nsize)) {
            (Some(p), Some(s), None) => unsafe { allocator.free(p, s) },
            (Some(p), Some(os), Some(ns)) => {
                return match unsafe { allocator.realloc(p, os, ns) } {
                    Some(ptr) => ptr.as_ptr() as *mut c_void,
                    None => null_mut(),
                }
            }
            (None, None, Some(s)) => {
                return match unsafe { allocator.alloc(s) } {
                    Some(ptr) => ptr.as_ptr() as *mut c_void,
                    None => null_mut(),
                }
            }

            // all other variants are invalid and UB, theres no point in trying to panic here
            // or handle it differently since we cant unwind across the C abi
            // just embrace the UB.
            _ => unsafe { unreachable_unchecked() },
        };

        null_mut()
    }

    Some(raw_alloc::<A>)
}
