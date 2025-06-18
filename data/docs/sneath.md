# Sneath Similarity Index Matrix

 The Sneath matrix encodes the similarity between amino acids based on shared features
 across multiple biochemical and structural properties. It is derived from binary feature profiles
 for each amino acid, such as polarity, aromaticity, size, and charge.

 The Sneath index is a symmetric similarity matrix (higher values mean greater similarity),
 rather than a distance matrix.

 **Citation**:
 Sneath, P. H. A. (1966). Relations between chemical structure and biological activity in peptides.
 *Journal of Theoretical Biology*, 12(2), 157–195. <https://doi.org/10.1016/0022-5193(66)90112-9>

 **Value type**: `u16`

 **Symmetry**: Yes (symmetric)

 **Missing values**: None; all values are filled

 **Amino Acid Row/Column Order**:
 A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
