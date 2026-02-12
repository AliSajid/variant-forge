// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Registry of supported distance metrics.
//!
//! Defines the [`MetricDataset`] enum to enable dynamic selection
//! of different amino acid substitution matrices.
use crate::{
    datasets::{
        Epstein,
        Exchangability,
        Grantham,
        Miyata,
        Sneath,
    },
    models::{
        AminoAcid,
        DistanceMetric,
    },
};

/// A unified enum representing all available amino acid distance metrics.
///
/// `MetricDataset` allows dynamic dispatch over multiple distance matrix implementations
/// that conform to the [`DistanceMetric`] trait. This enables users
/// to select and use different substitution models at runtime without changing logic.
///
/// # Variants
///
/// - [`Grantham`](MetricDataset::Grantham): Based on composition, polarity, and volume.
/// - [`Miyata`](MetricDataset::Miyata): Uses hydrophobicity and side-chain volume.
/// - [`Sneath`](MetricDataset::Sneath): Derived from taxonomic and chemical classifications.
/// - [`Epstein`](MetricDataset::Epstein): Captures empirical biochemical groupings.
/// - [`Exchangability`](MetricDataset::Exchangability): Experimental substitution frequencies.
///
/// # Examples
///
/// ```rust
/// use variant_forge_lib::models::{
///     AminoAcid,
///     DistanceMetric,
///     MetricDataset,
/// };
///
/// let metric = MetricDataset::Grantham(Default::default());
///
/// let distance = metric.lookup(AminoAcid::A, AminoAcid::V);
/// println!("Ala → Val distance: {:?}", distance);
/// ```
#[derive(Debug, Clone)]
pub enum MetricDataset {
    /// The Grantham Distance matrix
    Grantham(Grantham),
    /// The Miyata Distance matrix
    Miyata(Miyata),
    /// The Sneath Index matrix
    Sneath(Sneath),
    /// The Epstein Coefficient matrix
    Epstein(Epstein),
    /// The Experimental Exchangability matrix
    Exchangability(Exchangability),
}

impl DistanceMetric for MetricDataset {
    fn name(&self) -> &'static str {
        match self {
            Self::Grantham(d) => d.name(),
            Self::Miyata(d) => d.name(),
            Self::Sneath(d) => d.name(),
            Self::Epstein(d) => d.name(),
            Self::Exchangability(d) => d.name(),
        }
    }

    fn is_symmetric(&self) -> bool {
        match self {
            Self::Grantham(d) => d.is_symmetric(),
            Self::Miyata(d) => d.is_symmetric(),
            Self::Sneath(d) => d.is_symmetric(),
            Self::Epstein(d) => d.is_symmetric(),
            Self::Exchangability(d) => d.is_symmetric(),
        }
    }

    fn lookup(&self, a1: AminoAcid, a2: AminoAcid) -> Option<f64> {
        match self {
            Self::Grantham(d) => d.lookup(a1, a2),
            Self::Miyata(d) => d.lookup(a1, a2),
            Self::Sneath(d) => d.lookup(a1, a2),
            Self::Epstein(d) => d.lookup(a1, a2),
            Self::Exchangability(d) => d.lookup(a1, a2),
        }
    }
}
