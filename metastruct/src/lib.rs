#[cfg(feature = "macro")]
pub use metastruct_macro::metastruct;

/// Trait for structs with a countable number of fields.
///
/// The `Selector` type can be used to select different subsets of fields.
///
/// Implementations of this trait are intended to be written using the `metastruct` macro
/// and the `num_fields` attribute.
pub trait NumFields<Selector> {
    const NUM_FIELDS: usize;
}

pub mod selectors {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AllFields {}
}
