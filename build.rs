// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

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
};

use csv::Reader;

struct Dataset {
    name:        &'static str,
    input_file:  &'static str,
    output_file: &'static str,
    value_type:  &'static str,
    symmetric:   bool,
}

const DATASETS: &[Dataset] = &[
    Dataset {
        name:        "Grantham",
        input_file:  "data/processed/grantham.csv",
        output_file: "lib/datasets/grantham.rs",
        value_type:  "u16",
        symmetric:   true,
    },
    Dataset {
        name:        "Miyata",
        input_file:  "data/processed/miyata.csv",
        output_file: "lib/datasets/miyata.rs",
        value_type:  "f32",
        symmetric:   true,
    },
    Dataset {
        name:        "Epstein",
        input_file:  "data/processed/epstein.csv",
        output_file: "lib/datasets/epstein.rs",
        value_type:  "f32",
        symmetric:   false,
    },
    Dataset {
        name:        "Sneath",
        input_file:  "data/processed/sneath.csv",
        output_file: "lib/datasets/sneath.rs",
        value_type:  "u16",
        symmetric:   false,
    },
    Dataset {
        name:        "Exchangability",
        input_file:  "data/processed/exchangability.csv",
        output_file: "lib/datasets/exchangability.rs",
        value_type:  "u16",
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
        // println!("cargo:rerun-if-changed={}", dataset.input_file);
        generate_dataset(dataset).expect("Failed to generate dataset");
    }
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

#[allow(clippy::too_many_lines)]
fn generate_dataset(dataset: &Dataset) -> std::io::Result<()> {
    let name_map = normalize_amino_acid_names();

    let file = File::open(dataset.input_file)?;
    let mut rdr = Reader::from_reader(BufReader::new(file));

    #[allow(clippy::unwrap_used)]
    let headers = rdr
        .headers()
        .unwrap()
        .iter()
        .skip(1)
        .map(|h| {
            let h_trimmed = h.trim();
            let h_upper = h_trimmed.to_uppercase();
            println!("Processing header: '{h_trimmed}'");
            name_map
                .get(&h_upper)
                .unwrap_or_else(|| panic!("❌ Unknown header: '{h_trimmed}'"))
                .clone()
        })
        .collect::<Vec<_>>();

    let col_index: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, label)| (label.clone(), i))
        .collect();

    let mut matrix: HashMap<String, Vec<String>> = HashMap::new();

    #[allow(clippy::unwrap_used)]
    for result in rdr.records() {
        let record = result?;
        let row_label = record.get(0).unwrap();
        let row_key = name_map
            .get(&row_label.to_uppercase())
            .unwrap_or_else(|| panic!("Unknown row label: {row_label}"))
            .clone();

        let values: Vec<String> = record
            .iter()
            .skip(1)
            .map(|v| {
                let trimmed = v.trim();
                match dataset.value_type {
                    "u16" => {
                        if trimmed == "." {
                            u16::MAX.to_string()
                        } else {
                            trimmed.parse::<u16>().map_or_else(
                                |_| panic!("Failed to parse u16: {trimmed}"),
                                |v| v.to_string(),
                            )
                        }
                    }
                    "f32" => trimmed.parse::<f32>().map_or_else(
                        |_| panic!("Failed to parse f32: {trimmed}"),
                        |v| format!("{v}f32"),
                    ),
                    _ => panic!("Unsupported value type"),
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

    writeln!(
        file,
        "//! Auto-generated from {}\n//! Matrix: {}\n//! Symmetric: {}\n",
        dataset.input_file, dataset.name, dataset.symmetric
    )?;
    writeln!(
        file,
        "use crate::types::AminoAcid;\nuse crate::traits::DistanceMetric;\n"
    )?;

    writeln!(
        file,
        "pub const {}_MATRIX: [{}; {}] = [",
        dataset.name.to_uppercase(),
        dataset.value_type,
        20 * 20
    )?;

    for row_label in AA_ORDER {
        let row = matrix
            .get(row_label)
            .unwrap_or_else(|| panic!("Missing row: {row_label}"));
        for col_label in AA_ORDER {
            let col = col_index
                .get(col_label)
                .unwrap_or_else(|| panic!("Missing column: {col_label}"));
            writeln!(file, "    {},", row[*col])?;
        }
    }

    writeln!(file, "];\n")?;

    writeln!(
        file,
        "#[derive(Debug, Copy, Clone)]\npub struct {};\n",
        dataset.name
    )?;

    writeln!(file, "impl DistanceMetric for {} {{", dataset.name)?;
    writeln!(file, "    type Value = {};", dataset.value_type)?;
    writeln!(
        file,
        "    fn name(&self) -> &'static str {{ \"{}\" }}",
        dataset.name
    )?;
    writeln!(
        file,
        "    fn symmetric(&self) -> bool {{ {} }}",
        dataset.symmetric
    )?;
    writeln!(
        file,
        "    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<Self::Value> {{"
    )?;
    writeln!(file, "        let i = a.index()? * 20 + b.index()?;")?;
    if dataset.symmetric {
        writeln!(file, "        let j = b.index()? * 20 + a.index()?;")?;
        writeln!(file, "        let idx = if i <= j {{ i }} else {{ j }};")?;
    } else {
        writeln!(file, "        let idx = i;")?;
    }
    writeln!(
        file,
        "        Some({}_MATRIX[idx])",
        dataset.name.to_uppercase()
    )?;
    writeln!(file, "    }}\n}}")?;

    Ok(())
}
