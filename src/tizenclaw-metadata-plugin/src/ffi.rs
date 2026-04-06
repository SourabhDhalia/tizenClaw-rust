/*
 * Copyright (c) 2026 Samsung Electronics Co., Ltd.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! FFI type definitions and extern declarations for Tizen pkgmgr APIs.

use std::ffi::{c_char, c_int, c_void};

/// Tizen metadata structure from `<pkgmgr_parser.h>`.
///
/// This matches the `__metadata_t` typedef used by Tizen's package manager
/// parser plugin interface.
#[repr(C)]
pub struct MetadataT {
    pub key: *const c_char,
    pub value: *const c_char,
}

/// GLib doubly-linked list node from `<glib.h>`.
#[repr(C)]
pub struct GList {
    pub data: *mut c_void,
    pub next: *mut GList,
    pub prev: *mut GList,
}

/// Privilege level constants from `<pkgmgr-info.h>`.
pub const PM_PRIVILEGE_UNKNOWN: c_int = -1;
pub const PM_PRIVILEGE_UNTRUSTED: c_int = 0;
pub const PM_PRIVILEGE_PUBLIC: c_int = 1;
pub const PM_PRIVILEGE_PARTNER: c_int = 2;
/// Platform-level privilege (highest tier).
pub const PM_PRIVILEGE_PLATFORM: c_int = 3;

extern "C" {
    /// Query the privilege level of the current package installer session.
    ///
    /// From `<pkgmgr_installer_info.h>`.
    /// Returns 0 on success, non-zero on failure.
    pub fn pkgmgr_installer_info_get_privilege_level(level: *mut c_int) -> c_int;
}
