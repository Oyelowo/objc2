//! # Bindings to the `BrowserEngineCore` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/browserenginecore/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-browser-engine-core/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

#[cfg(feature = "BEMemory")]
#[cfg(target_arch = "aarch64")] // #if TARGET_CPU_ARM64
mod memory;
#[cfg(feature = "BEMemory")]
#[cfg(target_arch = "aarch64")]
pub use self::memory::*;
