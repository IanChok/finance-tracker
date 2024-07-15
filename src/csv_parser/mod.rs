use std::{error::Error, fs::File};

use chrono::NaiveDate;
use csv::ReaderBuilder;

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

#[allow(dead_code)]
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
                .expect(format!("Attempted to parse date with NaiveDate: {:?}. Expected the format to be '%Y%m%d', e.g., '20240601'", str).as_str()),
            None => NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        }
    }
}

trait CharExtensions {
    fn is_quote(self) -> bool;
}

impl CharExtensions for char {
    fn is_quote(self) -> bool {
        self == '\'' || self == '\"'
    }
}

/// Extracts data from a CSV file
///
/// # Arguments
///
/// `file_path` - A string slice representing the path to the CSV file.
///
/// # CSV Content
/// The CSV file is expected to _at least_ have these 5 fields in this exact order:
/// ```csv
/// First Bank Card, Transaction Type, Date Posted, Transaction Amount, Description
/// ```
///
/// # Returns
///
/// A `Result` containing `<Vec<Data>` if the operation is successful, or a boxed `dyn Error` trait object if an error occurs
///
/// # Example
///
/// *CSV*
/// ```csv
/// Following data is valid as of 20240714164814 (Year/Month/Day/Hour/Minute/Second)
///
///
/// First Bank Card,Transaction Type,Date Posted, Transaction Amount,Description
///
///
/// '6007620712733055',DEBIT,20240603,-1374.47,[DS]BANK         MTG/HYP                                                    
/// '6007620712733055',DEBIT,20240603,-231.97,[DS]STRATA FEE      
/// ```
///
/// *Code*
/// ```
/// let file_path = "path/to/your/file.csv"
/// let contents = parse_csv(file_path);
/// ```
pub fn parse_csv(file_path: &str) -> Result<Vec<Data>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

    let vec: Vec<Data> = rdr
        .records()
        .filter_map(|result| match result {
            Ok(record) => {
                let valid_record_len = record.len() >= 5;
                let valid_first_item = record.get(0)
                .unwrap_or("default")
                .chars()
                .all(|c| c.is_numeric() || c.is_quote());

                if !valid_record_len || !valid_first_item {
                    return None;
                }
                
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
