extern crate joinery;
extern crate serde;

#[macro_use]
extern crate failure;
#[cfg(test)]
#[macro_use]
extern crate indoc;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod report_to_flat;
mod response_to_row_array;
pub mod types;

use failure::Error;
use report_to_flat::report_to_flat;
use response_to_row_array::response_to_row_array;
use serde_json::value::Value;
use types::{Report, ReportResponse};

pub fn to_delimited(raw_report_response: &str, delimiter: &str) -> Result<String, Error> {
    let empty_response = Ok("".to_string());

    if raw_report_response.is_empty() {
        return empty_response;
    }

    let parsed_report: ReportResponse = serde_json::from_str(raw_report_response)?;

    let report: &Report = match parsed_report.reports.len() {
        0 => return empty_response,
        1 => &parsed_report.reports[0],
        _ => bail!("Delimited output of multi-report responses is not supported."),
    };

    if report.is_empty() {
        return empty_response;
    }

    Ok(report_to_flat(&report, &delimiter))
}

pub fn to_flat_json(raw_report: &str) -> Result<Value, Error> {
    if raw_report.is_empty() {
        return Ok(json!("[]"));
    }

    let deserialized_response: ReportResponse = serde_json::from_str(raw_report)?;

    Ok(response_to_row_array(&deserialized_response))
}

#[cfg(test)]
mod tests {
    use super::to_delimited;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn empty_raw_report() {
        assert_eq!(to_delimited(&("".to_string()), ",").unwrap(), "")
    }

    #[test]
    fn rejects_reports_containing_unsupported_features() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/unsupported_feature.json"
        ))).unwrap();

        assert!(to_delimited(&data, ",").is_err())
    }

    #[test]
    fn rejects_multiple_reports() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/multiple_reports.json"
        ))).unwrap();

        assert!(to_delimited(&data, ",").is_err())
    }

    #[test]
    fn no_rows() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/no_rows.json"
        ))).unwrap();

        assert_eq!(to_delimited(&data, ",").unwrap(), "".to_string())
    }
}
