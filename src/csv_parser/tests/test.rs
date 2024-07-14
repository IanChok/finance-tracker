use super::*;
use rstest::rstest;

const TEST_FILE_PATH: &str = "src/csv_parser/tests/test_statement.csv";

#[test]
fn test_parse_csv() {
    let contents = parse_csv(TEST_FILE_PATH);
    match contents {
        Ok(data) => {
            assert!(
                !data.is_empty(),
                "Expected 'Vec<Data>' to not be emtpy. Got 'Empty'"
            );
            assert_eq!(
                Data {
                    transaction_type: TransactionType::DEBIT,
                    date: NaiveDate::from_ymd_opt(2024, 6, 3).unwrap(),
                    amount: -1374.47,
                    description: String::from("[DS]BANK         MTG/HYP"),
                    category: TransactionCategory::Other
                },
                *data.get(0).unwrap()
            )
        }
        Err(e) => {
            panic!("parse_csv() failed: {e}")
        }
    }
}

#[rstest]
#[case(Some(String::from("20240610")))]
#[case(None)]
fn test_parse_date(#[case] date: Option<String>) {
    match date {
        Some(date) => assert_eq!(
            NaiveDate::from_ymd_opt(2024, 6, 10).unwrap(),
            Data::parse_date(Some(date.as_str()))
        ),
        None => assert_eq!(
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            Data::parse_date(date.as_deref())
        ),
    }
}

#[test]
#[should_panic(
    expected = "Attempted to parse date with NaiveDate: \"05/01/2024\": ParseError(Invalid)"
)]
fn test_parse_date_panic() {
    const INVALID_DATE_FORMAT: &str = "05/01/2024";
    Data::parse_date(Some(INVALID_DATE_FORMAT));
}
