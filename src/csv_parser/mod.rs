use std::{error::Error, fs::File};

use chrono::NaiveDate;

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    CREDIT,
    DEBIT,
}

impl TransactionType {
    fn from_option_str(opt: Option<&str>) -> Result<TransactionType, String> {
        match opt {
            Some("DEBIT") => Ok(TransactionType::DEBIT),
            Some("CREDIT") => Ok(TransactionType::CREDIT),
            _ => Err(format!("Invalid transaction type provided: {:?}", opt)),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum TransactionCategory {
    Food,
    Utilities,
    Bills,
    Entertainment,
    Transportation,
    Healthcare,
    Education,
    AccountTransfers,
    Other,
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub transaction_type: TransactionType,
    pub date: NaiveDate,
    pub amount: f32,
    pub description: String,
    pub category: TransactionCategory,
}

impl Data {
    fn parse_date(str: Option<&str>) -> NaiveDate {
        match str {
            Some(str) => NaiveDate::parse_from_str(str, "%Y%m%d")
                .expect(format!("Attempted to parse date with NaiveDate: {:?}", str).as_str()),
            None => NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        }
    }
}

/// Extracts data from a CSV file
///
/// # Arguments
///
/// `file_path` - A string slice representing the path to the CSV file
///
/// # Returns
///
/// A `Result` containing `<Vec<Data>` if the operation is successful, or a boxed `dyn Error` trait object if an error occurs
///
/// # Example
/// ```
/// let file_path = "path/to/your/file.csv"
/// let contents = parse_csv(file_path);
/// ```
pub fn parse_csv(file_path: &str) -> Result<Vec<Data>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let vec: Vec<Data> = rdr
        .records()
        .filter_map(|result| match result {
            Ok(record) => {
                if record.iter().any(|field| !field.is_empty()) {
                    Some(Data {
                        transaction_type: TransactionType::from_option_str(record.get(1))
                            .expect("Expected TransactionType to be either 'DEBIT' or 'CREDIT'."),
                        date: Data::parse_date(record.get(2)),
                        amount: record.get(3).unwrap().parse::<f32>().unwrap_or(0.0),
                        description: record
                            .get(4)
                            .unwrap_or("N/A")
                            .to_string()
                            .trim()
                            .to_string(),
                        category: TransactionCategory::Other, // TODO: Use the correct category for the data. (use chatgpt api call to organize it for you)
                    })
                } else {
                    // Skip empty row (optional: log or handle empty row)
                    None
                }
            }
            Err(e) => {
                panic!("Error parsing CSV record: {e}")
            }
        })
        .collect();

    Ok(vec)
}

#[cfg(test)]
#[path = "./tests/test.rs"]
mod test;
