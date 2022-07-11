use crate::backend::TX;
use crate::events::{Event, EventKind};
use crate::mir_loc::MirLocId;

/// A hook function (see [`HOOK_FUNCTIONS`]).
pub fn malloc(mir_loc: MirLocId, size: u64, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Alloc {
            size: size as usize,
            ptr,
        },
    })
    .unwrap();
}

/// A hook function (see [`HOOK_FUNCTIONS`]).
pub fn free(mir_loc: MirLocId, ptr: usize, _free_ret_val: ()) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Free { ptr },
    })
    .unwrap();
}

/// A hook function (see [`HOOK_FUNCTIONS`]).
pub fn calloc(mir_loc: MirLocId, nmemb: u64, size: u64, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Alloc {
            size: (nmemb * size) as usize,
            ptr,
        },
    })
    .unwrap();
}

/// A hook function (see [`HOOK_FUNCTIONS`]).
pub fn realloc(mir_loc: MirLocId, old_ptr: usize, size: u64, new_ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Free { ptr: old_ptr },
    })
    .unwrap();
    TX.send(Event {
        mir_loc,
        kind: EventKind::Alloc {
            size: size as usize,
            ptr: new_ptr,
        },
    })
    .unwrap();
}

/// A hook function (see [`HOOK_FUNCTIONS`]).
pub fn reallocarray(mir_loc: MirLocId, old_ptr: usize, nmemb: u64, size: u64, new_ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Realloc {
            old_ptr,
            size: (nmemb * size) as usize,
            new_ptr,
        },
    })
    .unwrap();
}

/// A hook function (see [`HOOK_FUNCTIONS`]).
pub fn offset(mir_loc: MirLocId, ptr: usize, offset: isize, new_ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Offset(ptr, offset, new_ptr),
    })
    .unwrap();
}

/// List of functions we want hooked for the lifetime analyis runtime.
/// 
/// For functions in [`HOOK_FUNCTIONS`], the tracing passes 
/// the return value of the traced function as the last argument to the trace hook for it.
/// See the `if after_call` block in `apply_instrumentation` in `dynamic_instrumentation/src/instrument_memory.rs`.
pub const HOOK_FUNCTIONS: &[&str] = &[
    "malloc",
    "free",
    "calloc",
    "realloc",
    "reallocarray",
    "offset",
];

pub fn ptr_field(mir_loc: MirLocId, ptr: usize, field_id: u32) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Field(ptr, field_id),
    })
    .unwrap();
}

pub fn ptr_copy(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::CopyPtr(ptr as usize),
    })
    .unwrap();
}

pub fn ptr_contrive(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::FromInt(ptr as usize),
    })
    .unwrap();
}

pub fn ptr_to_int(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::ToInt(ptr as usize),
    })
    .unwrap();
}

pub fn addr_of_local(mir_loc: MirLocId, ptr: usize, local: u32) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::AddrOfLocal(ptr, local.into()),
    })
    .unwrap();
}

pub fn ref_copy(mir_loc: MirLocId, _dest_local_ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::CopyRef,
    })
    .unwrap();
}

pub fn load_value(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::LoadValue(ptr),
    })
    .unwrap();
}

pub fn store_value(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::StoreValue(ptr),
    })
    .unwrap();
}

pub fn ptr_ret(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::Ret(ptr),
    })
    .unwrap();
}

pub fn ptr_load(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::LoadAddr(ptr),
    })
    .unwrap();
}

pub fn ptr_store(mir_loc: MirLocId, ptr: usize) {
    TX.send(Event {
        mir_loc,
        kind: EventKind::StoreAddr(ptr),
    })
    .unwrap();
}
