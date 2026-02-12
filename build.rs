// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(missing_docs)]
use std::{
    collections::HashMap,
    fs::{
        self,
        File,
    },
    io::{
        BufReader,
        BufWriter,
        Write,
    },
    path::Path,
    process::Command,
};

use csv::Reader;

struct Dataset {
    name:        &'static str,
    input_file:  &'static str,
    doc_file:    &'static str,
    output_file: &'static str,
    symmetric:   bool,
}

const DATASETS: &[Dataset] = &[
    Dataset {
        name:        "Grantham",
        input_file:  "data/processed/grantham.csv",
        doc_file:    "data/docs/grantham.md",
        output_file: "src/variant_forge_lib/datasets/grantham.rs",
        symmetric:   true,
    },
    Dataset {
        name:        "Miyata",
        input_file:  "data/processed/miyata.csv",
        doc_file:    "data/docs/miyata.md",
        output_file: "src/variant_forge_lib/datasets/miyata.rs",
        symmetric:   true,
    },
    Dataset {
        name:        "Epstein",
        input_file:  "data/processed/epstein.csv",
        doc_file:    "data/docs/epstein.md",
        output_file: "src/variant_forge_lib/datasets/epstein.rs",
        symmetric:   false,
    },
    Dataset {
        name:        "Sneath",
        input_file:  "data/processed/sneath.csv",
        doc_file:    "data/docs/sneath.md",
        output_file: "src/variant_forge_lib/datasets/sneath.rs",
        symmetric:   true,
    },
    Dataset {
        name:        "Exchangability",
        input_file:  "data/processed/exchangability.csv",
        doc_file:    "data/docs/exchangability.md",
        output_file: "src/variant_forge_lib/datasets/exchangability.rs",
        symmetric:   false,
    },
];

// Standard amino acid order
const AA_ORDER: [&str; 20] = [
    "A", "R", "N", "D", "C", "Q", "E", "G", "H", "I", "L", "K", "M", "F", "P", "S", "T", "W", "Y",
    "V",
];

fn main() {
    for dataset in DATASETS {
        println!("cargo:rerun-if-changed={}", dataset.input_file);
        println!("cargo:rerun-if-changed={}", dataset.doc_file);
        generate_dataset(dataset).expect("Failed to generate dataset");
    }
}

/// Format a Rust file using rustfmt
fn format_rust_file(file_path: &str) -> std::io::Result<()> {
    let output = Command::new("rustfmt")
        .arg("+nightly")
        .arg("--edition")
        .arg("2024") // Specify Rust edition, adjust if needed
        .arg(file_path)
        .output()?;

    if output.status.success() {
        println!("Successfully formatted: {file_path}");
    } else {
        eprintln!(
            "rustfmt failed for {}: {}",
            file_path,
            String::from_utf8_lossy(&output.stderr)
        );
        // Don't fail the build if rustfmt fails, just warn
        eprintln!("Warning: Generated file {file_path} may not be properly formatted");
    }

    Ok(())
}

fn normalize_amino_acid_names() -> HashMap<String, String> {
    let aliases = [
        ("A", &["Ala", "Alanine"]),
        ("R", &["Arg", "Arginine"]),
        ("N", &["Asn", "Asparagine"]),
        ("D", &["Asp", "Aspartic Acid"]),
        ("C", &["Cys", "Cysteine"]),
        ("Q", &["Gln", "Glutamine"]),
        ("E", &["Glu", "Glutamic Acid"]),
        ("G", &["Gly", "Glycine"]),
        ("H", &["His", "Histidine"]),
        ("I", &["Ile", "Isoleucine"]),
        ("L", &["Leu", "Leucine"]),
        ("K", &["Lys", "Lysine"]),
        ("M", &["Met", "Methionine"]),
        ("F", &["Phe", "Phenylalanine"]),
        ("P", &["Pro", "Proline"]),
        ("S", &["Ser", "Serine"]),
        ("T", &["Thr", "Threonine"]),
        ("W", &["Trp", "Tryptophan"]),
        ("Y", &["Tyr", "Tyrosine"]),
        ("V", &["Val", "Valine"]),
    ];

    let mut map = HashMap::new();
    for (short, names) in aliases {
        for &alias in names {
            map.insert(alias.to_uppercase(), short.to_string());
        }
        map.insert(short.to_string(), short.to_string());
    }
    map
}

