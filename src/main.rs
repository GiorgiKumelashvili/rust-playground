use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

// --- Data Structure ---
// This struct will be our intermediary representation.
// We derive Serialize and Deserialize for it.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Record {
    id: u32,
    name: String,
    value: f64,
    active: bool,
}

// --- Format Enum ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Json,
    Yaml,
    Csv,
    Toml,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // Simple debug representation for display
    }
}


// --- Custom Error Type ---
#[derive(Debug, thiserror::Error)]
enum ConversionError {
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML Error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("CSV Error: {0}")]
    Csv(#[from] csv::Error),
    #[error("TOML Deserialization Error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("TOML Serialization Error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("UTF8 Error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("IO Error: {0}")] // Needed for CSV writer-to-memory
    Io(#[from] std::io::Error),
    #[error("Cannot represent this data structure as {0}")]
    UnsupportedRepresentation(Format),
    #[error("Input data for {0} format is empty")]
    EmptyInput(Format),
}


// --- Conversion Logic ---

// Helper to deserialize from a string based on format
fn deserialize_from_string(
    input: &str,
    format: Format,
) -> Result<Vec<Record>, ConversionError> {
    if input.trim().is_empty() {
        return Err(ConversionError::EmptyInput(format));
    }

    match format {
        Format::Json => {
            let records: Vec<Record> = serde_json::from_str(input)?;
            Ok(records)
        }
        Format::Yaml => {
            let records: Vec<Record> = serde_yaml::from_str(input)?;
            Ok(records)
        }
        Format::Csv => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true) // Assume CSV has headers matching struct fields
                .from_reader(input.as_bytes());
            let mut records = Vec::new();
            for result in reader.deserialize() {
                let record: Record = result?;
                records.push(record);
            }
            Ok(records)
        }
        Format::Toml => {
            // TOML often represents a single structure. If we want a list,
            // it's typically represented as an array of tables.
            // We might need a wrapper struct if the TOML isn't directly an array.
            // Let's assume the TOML *is* an array of tables representing Vec<Record>.
            // Or, more commonly, it's under a specific key. Let's use a wrapper.
            #[derive(Deserialize)]
            struct TomlWrapper {
                records: Vec<Record>,
            }
            let wrapper: TomlWrapper = toml::from_str(input)?;
            Ok(wrapper.records)
            // // If the TOML was *just* the array of tables directly:
            // let records: Vec<Record> = toml::from_str(input)?;
            // Ok(records)
        }
    }
}

// Helper to serialize to a string based on format
fn serialize_to_string(
    records: &[Record],
    format: Format,
) -> Result<String, ConversionError> {
    match format {
        Format::Json => {
            let json_string = serde_json::to_string_pretty(records)?;
            Ok(json_string)
        }
        Format::Yaml => {
            let yaml_string = serde_yaml::to_string(records)?;
            Ok(yaml_string)
        }
        Format::Csv => {
            // Write CSV to a Vec<u8> in memory, then convert to String
            let mut writer = csv::WriterBuilder::new()
                .has_headers(true) // Write headers based on struct fields
                .from_writer(Vec::new());
            for record in records {
                writer.serialize(record)?;
            }
            writer.flush()?; // Ensure all data is written to the buffer
            let csv_data = writer.into_inner().unwrap();
            let csv_string = String::from_utf8(csv_data)?;
            Ok(csv_string)
        }
        Format::Toml => {
            // To serialize Vec<Record> into a meaningful TOML, we usually
            // put it under a key, as TOML files prefer a top-level table.
            #[derive(Serialize)]
            struct TomlWrapper<'a> {
                records: &'a [Record],
            }
            let wrapper = TomlWrapper { records };
            let toml_string = toml::to_string_pretty(&wrapper)?;
            Ok(toml_string)
            // // If you wanted to serialize *just* the array of tables (less common for root):
            // let toml_string = toml::to_string_pretty(records)?;
            // Ok(toml_string)
        }
    }
}

// --- Main Conversion Function ---
fn convert_data(
    input_string: &str,
    input_format: Format,
    output_format: Format,
) -> Result<String, ConversionError> {
    println!(
        "\n---> Converting from {} to {}...",
        input_format, output_format
    );
    // Step 1: Deserialize input string into our common Rust structure (Vec<Record>)
    let records = deserialize_from_string(input_string, input_format)?;
    println!("Deserialized Records: {:?}", records); // Optional: print intermediate struct

    // Step 2: Serialize the Rust structure into the target output string format
    let output_string = serialize_to_string(&records, output_format)?;
    Ok(output_string)
}

// --- Example Usage ---
fn main() -> Result<(), Box<dyn Error>> {
    // --- Sample Data ---
    // Define the initial data as a JSON string

    // let initial_json = r#"[
    //     { "id": 1, "name": "Alice", "value": 12.34, "active": true },
    //     { "id": 2, "name": "Bob", "value": 56.78, "active": false },
    //     { "id": 3, "name": "Charlie", "value": 99.0, "active": true }
    // ]"#;

    let initial_json = r#"[
        {
            "id": 1,
            "name": "Alice",
            "value": 12.34,
            "active": true,
            "child": [
                { "id": 1, "name": "Alice", "value": 12.34, "active": true },
                { "id": 2, "name": "Bob", "value": 56.78, "active": false },
                { "id": 3, "name": "Charlie", "value": 99.0, "active": true }
            ]
        },
        { "id": 2, "name": "Bob", "value": 56.78, "active": false },
        { "id": 3, "name": "Charlie", "value": 99.0, "active": true }
    ]"#;

    println!("Initial JSON:\n{}", initial_json);

    // --- Circular Conversion ---

    // 1. JSON -> YAML
    let yaml_string = convert_data(initial_json, Format::Json, Format::Yaml)?;
    println!("Converted YAML:\n{}", yaml_string);

    // 2. YAML -> CSV
    let csv_string = convert_data(&yaml_string, Format::Yaml, Format::Csv)?;
    println!("Converted CSV:\n{}", csv_string);

    // 3. CSV -> TOML
    let toml_string = convert_data(&csv_string, Format::Csv, Format::Toml)?;
    println!("Converted TOML:\n{}", toml_string);

    // 4. TOML -> JSON
    let final_json_string = convert_data(&toml_string, Format::Toml, Format::Json)?;
    println!("Converted back to JSON:\n{}", final_json_string);

    // Optional: Verify the final JSON matches the initial structure (requires deserializing again)
    let initial_records: Vec<Record> = serde_json::from_str(initial_json)?;
    let final_records: Vec<Record> = serde_json::from_str(&final_json_string)?;
    assert_eq!(initial_records, final_records, "Data mismatch after full cycle!");
    println!("\n✅ Data matches after full conversion cycle!");


    Ok(())
}