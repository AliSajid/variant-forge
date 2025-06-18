// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Variant Forge Library
//!
//! This library contains all the data and logic for the variant-forge application
mod datasets;
mod models;

#[allow(unused_imports)]
pub use datasets::{
    epstein::Epstein,
    exchangability::Exchangability,
    grantham::Grantham,
    miyata::Miyata,
    sneath::Sneath,
};
#[allow(unused_imports)]
pub use models::{
    amino_acid::AminoAcid,
    distance::DistanceMetric,
    registry::Registry,
};
