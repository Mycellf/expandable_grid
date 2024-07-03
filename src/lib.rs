//! Vectors are stored and modified via `nalgebra::Vector2`
//!
//! See `expandable_grid::ExpanableGrid` for more information.

pub mod expandable_grid;
pub use expandable_grid::ExpandableGrid;

pub mod subchunk;

pub(crate) mod util;

mod tests;
