// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Trait definition for amino acid distance metrics.
//!
//! Contains the [`DistanceMetric`] trait, which all
//! substitution matrices must implement.
use crate::models::AminoAcid;

/// A trait for implementing amino acid substitution distance metrics.
///
/// This trait defines a common interface for computing the "distance"
/// between pairs of amino acids, representing differences in chemical,
/// physical, or evolutionary properties. These distances can be used
/// to compare individual amino acid substitutions or entire peptides.
///
/// # Supported Metrics
///
/// This trait is implemented by a variety of established metrics:
///
/// - [`Grantham`](crate::datasets::Grantham): Based on composition, polarity, and molecular volume.
/// - [`Miyata`](crate::datasets::Miyata): Based on hydrophobicity and volume.
/// - [`Sneath`](crate::datasets::Sneath): Combines taxonomic and biochemical similarity.
/// - [`Epstein`](crate::datasets::Epstein): Derived from observed chemical groupings.
/// - [`Exchangability`](crate::datasets::Exchangability): Based on empirical substitution rates.
///
/// All metrics return a distance as an `f64`, regardless of internal representation.
///
/// # Example Use Cases
///
/// - Compute the distance between two amino acids
/// - Compare peptide similarity by summing per-position distances
/// - Select different metrics dynamically for sensitivity analysis
///
/// # Examples
///
/// Basic usage:
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
/// let d = metric.lookup(AminoAcid::A, AminoAcid::V);
/// println!("Ala → Val distance: {:?}", d);
///
/// // Metrics are usually symmetric
/// assert_eq!(
///     metric.lookup(AminoAcid::A, AminoAcid::V),
///     metric.lookup(AminoAcid::V, AminoAcid::A)
/// );
/// ```
pub trait DistanceMetric {
    /// Returns a human-readable name for this distance metric.
    ///
    /// Useful for logging, UI display, or debugging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::models::{
    ///     DistanceMetric,
    ///     MetricDataset,
    /// };
    ///
    /// let metric = MetricDataset::Grantham(Default::default());
    /// let metric_name = metric.name();
    /// println!("Using metric: {metric_name}");
    /// ```
    fn name(&self) -> &'static str;

    /// Returns whether this metric is symmetric.
    ///
    /// A symmetric metric satisfies `lookup(a, b) == lookup(b, a)`.
    /// Most substitution matrices follow this pattern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::models::{
    ///     DistanceMetric,
    ///     MetricDataset,
    /// };
    ///
    /// let epstein = MetricDataset::Epstein(Default::default());
    /// assert!(!epstein.is_symmetric());
    /// ```
    fn is_symmetric(&self) -> bool;

    /// Computes the distance between two amino acids.
    ///
    /// # Arguments
    ///
    /// * `a` - First amino acid
    /// * `b` - Second amino acid
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - Distance value between the two amino acids
    /// * `None` - If the pair is unsupported by the metric
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
    /// if let Some(distance) = metric.lookup(AminoAcid::A, AminoAcid::V) {
    ///     println!("Distance A → V: {:.2}", distance);
    /// }
    /// ```
    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<f64>;
}
