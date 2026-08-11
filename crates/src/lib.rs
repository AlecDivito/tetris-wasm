mod utils;
pub mod tetris;

#[cfg(target_arch = "wasm32")]
extern crate web_sys;
#[cfg(target_arch = "wasm32")]
extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

