use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum TopOutType {
    /// Piece spawns overlapping board
    BlockOut = "BLOCK_OUT",
    /// The piece that spawns in and locks is still in the hidden area
    LockOut = "LOCK_OUT",
    /// Any piece of the board is in the hidden area
    TopOut = "TOP_OUT",
}