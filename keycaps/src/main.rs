mod design;
mod keys;

fn main() -> Result<(), serde_json::Error> {
    serde_json::to_writer_pretty(std::io::stdout().lock(), &design::build_design())
}
