#!/bin/bash

# SPDX-FileCopyrightText: 2025 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

# Comprehensive Amino Acid Data Fetcher
# Outputs both JSON and CSV formats
set -euo pipefail # Strict error handling

# Configuration
API_BASE="https://aaprop-c1io.shuttle.app/v1/amino_acid"
OUTPUT_JSON="data/amino_acids_full.json"
FILTERED_JSON="data/amino_acids_filtered.json"
OUTPUT_CSV="data/amino_acids.csv"
STANDARD_AMINO_ACIDS=("A" "C" "D" "E" "F" "G" "H" "I" "K" "L" "M" "N" "P" "Q" "R" "S" "T" "V" "W" "Y")

# Check dependencies
check_dependencies() {
  for cmd in curl jq; do
    if ! command -v "$cmd" &>/dev/null; then
      echo "Error: $cmd is required but not installed." >&2
      exit 1
    fi
  done
}

# Create output directory if it doesn't exist
create_output_dir() {
  mkdir -p "$(dirname "$OUTPUT_JSON")"
}

# Initialize output files with headers
init_files() {
  # JSON initialization
  echo "[" >"${OUTPUT_JSON}.tmp"
  # Filtered JSON initialization
  echo "[" >"${FILTERED_JSON}.tmp"
  # CSV header
  echo -e "abbreviation,name,short_name" >"${OUTPUT_CSV}.tmp"
}

# Cleanup function
cleanup() {
  for file in "${OUTPUT_JSON}.tmp" "${FILTERED_JSON}.tmp" "${OUTPUT_CSV}.tmp"; do
    if [ -f "$file" ]; then
      rm -f "$file"
    fi
  done
}

# Main execution
main() {
  trap cleanup EXIT ERR
  check_dependencies
  create_output_dir
  init_files

  echo "Starting data fetch at $(date -u)"
  echo "----------------------------------------"

  local count=0
  local success_count=0

  for letter in "${STANDARD_AMINO_ACIDS[@]}"; do
    ((count++))
    printf "Processing %s (%d/%d)..." "$letter" "$count" "${#STANDARD_AMINO_ACIDS[@]}"

    local response
    local retries=3
    local success=false

    # Retry logic
    for ((i = 1; i <= retries; i++)); do
      if response=$(curl -sS --fail --max-time 10 "${API_BASE}/${letter}" 2>/dev/null); then
        if jq -e . >/dev/null 2>&1 <<<"$response"; then
          success=true
          break
        fi
      fi
      sleep 1
    done

    if $success; then
      ((success_count++))

      # Add comma separator for JSON (except for first item)
      if [ $success_count -gt 1 ]; then
        echo "," >>"${OUTPUT_JSON}.tmp"
        echo "," >>"${FILTERED_JSON}.tmp"
      fi

      # Full JSON processing
      echo "$response" >>"${OUTPUT_JSON}.tmp"

      # Filtered JSON processing
      jq '{name, short_name, abbreviation}' <<<"$response" >>"${FILTERED_JSON}.tmp"

      # CSV processing
      jq -r '[.abbreviation, .name, .short_name] | @csv' <<<"$response" >>"${OUTPUT_CSV}.tmp"

      echo " ✓"
    else
      echo " ✗ (failed after $retries attempts)"
    fi
  done

  # Finalize JSON files
  echo "]" >>"${OUTPUT_JSON}.tmp"
  echo "]" >>"${FILTERED_JSON}.tmp"

  # Pretty-print and move to final locations
  jq '.' "${OUTPUT_JSON}.tmp" >"$OUTPUT_JSON"
  jq '.' "${FILTERED_JSON}.tmp" >"$FILTERED_JSON"
  mv "${OUTPUT_CSV}.tmp" "$OUTPUT_CSV"

  # Clean up temp files
  rm -f "${OUTPUT_JSON}.tmp" "${FILTERED_JSON}.tmp" "${OUTPUT_CSV}.tmp"

  echo "----------------------------------------"
  echo "Successfully processed $success_count/${#STANDARD_AMINO_ACIDS[@]} amino acids"
  echo "Output files:"
  echo "- Full JSON: $OUTPUT_JSON"
  echo "- Filtered JSON: $FILTERED_JSON"
  echo "- CSV: $OUTPUT_CSV"
}

main
