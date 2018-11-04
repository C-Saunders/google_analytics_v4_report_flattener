extern crate itertools;
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate indoc;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod to_delimited;
pub mod to_row_array;
pub mod types;

use crate::to_delimited::response_to_delimited_reports;
use crate::to_row_array::response_to_row_array;
use crate::types::ReportResponse;
use serde_json::value::Value;
use serde_json::Error;

pub fn to_delimited(raw_report_response: &str, delimiter: &str) -> Result<Vec<String>, Error> {
    if raw_report_response.is_empty() {
        return Ok(vec!["".to_string()]);
    }

    let deserialized_response: ReportResponse = serde_json::from_str(raw_report_response)?;

    Ok(response_to_delimited_reports(
        &deserialized_response,
        &delimiter,
    ))
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
    fn rejects_reports_containing_unsupported_features() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/unsupported_feature.json"
        )))
        .unwrap();

        assert!(to_delimited(&data, ",").is_err())
    }
}
