// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Core definitions and utilities for amino acid representation.
//!
//! Provides the [`AminoAcid`] enum and associated functionality
//! for indexing, display, and conversion from strings.

use std::fmt::{
    self,
    Display,
};

/// Represents the 20 canonical (proteinogenic) amino acids using their standard one-letter codes.
///
/// This enum is designed for bioinformatics applications, particularly for protein sequence
/// analysis, substitution matrices, and molecular biology computations. Each variant corresponds
/// to one of the 20 standard amino acids found in proteins.
///
/// # Amino Acid Mapping
///
/// | Code | Full Name | Three-Letter | Properties |
/// |------|-----------|--------------|------------|
/// | A | Alanine | Ala | Nonpolar, aliphatic |
/// | R | Arginine | Arg | Positively charged |
/// | N | Asparagine | Asn | Polar, uncharged |
/// | D | Aspartic Acid | Asp | Negatively charged |
/// | C | Cysteine | Cys | Polar, contains sulfur |
/// | Q | Glutamine | Gln | Polar, uncharged |
/// | E | Glutamic Acid | Glu | Negatively charged |
/// | G | Glycine | Gly | Nonpolar, smallest amino acid |
/// | H | Histidine | His | Positively charged (at physiological pH) |
/// | I | Isoleucine | Ile | Nonpolar, branched aliphatic |
/// | L | Leucine | Leu | Nonpolar, branched aliphatic |
/// | K | Lysine | Lys | Positively charged |
/// | M | Methionine | Met | Nonpolar, contains sulfur |
/// | F | Phenylalanine | Phe | Nonpolar, aromatic |
/// | P | Proline | Pro | Nonpolar, cyclic (introduces kinks) |
/// | S | Serine | Ser | Polar, uncharged |
/// | T | Threonine | Thr | Polar, uncharged |
/// | W | Tryptophan | Trp | Nonpolar, aromatic, largest |
/// | Y | Tyrosine | Tyr | Polar, aromatic |
/// | V | Valine | Val | Nonpolar, branched aliphatic |
///
/// # Examples
///
/// ```rust
/// use variant_forge_lib::models::AminoAcid;
///
/// // Convert from character
/// let ala = AminoAcid::from_char('A').unwrap();
/// assert_eq!(ala, AminoAcid::A);
///
/// // Convert from name
/// let arg = AminoAcid::from_name("Arginine").unwrap();
/// assert_eq!(arg, AminoAcid::R);
///
/// // Get index for matrix access
/// let index = ala.index();
/// assert_eq!(index, 0);
///
/// // Iterate over all amino acids
/// for aa in AminoAcid::ALL {
///     println!("{}", aa);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum AminoAcid {
    /// Alanine (Ala) - Nonpolar, aliphatic amino acid with a methyl side chain.
    /// Small and chemically inert, often found in alpha helices.
    A,
    /// Arginine (Arg) - Positively charged amino acid with a guanidinium group.
    /// Highly polar and basic, often involved in protein-protein interactions.
    R,
    /// Asparagine (Asn) - Polar, uncharged amino acid with an amide side chain.
    /// Hydrophilic and often found on protein surfaces or in active sites.
    N,
    /// Aspartic acid (Asp) - Negatively charged amino acid with a carboxyl group.
    /// Acidic residue, important for enzyme catalysis and metal binding.
    D,
    /// Cysteine (Cys) - Polar amino acid containing sulfur, capable of forming disulfide bonds.
    /// Critical for protein structure stabilization through cysteine bridges.
    C,
    /// Glutamine (Gln) - Polar, uncharged amino acid with an amide side chain.
    /// Longer than asparagine, often involved in hydrogen bonding networks.
    Q,
    /// Glutamic acid (Glu) - Negatively charged amino acid with a carboxyl group.
    /// Acidic residue, longer than aspartic acid, important in enzyme active sites.
    E,
    /// Glycine (Gly) - The smallest amino acid with only a hydrogen as side chain.
    /// Provides flexibility in protein structures, often found in tight turns.
    G,
    /// Histidine (His) - Positively charged amino acid with an imidazole ring.
    /// Can be protonated/deprotonated at physiological pH, important in catalysis.
    H,
    /// Isoleucine (Ile) - Nonpolar, branched aliphatic amino acid.
    /// Hydrophobic with a branched side chain, commonly found in protein cores.
    I,
    /// Leucine (Leu) - Nonpolar, branched aliphatic amino acid.
    /// Highly hydrophobic, most common amino acid in proteins, favors alpha helices.
    L,
    /// Lysine (Lys) - Positively charged amino acid with an amino group.
    /// Basic residue with a long aliphatic chain, important for DNA binding.
    K,
    /// Methionine (Met) - Nonpolar amino acid containing sulfur.
    /// Often the start amino acid in proteins, important for protein synthesis initiation.
    M,
    /// Phenylalanine (Phe) - Nonpolar, aromatic amino acid with a benzyl side chain.
    /// Large hydrophobic residue, important for protein stability and interactions.
    F,
    /// Proline (Pro) - Nonpolar, cyclic amino acid that introduces rigidity.
    /// Creates kinks in protein structures, breaks alpha helices and beta sheets.
    P,
    /// Serine (Ser) - Polar, uncharged amino acid with a hydroxyl group.
    /// Often involved in phosphorylation and hydrogen bonding networks.
    S,
    /// Threonine (Thr) - Polar, uncharged amino acid with a hydroxyl group.
    /// Similar to serine but with an additional methyl group, also phosphorylatable.
    T,
    /// Tryptophan (Trp) - Nonpolar, aromatic amino acid with an indole side chain.
    /// Largest amino acid, strongly hydrophobic, often buried in protein cores.
    W,
    /// Tyrosine (Tyr) - Polar, aromatic amino acid with a phenolic hydroxyl group.
    /// Can form hydrogen bonds and undergo phosphorylation, important in signaling.
    Y,
    /// Valine (Val) - Nonpolar, branched aliphatic amino acid.
    /// Hydrophobic with a branched side chain, commonly found in beta sheets.
    V,
}

