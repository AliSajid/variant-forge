// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # Variant Forge Library
//!
//! The **Variant Forge** library provides the data structures, distance matrices, and
//! algorithmic tools for evaluating the biochemical similarity of amino acid substitutions.
//!
//! It implements a collection of well-established amino acid distance metrics, each represented
//! as a lookup table or matrix, and exposes a unified trait-based interface to interact with them.
//!
//! ## Features
//!
//! - Unified [`DistanceMetric`](crate::models::DistanceMetric) trait for querying pairwise amino
//!   acid distances.
//! - Multiple metrics supported: [`Grantham`](crate::datasets::Grantham),
//!   [`Miyata`](crate::datasets::Miyata), [`Epstein`](crate::datasets::Epstein),
//!   [`Sneath`](crate::datasets::Epstein), [`Exchangability`](crate::datasets::Exchangability).
//! - Data-backed lookup implementations based on curated CSV matrices.
//! - Peptide comparison utilities (via client code using
//!   [`MetricDataset`](crate::models::MetricDataset)).
//! - Fully type-safe interface using a well-defined [`AminoAcid`](crate::models::AminoAcid) enum.
//!
//! ## Modules
//!
//! - [`datasets`] — Contains the matrix data and struct implementations for each supported metric.
//! - [`models`] — Core types including the amino acid enum, trait definitions, and registry.
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
//! let metric = MetricDataset::Grantham(Default::default());
//!
//! let distance = metric.lookup(AminoAcid::A, AminoAcid::V);
//! println!("Distance A→V: {:?}", distance);
//! ```
//!
//! ## References
//!
//! The distance matrices implemented in this crate are derived from literature sources:
//!
//! - Grantham, R. (1974). *Science*, 185(4154), 862–864.
//! - Miyata, T. et al. (1979). *PNAS*, 76(10), 5400–5404.
//! - Sneath, P. H. A. (1966). *Journal of Theoretical Biology*, 12(2), 157–195.
//! - Epstein, C. J. (1967). *Genetics*, 57(3), 531–544.
//! - Yampolsky & Stoltzfus (2005). *Molecular Biology and Evolution*, 22(4), 803–806.

pub mod datasets;
pub mod models;
pub mod utils;
