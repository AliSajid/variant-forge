// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # Core Models for Variant Forge
//!
//! This module defines the foundational data types and traits used throughout
//! the Variant Forge library for analyzing amino acid substitutions.
//!
//! ## Contents
//!
//! - [`AminoAcid`] — Defines the [`AminoAcid`] enum representing the 20 standard amino acids, with
//!   utility methods for indexing, validation, and conversion from names or codes.
//!
//! - [`DistanceMetric`] — Defines the core [`DistanceMetric`] trait, which allows querying
//!   distances between amino acid pairs, with optional symmetry properties.
//!
//! - [`MetricDataset`] — Provides the [`MetricDataset`] enum, a unified type for selecting between
//!   all supported distance metrics at runtime.
//!
//! ## Purpose
//!
//! This module is designed to provide a clean, strongly typed interface between
//! biochemical concepts (like amino acid similarity) and computational logic for
//! distance metric calculations and peptide comparison workflows.
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
//! let distance = metric.lookup(AminoAcid::A, AminoAcid::V);
//! println!("Distance A → V: {:?}", distance);
//! ```
//!
//! See each submodule for more details.

mod amino_acid;
mod distance;
mod registry;

pub use amino_acid::AminoAcid;
pub use distance::DistanceMetric;
pub use registry::MetricDataset;
