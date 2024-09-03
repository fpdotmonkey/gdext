/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Compatibility

// Code generated by Rust derive macros cannot cause any deprecation warnings, due to questionable "feature"
// https://github.com/rust-lang/rust/pull/58994. Fortunately, an extra layer of indirection solves most problems: we generate a declarative
// macro that itself isn't deprecated, but _its_ expansion is. Since the expansion happens in a later step, the warning is emitted.

// Usage example.
//
// 1. Declare a const fn which describes the deprecation warning.
//
//     #[deprecated = "#[base] is no longer needed; Base<T> is recognized directly. \n\
//             More information on https://github.com/godot-rust/gdext/pull/577."]
//     pub const fn base_attribute() {}
//
// 2. At the place of usage, use the `emit_deprecated_warning!` macro to emit the warning. This can be generated by codegen, as well.
//
//     #[cfg(feature = "custom-godot")]
//     __deprecated::emit_deprecated_warning!(feature_custom_godot);

#[macro_export]
macro_rules! emit_deprecated_warning {
    ($warning_fn:ident) => {
        const _: () = $crate::__deprecated::$warning_fn();
    };
}

pub use crate::emit_deprecated_warning;

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Library-side deprecations

#[deprecated = "\nThe attribute key #[init(val = ...)] replaces #[init(default = ...)].\n\
	More information on https://github.com/godot-rust/gdext/pull/844."]
pub const fn init_default() {}

#[deprecated = "\nThe attribute key #[class(editor_plugin)] is now implied by #[class(base = EditorPlugin)]. It is ignored.\n\
	More information on https://github.com/godot-rust/gdext/pull/884."]
pub const fn editor_plugin() {}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Godot-side deprecations

// This is a Godot-side deprecation. Since it's the only way in Godot 4.1, we keep compatibility for now.
#[cfg_attr(
    since_api = "4.2",
    deprecated = "\nUse #[export(range = (radians_as_degrees))] and not #[export(range = (radians))].\n\
	More information on https://github.com/godotengine/godot/pull/82195."
)]
pub const fn export_range_radians() {}