#[allow(dead_code)]
impl AminoAcid {
    /// Array containing all 20 amino acids in alphabetical order by one-letter code.
    ///
    /// Useful for iteration, enumeration, and creating lookup tables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::models::AminoAcid;
    ///
    /// // Count total amino acids
    /// assert_eq!(AminoAcid::ALL.len(), 20);
    ///
    /// // Iterate over all
    /// for aa in AminoAcid::ALL {
    ///     println!("Amino acid: {}", aa);
    /// }
    /// ```
    pub const ALL: [Self; 20] = [
        Self::A,
        Self::R,
        Self::N,
        Self::D,
        Self::C,
        Self::Q,
        Self::E,
        Self::G,
        Self::H,
        Self::I,
        Self::L,
        Self::K,
        Self::M,
        Self::F,
        Self::P,
        Self::S,
        Self::T,
        Self::W,
        Self::Y,
        Self::V,
    ];

    /// Returns the zero-based index (0–19) of this amino acid for matrix access.
    ///
    /// This method is primarily used for accessing elements in substitution matrices
    /// (like BLOSUM or PAM matrices), frequency tables, or other 20x20 amino acid matrices.
    /// The ordering follows the alphabetical order of the one-letter codes.
    ///
    /// # Returns
    ///
    /// A `usize` from 0 to 19, where:
    /// - A = 0, R = 1, N = 2, ..., V = 19
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::models::AminoAcid;
    ///
    /// assert_eq!(AminoAcid::A.index(), 0);
    /// assert_eq!(AminoAcid::V.index(), 19);
    ///
    /// // Use for matrix access
    /// let substitution_matrix: [[i32; 20]; 20] = [[0; 20]; 20];
    /// let score = substitution_matrix[AminoAcid::A.index()][AminoAcid::R.index()];
    /// ```
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::R => 1,
            Self::N => 2,
            Self::D => 3,
            Self::C => 4,
            Self::Q => 5,
            Self::E => 6,
            Self::G => 7,
            Self::H => 8,
            Self::I => 9,
            Self::L => 10,
            Self::K => 11,
            Self::M => 12,
            Self::F => 13,
            Self::P => 14,
            Self::S => 15,
            Self::T => 16,
            Self::W => 17,
            Self::Y => 18,
            Self::V => 19,
        }
    }

    /// Converts a single character to an `AminoAcid` enum variant.
    ///
    /// Accepts both uppercase and lowercase characters. Returns `None` for invalid
    /// amino acid codes or non-standard characters.
    ///
    /// # Arguments
    ///
    /// * `c` - A character representing an amino acid one-letter code
    ///
    /// # Returns
    ///
    /// * `Some(AminoAcid)` if the character is a valid amino acid code
    /// * `None` if the character is not recognized
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::models::AminoAcid;
    ///
    /// assert_eq!(AminoAcid::from_char('A'), Some(AminoAcid::A));
    /// assert_eq!(AminoAcid::from_char('a'), Some(AminoAcid::A)); // Case insensitive
    /// assert_eq!(AminoAcid::from_char('X'), None); // Invalid
    ///
    /// // Parse a sequence
    /// let sequence = "ACGT";
    /// let amino_acids: Vec<AminoAcid> = sequence.chars()
    ///     .filter_map(AminoAcid::from_char)
    ///     .collect();
    /// ```
    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Self::A),
            'R' => Some(Self::R),
            'N' => Some(Self::N),
            'D' => Some(Self::D),
            'C' => Some(Self::C),
            'Q' => Some(Self::Q),
            'E' => Some(Self::E),
            'G' => Some(Self::G),
            'H' => Some(Self::H),
            'I' => Some(Self::I),
            'L' => Some(Self::L),
            'K' => Some(Self::K),
            'M' => Some(Self::M),
            'F' => Some(Self::F),
            'P' => Some(Self::P),
            'S' => Some(Self::S),
            'T' => Some(Self::T),
            'W' => Some(Self::W),
            'Y' => Some(Self::Y),
            'V' => Some(Self::V),
            _ => None,
        }
    }

    /// Converts a full amino acid name or three-letter code to an `AminoAcid` enum variant.
    ///
    /// Accepts both full names (e.g., "Alanine") and three-letter codes (e.g., "Ala").
    /// The comparison is case-insensitive.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice containing either:
    ///   - Full amino acid name (e.g., "Alanine", "Arginine")
    ///   - Three-letter code (e.g., "Ala", "Arg")
    ///
    /// # Returns
    ///
    /// * `Some(AminoAcid)` if the name or code is recognized
    /// * `None` if the input is not a valid amino acid identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variant_forge_lib::models::AminoAcid;
    ///
    /// // Full names
    /// assert_eq!(AminoAcid::from_name("Alanine"), Some(AminoAcid::A));
    /// assert_eq!(AminoAcid::from_name("ALANINE"), Some(AminoAcid::A));
    ///
    /// // Three-letter codes
    /// assert_eq!(AminoAcid::from_name("Ala"), Some(AminoAcid::A));
    /// assert_eq!(AminoAcid::from_name("ALA"), Some(AminoAcid::A));
    ///
    /// // Invalid input
    /// assert_eq!(AminoAcid::from_name("Invalid"), None);
    /// ```
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        let identifier = name.to_ascii_uppercase();
        match identifier.as_str() {
            "ALANINE" | "ALA" => Some(Self::A),
            "ARGININE" | "ARG" => Some(Self::R),
            "ASPARAGINE" | "ASN" => Some(Self::N),
            "ASPARTIC ACID" | "ASP" => Some(Self::D),
            "CYSTEINE" | "CYS" => Some(Self::C),
            "GLUTAMIC ACID" | "GLU" => Some(Self::E),
            "GLUTAMINE" | "GLN" => Some(Self::Q),
            "GLYCINE" | "GLY" => Some(Self::G),
            "HISTIDINE" | "HIS" => Some(Self::H),
            "ISOLEUCINE" | "ILE" => Some(Self::I),
            "LEUCINE" | "LEU" => Some(Self::L),
            "LYSINE" | "LYS" => Some(Self::K),
            "METHIONINE" | "MET" => Some(Self::M),
            "PHENYLALANINE" | "PHE" => Some(Self::F),
            "PROLINE" | "PRO" => Some(Self::P),
            "SERINE" | "SER" => Some(Self::S),
            "THREONINE" | "THR" => Some(Self::T),
            "TRYPTOPHAN" | "TRP" => Some(Self::W),
            "TYROSINE" | "TYR" => Some(Self::Y),
            "VALINE" | "VAL" => Some(Self::V),
            _ => None,
        }
    }
}

