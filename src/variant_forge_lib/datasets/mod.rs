// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # Built-in Distance Matrices
//!
//! This module contains amino acid substitution matrices used by the Variant Forge library.
//! Each matrix is backed by curated data and implements the
//! [`DistanceMetric`](crate::models::DistanceMetric) trait, enabling unified distance queries
//! across metrics.
//!
//! ## Auto-Generated Modules
//!
//! All modules in this namespace are **automatically generated at build time** from CSV data
//! and Markdown documentation using the `build.rs` script. Do not manually edit them.
//!
//! ### Included Datasets
//!
//! - [`Grantham`] — Grantham distances based on composition, polarity, and molecular volume.
//! - [`Miyata`] — Miyata distances derived from hydrophobicity and side-chain volume.
//! - [`Sneath`] — Sneath index reflecting chemical taxonomy and biochemical similarity.
//! - [`Epstein`] — Epstein matrix built from chemical groupings and empirical exchange frequencies.
//! - [`Exchangability`] — Experimental exchangeability based on observed substitution frequencies.
//!
//! Each matrix is exposed as a struct (e.g., `Grantham`, `Miyata`) and used through the
//! [`MetricDataset`](crate::models::MetricDataset) enum for ergonomic runtime selection.
//!
//! ## Example
//!
//! ```rust
//! use variant_forge_lib::models::{
//!     AminoAcid,
//!     DistanceMetric,
//!     MetricDataset,
//! };
//!
//! let metric = MetricDataset::Miyata(Default::default());
//! let score = metric.lookup(AminoAcid::A, AminoAcid::V);
//!
//! println!("Miyata distance A→V: {:?}", score);
//! ```
//!
//! ## Note
//!
//! These matrices are flattened 20×20 arrays over the standard amino acid alphabet in
//! row-major order. Missing values (if any) are encoded as `f64::INFINITY`.

mod epstein;
mod exchangability;
mod grantham;
mod miyata;
mod sneath;

pub use epstein::Epstein;
pub use exchangability::Exchangability;
pub use grantham::Grantham;
pub use miyata::Miyata;
pub use sneath::Sneath;
