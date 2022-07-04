#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[cfg(feature = "runtime")]
pub use build_info_common::{
	chrono, semver, BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, GitInfo, OptimizationLevel, VersionControl,
};
/// This crate defines macro_rules that pass `$crate` (i.e., this crate) to the proc-macros doing the actual work
/// The proc-macro crate that contains said proc-macros is reexported here, to be found in the macro_rules.
#[doc(hidden)]
pub use build_info_proc as proc;
/// Used internally by the function that is generated by `build_info::build_info!`
#[doc(hidden)]
#[cfg(feature = "runtime")]
pub use lazy_static::lazy_static;
use proc_macro_hack::proc_macro_hack;

/**
Generates a function that returns a reference to the `BuildInfo` structure for the crate.

Usage: `build_info!(fn build_info_function);`
*/
#[cfg(feature = "runtime")]
#[macro_export]
macro_rules! build_info {
	($($tokens:tt)*) => { $crate::proc::build_info!{$crate $($tokens)*} };
}

/**
Generates a string at compile-time that includes build information.

This function-like macro takes a single string-literal as its argument, on which it performs string interpolation with
the current build information. To do so, you can use a subset of the normal format language, with the special
"variable" `$` that denotes the `BuildInfo` object. For example, `build_info::format!("Built at {}", $.timestamp)`
might return "Built at 2020-05-28 20:09:40Z".`

You can use `?` to unwrap `Option`s and some additional types can be formatted this way (e.g., `Vec<T>`).

Literal curly braces can be printed by doubling them up: `build_info::format!("{{}}") // yields "{}"`.
*/
#[cfg_attr(not(feature = "nested"), proc_macro_hack)]
#[cfg_attr(feature = "nested", proc_macro_hack(support_nested))]
pub use build_info_proc::format;
