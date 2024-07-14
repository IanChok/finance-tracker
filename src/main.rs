mod csv_parser;
fn main() {
    csv_parser::parse_csv("assets/test_statement.csv").expect("Failed to parse CSV file");
}
