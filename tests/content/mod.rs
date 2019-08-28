#[cfg(all(
    feature = "derive",
    any(feature = "serde-base", feature = "serde-full")
))]
mod serde_support;

#[cfg(all(feature = "derive", feature = "unhtml-html"))]
mod unhtml_support;
