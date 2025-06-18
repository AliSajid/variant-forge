// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::AminoAcid;

/// A trait for implementing distance metrics between amino acids.
///
/// This trait provides a common interface for amino acid distance metrics that quantify
/// the physicochemical and evolutionary differences between amino acid pairs. These distances
/// are used to calculate functional "distance" between peptides by summing the individual
/// amino acid distances at each position.
///
/// # Supported Distance Metrics
///
/// This trait is designed to support the following established amino acid distance metrics:
///
/// - **Grantham Distance**: Based on composition, polarity, and molecular volume differences
/// - **Miyata Distance**: Derived from physicochemical properties including hydrophobicity and
///   volume
/// - **Sneath Distance**: Based on taxonomic and physicochemical relationships
/// - **Epstein Distance**: Considers amino acid exchange frequencies and chemical properties
/// - **Experimental Exchangeability**: Based on observed substitution rates in laboratory
///   experiments
///
/// # Peptide Distance Calculation
///
/// The primary use case is calculating distances between peptides of equal length by:
/// 1. Computing the distance between amino acids at each corresponding position
/// 2. Summing these individual distances to get the total peptide distance
///
/// ```rust
/// use variant_forge_lib::{
///     AminoAcid,
///     DistanceMetric,
/// };
///
/// fn peptide_distance<M: DistanceMetric>(
///     metric: &M,
///     peptide1: &[AminoAcid],
///     peptide2: &[AminoAcid],
/// ) -> Option<M::Value>
/// where
///     M::Value: std::ops::Add<Output = M::Value> + Default,
/// {
///     if peptide1.len() != peptide2.len() {
///         return None;
///     }
///
///     let mut total_distance = M::Value::default();
///     for (aa1, aa2) in peptide1.iter().zip(peptide2.iter()) {
///         let distance = metric.lookup(*aa1, *aa2)?;
///         total_distance = total_distance + distance;
///     }
///     Some(total_distance)
/// }
///
/// // Example usage
/// let grantham = variant_forge_lib::Grantham;
/// let peptide1 = vec![AminoAcid::A, AminoAcid::L, AminoAcid::A];
/// let peptide2 = vec![AminoAcid::V, AminoAcid::I, AminoAcid::S];
///
/// if let Some(distance) = peptide_distance(&grantham, &peptide1, &peptide2) {
///     println!("Peptide distance: {:?}", distance);
/// }
/// ```
///
/// # Examples
///
/// ```rust
/// use variant_forge_lib::{self, AminoAcid, DistanceMetric};
///
/// // Example: Grantham distance implementation
/// let grantham = variant_forge_lib::Grantham;
///
/// // Get distance between individual amino acids
/// let distance = grantham.lookup(AminoAcid::A, AminoAcid::V);
/// println!("Grantham distance Ala->Val: {:?}", distance);
///
/// // All supported metrics are symmetric
/// assert_eq!(
///     grantham.lookup(AminoAcid::A, AminoAcid::V),
///     grantham.lookup(AminoAcid::V, AminoAcid::A)
/// );
///
/// // Compare different amino acid pairs
/// let close_pair = grantham.lookup(AminoAcid::A, AminoAcid::V);  // Similar properties
/// let distant_pair = grantham.lookup(AminoAcid::A, AminoAcid::W); // Very different
///
/// if let (Some(close), Some(distant)) = (close_pair, distant_pair) {
///     assert!(close < distant); // Ala-Val should be closer than Ala-Trp
/// }
/// ```
#[allow(dead_code)]
pub trait DistanceMetric {
    /// The numeric type used to represent distance values.
    ///
    /// Must implement `Copy` for efficiency, `PartialOrd` for comparisons,
    /// and `Debug` for diagnostic output. Common types include:
    /// - `f32` for memory-efficient floating point
    /// - `u32` for non-negative integer distances
    type Value: Copy + PartialOrd + std::fmt::Debug;

    /// Returns a human-readable name for this distance metric.
    ///
    /// This is useful for logging, debugging, and user interfaces where the
    /// specific metric being used needs to be identified.
    ///
    /// # Returns
    ///
    /// A static string slice containing the metric name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::{
    ///     DistanceMetric,
    ///     Grantham,
    /// };
    ///
    /// println!("Using metric: {}", Grantham.name());
    /// ```
    fn name(&self) -> &'static str;

    /// Returns whether this distance metric is symmetric.
    ///
    /// A symmetric metric satisfies: `distance(a, b) == distance(b, a)`
    /// Most distance metrics are symmetric, but some specialized metrics
    /// (like directional substitution matrices) may not be.
    ///
    /// This information can be used for optimizations when calculating
    /// distance matrices or when the order of comparison doesn't matter.
    ///
    /// # Returns
    ///
    /// `true` if the metric is symmetric, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::{
    ///     self,
    ///     DistanceMetric,
    /// };
    ///
    /// let grantham = variant_forge_lib::Grantham;
    /// let epstein = variant_forge_lib::Epstein;
    ///
    /// println!(
    ///     "Is Grantham distance Symmetric? {}",
    ///     grantham.is_symmetric()
    /// );
    /// println!("Is Epstein distance Symmetric? {}", epstein.is_symmetric());
    /// ```
    fn is_symmetric(&self) -> bool;

    /// Looks up the distance between two amino acids.
    ///
    /// This is the core method that computes or retrieves the distance value
    /// between any two amino acids according to this metric's definition.
    ///
    /// # Arguments
    ///
    /// * `a` - First amino acid
    /// * `b` - Second amino acid
    ///
    /// # Returns
    ///
    /// * `Some(Value)` - The distance/similarity score between the amino acids
    /// * `None` - If the distance cannot be computed (e.g., unsupported amino acid pair)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::{
    ///     self,
    ///     AminoAcid,
    ///     DistanceMetric,
    /// };
    ///
    /// let metric = variant_forge_lib::Grantham;
    ///
    /// // Get distance between alanine and valine
    /// if let Some(distance) = metric.lookup(AminoAcid::A, AminoAcid::V) {
    ///     println!("Ala-Val distance: {:?}", distance);
    /// }
    ///
    /// // Compare distances
    /// let dist1 = metric.lookup(AminoAcid::A, AminoAcid::V);
    /// let dist2 = metric.lookup(AminoAcid::A, AminoAcid::F);
    ///
    /// match (dist1, dist2) {
    ///     (Some(d1), Some(d2)) if d1 < d2 => {
    ///         println!("Ala is closer to Val than to Phe");
    ///     }
    ///     _ => println!("Cannot compare distances"),
    /// }
    /// ```
    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<Self::Value>;
}