/// Implements formatted output for amino acids using their one-letter code.
///
/// # Examples
///
/// ```rust
/// use variant_forge_lib::models::AminoAcid;
///
/// let ala = AminoAcid::A;
/// println!("{}", ala); // Prints: A
///
/// let sequence = vec![AminoAcid::A, AminoAcid::C, AminoAcid::G];
/// for aa in sequence {
///     print!("{}", aa);
/// }
/// // Prints: ACG
/// ```
impl Display for AminoAcid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::*;

    use super::*;

    #[test]
    fn test_amino_acid_debug() {
        let aa = AminoAcid::A;
        let debug_str = format!("{aa:?}");
        assert_eq!(debug_str, "A");
    }

    #[test]
    fn test_amino_acid_partial_eq() {
        assert_eq!(AminoAcid::A, AminoAcid::A);
        assert_ne!(AminoAcid::A, AminoAcid::R);
    }

    // Test index method
    #[rstest]
    #[case(AminoAcid::A, 0)]
    #[case(AminoAcid::R, 1)]
    #[case(AminoAcid::N, 2)]
    #[case(AminoAcid::D, 3)]
    #[case(AminoAcid::C, 4)]
    #[case(AminoAcid::Q, 5)]
    #[case(AminoAcid::E, 6)]
    #[case(AminoAcid::G, 7)]
    #[case(AminoAcid::H, 8)]
    #[case(AminoAcid::I, 9)]
    #[case(AminoAcid::L, 10)]
    #[case(AminoAcid::K, 11)]
    #[case(AminoAcid::M, 12)]
    #[case(AminoAcid::F, 13)]
    #[case(AminoAcid::P, 14)]
    #[case(AminoAcid::S, 15)]
    #[case(AminoAcid::T, 16)]
    #[case(AminoAcid::W, 17)]
    #[case(AminoAcid::Y, 18)]
    #[case(AminoAcid::V, 19)]
    fn test_amino_acid_index(#[case] amino_acid: AminoAcid, #[case] expected_index: usize) {
        assert_eq!(amino_acid.index(), expected_index);
    }

    #[test]
    fn test_all_amino_acids_have_unique_indices() {
        let mut indices = HashSet::new();
        for aa in AminoAcid::ALL {
            let index = aa.index();
            assert!(index < 20, "Index should be less than 20, got {index}");
            assert!(indices.insert(index), "Duplicate index found: {index}");
        }
        assert_eq!(indices.len(), 20);
    }

    // Test from_char method - valid uppercase chars
    #[rstest]
    #[case('A', Some(AminoAcid::A))]
    #[case('R', Some(AminoAcid::R))]
    #[case('N', Some(AminoAcid::N))]
    #[case('D', Some(AminoAcid::D))]
    #[case('C', Some(AminoAcid::C))]
    #[case('Q', Some(AminoAcid::Q))]
    #[case('E', Some(AminoAcid::E))]
    #[case('G', Some(AminoAcid::G))]
    #[case('H', Some(AminoAcid::H))]
    #[case('I', Some(AminoAcid::I))]
    #[case('L', Some(AminoAcid::L))]
    #[case('K', Some(AminoAcid::K))]
    #[case('M', Some(AminoAcid::M))]
    #[case('F', Some(AminoAcid::F))]
    #[case('P', Some(AminoAcid::P))]
    #[case('S', Some(AminoAcid::S))]
    #[case('T', Some(AminoAcid::T))]
    #[case('W', Some(AminoAcid::W))]
    #[case('Y', Some(AminoAcid::Y))]
    #[case('V', Some(AminoAcid::V))]
    fn test_from_char_uppercase_valid(#[case] input: char, #[case] expected: Option<AminoAcid>) {
        assert_eq!(AminoAcid::from_char(input), expected);
    }

    // Test from_char method - valid lowercase chars (should work due to to_ascii_uppercase)
    #[rstest]
    #[case('a', Some(AminoAcid::A))]
    #[case('r', Some(AminoAcid::R))]
    #[case('n', Some(AminoAcid::N))]
    #[case('d', Some(AminoAcid::D))]
    #[case('c', Some(AminoAcid::C))]
    #[case('q', Some(AminoAcid::Q))]
    #[case('e', Some(AminoAcid::E))]
    #[case('g', Some(AminoAcid::G))]
    #[case('h', Some(AminoAcid::H))]
    #[case('i', Some(AminoAcid::I))]
    #[case('l', Some(AminoAcid::L))]
    #[case('k', Some(AminoAcid::K))]
    #[case('m', Some(AminoAcid::M))]
    #[case('f', Some(AminoAcid::F))]
    #[case('p', Some(AminoAcid::P))]
    #[case('s', Some(AminoAcid::S))]
    #[case('t', Some(AminoAcid::T))]
    #[case('w', Some(AminoAcid::W))]
    #[case('y', Some(AminoAcid::Y))]
    #[case('v', Some(AminoAcid::V))]
    fn test_from_char_lowercase_valid(#[case] input: char, #[case] expected: Option<AminoAcid>) {
        assert_eq!(AminoAcid::from_char(input), expected);
    }

    // Test from_char method - valid lowercase chars (should work due to to_ascii_uppercase)
    #[rstest]
    #[case("Alanine", Some(AminoAcid::A))]
    #[case("Arginine", Some(AminoAcid::R))]
    #[case("Asparagine", Some(AminoAcid::N))]
    #[case("Aspartic Acid", Some(AminoAcid::D))]
    #[case("Cysteine", Some(AminoAcid::C))]
    #[case("Glutamic Acid", Some(AminoAcid::E))]
    #[case("Glutamine", Some(AminoAcid::Q))]
    #[case("Glycine", Some(AminoAcid::G))]
    #[case("Histidine", Some(AminoAcid::H))]
    #[case("Isoleucine", Some(AminoAcid::I))]
    #[case("Leucine", Some(AminoAcid::L))]
    #[case("Lysine", Some(AminoAcid::K))]
    #[case("Methionine", Some(AminoAcid::M))]
    #[case("Phenylalanine", Some(AminoAcid::F))]
    #[case("Proline", Some(AminoAcid::P))]
    #[case("Serine", Some(AminoAcid::S))]
    #[case("Threonine", Some(AminoAcid::T))]
    #[case("Tryptophan", Some(AminoAcid::W))]
    #[case("Tyrosine", Some(AminoAcid::Y))]
    #[case("Valine", Some(AminoAcid::V))]
    fn test_from_name(#[case] input: &str, #[case] expected: Option<AminoAcid>) {
        assert_eq!(AminoAcid::from_name(input), expected);
    }

    #[rstest]
    #[case("Ala", Some(AminoAcid::A))]
    #[case("Arg", Some(AminoAcid::R))]
    #[case("Asn", Some(AminoAcid::N))]
    #[case("Asp", Some(AminoAcid::D))]
    #[case("Cys", Some(AminoAcid::C))]
    #[case("Glu", Some(AminoAcid::E))]
    #[case("Gln", Some(AminoAcid::Q))]
    #[case("Gly", Some(AminoAcid::G))]
    #[case("His", Some(AminoAcid::H))]
    #[case("Ile", Some(AminoAcid::I))]
    #[case("Leu", Some(AminoAcid::L))]
    #[case("Lys", Some(AminoAcid::K))]
    #[case("Met", Some(AminoAcid::M))]
    #[case("Phe", Some(AminoAcid::F))]
    #[case("Pro", Some(AminoAcid::P))]
    #[case("Ser", Some(AminoAcid::S))]
    #[case("Thr", Some(AminoAcid::T))]
    #[case("Trp", Some(AminoAcid::W))]
    #[case("Tyr", Some(AminoAcid::Y))]
    #[case("Val", Some(AminoAcid::V))]
    fn test_from_name_short(#[case] input: &str, #[case] expected: Option<AminoAcid>) {
        assert_eq!(AminoAcid::from_name(input), expected);
    }

    // Test from_char method - invalid chars
    #[rstest]
    #[case('X')]
    #[case('Z')]
    #[case('B')]
    #[case('J')]
    #[case('O')]
    #[case('U')]
    #[case('1')]
    #[case('!')]
    #[case(' ')]
    #[case('\n')]
    #[case('\t')]
    #[case('α')] // Non-ASCII character
    fn test_from_char_invalid(#[case] input: char) {
        assert_eq!(AminoAcid::from_char(input), None);
    }

    // Test ALL constant
    #[test]
    fn test_all_constant_length() {
        assert_eq!(AminoAcid::ALL.len(), 20);
    }

    #[test]
    fn test_all_constant_contains_all_amino_acids() {
        let expected_amino_acids = [
            AminoAcid::A,
            AminoAcid::R,
            AminoAcid::N,
            AminoAcid::D,
            AminoAcid::C,
            AminoAcid::Q,
            AminoAcid::E,
            AminoAcid::G,
            AminoAcid::H,
            AminoAcid::I,
            AminoAcid::L,
            AminoAcid::K,
            AminoAcid::M,
            AminoAcid::F,
            AminoAcid::P,
            AminoAcid::S,
            AminoAcid::T,
            AminoAcid::W,
            AminoAcid::Y,
            AminoAcid::V,
        ];

        for (i, &expected) in expected_amino_acids.iter().enumerate() {
            assert_eq!(AminoAcid::ALL[i], expected, "Mismatch at index {i}");
        }
    }

    // Edge case tests
    #[test]
    fn test_from_char_with_unicode() {
        // Test with some Unicode characters that might look similar to amino acids
        assert_eq!(AminoAcid::from_char('Α'), None); // Greek Alpha
        assert_eq!(AminoAcid::from_char('А'), None); // Cyrillic A
        assert_eq!(AminoAcid::from_char('𝐀'), None); // Mathematical bold A
    }
}
