use csv::Writer;
use fake::faker;
use fake::Fake;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

const FILE_PATH: &str = "large_file.csv";
const ROWS: usize = 25_000_000; // Approx. 10GB
const CHUNK_SIZE: usize = 1000; // Rows per batch
const LOG_INTERVAL: usize = 250 * 1024 * 1024; // Log every 250MB
const ESTIMATED_ROW_SIZE: usize = 400; // Approximate row size in bytes

fn generate_fake_data() -> Vec<String> {
    let sanitize = |s: String| s.replace("\n", " ").replace("\r", " "); // Remove newlines

    vec![
        sanitize(fake::faker::name::en::Name().fake()),
        sanitize(fake::faker::internet::en::SafeEmail().fake()),
        sanitize(fake::uuid::UUIDv4.fake()),
        sanitize(fake::faker::phone_number::en::PhoneNumber().fake()),
        sanitize(fake::faker::company::en::CompanyName().fake()),
        sanitize(faker::company::en::Buzzword().fake()),
        sanitize(fake::faker::lorem::en::Sentence(5..10).fake()),
        sanitize(fake::faker::time::en::Date().fake()),
    ]
}

fn main() {
    let start_time = Instant::now();
    let file = File::create(FILE_PATH).expect("Failed to create file");
    let mut writer = Writer::from_writer(BufWriter::new(file));

    // Write header
    writer
        .write_record(&[
            "Name", "Email", "City", "UUID", "Phone", "Company", "Job", "Sentence", "Date", "Bool",
        ])
        .expect("Failed to write header");

    let mut total_bytes_written: usize = 0;

    for batch in 0..(ROWS / CHUNK_SIZE) {
        let data: Vec<Vec<String>> = (0..CHUNK_SIZE).map(|_| generate_fake_data()).collect();

        for row in data {
            writer.write_record(&row).expect("Failed to write row");
        }

        // Estimate bytes written
        total_bytes_written += CHUNK_SIZE * ESTIMATED_ROW_SIZE;

        if total_bytes_written >= LOG_INTERVAL {
            println!(
                "Batch {}: Generated {:.2} MB...",
                batch + 1,
                (total_bytes_written as f64) / (1024.0 * 1024.0)
            );
            total_bytes_written = 0; // Reset counter
        }
    }

    writer.flush().expect("Failed to flush data");

    println!(
        "10GB CSV generated in {:.2} seconds",
        start_time.elapsed().as_secs_f64()
    );
}