#[allow(clippy::too_many_lines, clippy::unwrap_used)]
fn generate_dataset(dataset: &Dataset) -> std::io::Result<()> {
    let name_map = normalize_amino_acid_names();
    let file = File::open(dataset.input_file)?;
    let mut rdr = Reader::from_reader(BufReader::new(file));

    let headers = rdr
        .headers()?
        .iter()
        .skip(1)
        .map(|h| {
            let h_upper = h.trim().to_uppercase();
            name_map
                .get(&h_upper)
                .unwrap_or_else(|| panic!("❌ Unknown header: {h}"))
                .clone()
        })
        .collect::<Vec<_>>();

    let col_index: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, label)| (label.clone(), i))
        .collect();

    let mut matrix: HashMap<String, Vec<String>> = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let row_label = record.get(0).unwrap().to_uppercase();
        let row_key = name_map
            .get(&row_label)
            .unwrap_or_else(|| panic!("Unknown row label: {row_label}"))
            .clone();

        let values: Vec<String> = record
            .iter()
            .skip(1)
            .map(|v| {
                let v = v.trim();
                if v == "." {
                    format!("{}f64", f64::from(u16::MAX))
                } else {
                    format!("{}f64", v.parse::<f64>().unwrap())
                }
            })
            .collect();

        matrix.insert(row_key, values);
    }

    let out_path = Path::new(dataset.output_file);
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = BufWriter::new(File::create(out_path)?);

    let doc = fs::read_to_string(dataset.doc_file).unwrap_or_default();

    writeln!(file, "// SPDX-FileCopyrightText: 2025 Ali Sajid Imami")?;
    writeln!(file, "// SPDX-License-Identifier: Apache-2.0")?;
    writeln!(file, "// SPDX-License-Identifier: MIT\n")?;
    writeln!(file, "//! Auto-generated from `{}`", dataset.input_file)?;
    writeln!(file, "//! Matrix: `{}`", dataset.name)?;
    writeln!(file, "//! Symmetric: {}\n", dataset.symmetric)?;

    writeln!(file, "use crate::models::{{AminoAcid, DistanceMetric}};\n")?;

    for line in doc.lines() {
        writeln!(file, "/// {line}")?;
    }

    writeln!(
        file,
        "#[derive(Debug, Copy, Clone, Default)]\npub struct {};\n",
        dataset.name
    )?;

    writeln!(file, "impl {} {{", dataset.name)?;
    writeln!(file, "    const MATRIX: [f64; 400] = [")?;

    for row_label in AA_ORDER {
        let row = &matrix[row_label];
        for col_label in AA_ORDER {
            let col = col_index[col_label];
            writeln!(file, "        {},", row[col])?;
        }
    }

    writeln!(file, "    ];\n")?;

    writeln!(
        file,
        "    /// Returns a reference to the full amino acid substitution matrix.\n///\n/// This \
         function provides direct access to the raw 20×20 distance matrix\n/// associated with \
         this metric. The matrix is flattened into a one-dimensional\n/// array in **row-major \
         order**, where the index of a pair `(i, j)` can be\n/// computed as `i * 20 + \
         j`.\n///\n/// The values represent pairwise distances between amino acids as \
         defined\n/// by the metric. These values are `f64` values converted from the \
         original\n/// input data. The matrix should generally be accessed through the\n/// \
         [`lookup`](Self::lookup) method for semantic clarity and optionality.\n///\n/// # \
         Returns\n///\n/// A reference to the static array of 400 `f64` values representing \
         the\n/// amino acid substitution matrix.\n///\n/// # Example\n///\n/// ```\n/// use \
         variant_forge_lib::datasets::{};\n///\n/// let matrix = {}::matrix();\n/// let distance \
         = matrix[0 * 20 + 1]; // A → R\n/// assert!(distance >= 0.0);\n/// ```",
        dataset.name, dataset.name,
    )?;
    writeln!(file, "#[must_use]")?;
    writeln!(
        file,
        "    pub const fn matrix() -> &'static [f64; 400] {{ &Self::MATRIX }}"
    )?;
    writeln!(file, "}}\n")?;

    writeln!(file, "impl DistanceMetric for {} {{", dataset.name)?;
    writeln!(
        file,
        "    fn name(&self) -> &'static str {{ \"{}\" }}",
        dataset.name
    )?;
    writeln!(
        file,
        "    fn is_symmetric(&self) -> bool {{ {} }}",
        dataset.symmetric
    )?;
    writeln!(
        file,
        "    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<f64> {{"
    )?;
    writeln!(file, "        let i = a.index() * 20 + b.index();")?;
    if dataset.symmetric {
        writeln!(file, "        let j = b.index() * 20 + a.index();")?;
        writeln!(file, "        let idx = if i <= j {{ i }} else {{ j }};")?;
    } else {
        writeln!(file, "        let idx = i;")?;
    }
    writeln!(file, "        Some(Self::MATRIX[idx])")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;

    writeln!(file, "#[allow(clippy::unwrap_used)]")?;
    writeln!(file, "#[cfg(test)] mod tests {{")?;
    writeln!(file, "    use super::*;")?;
    writeln!(file, "    #[test] fn test_lookup_nonzero() {{")?;
    writeln!(file, "        let metric = {};", dataset.name)?;
    writeln!(
        file,
        "        let d = metric.lookup(AminoAcid::A, AminoAcid::V);"
    )?;
    writeln!(file, "        assert!(d.is_some());")?;
    writeln!(file, "        assert!(d.unwrap() > 0.0);")?;
    writeln!(file, "    }}")?;
    if dataset.symmetric {
        writeln!(file, "    #[test] fn test_symmetry() {{")?;
        writeln!(file, "        let metric = {};", dataset.name)?;
        writeln!(
            file,
            "        assert_eq!(metric.lookup(AminoAcid::A, AminoAcid::V), \
             metric.lookup(AminoAcid::V, AminoAcid::A));"
        )?;
        writeln!(file, "    }}")?;
    }
    writeln!(file, "}}\n")?;

    drop(file);
    format_rust_file(dataset.output_file).ok();

    Ok(())
}
