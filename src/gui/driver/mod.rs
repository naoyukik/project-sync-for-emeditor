//! GUI Driver layer
//! This module contains Win32 window operations and resource definitions.

/// Win32 resource identifiers generated from resource.h
pub mod resource {
    include!(concat!(env!("OUT_DIR"), "/resource.rs"));
}
