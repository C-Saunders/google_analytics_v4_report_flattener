extern crate joinery;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate failure;
#[cfg(test)]
#[macro_use]
extern crate indoc;
#[macro_use]
extern crate serde_derive;

mod report_to_flat;
pub mod types;

use failure::Error;
use report_to_flat::get_flattened;
use types::{Report, ReportResponse};

pub fn to_delimited(raw_report: &String, delimiter: &str) -> Result<String, Error> {
    let empty_response = Ok("".to_string());

    if raw_report.is_empty() {
        return empty_response;
    }

    let parsed_report: ReportResponse = serde_json::from_str(raw_report)?;

    let report: &Report = match parsed_report.reports.len() {
        0 => return empty_response,
        1 => &parsed_report.reports[0],
        _ => bail!("Flattening multi-report responses is not supported."),
    };

    if report.data.rows.is_none() {
        return empty_response;
    }

    Ok(get_flattened(&report, &delimiter))
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

    #[test]
    fn no_dimensions() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/no_dimensions.json"
        ))).unwrap();

        assert_eq!(
            to_delimited(&data, ",").unwrap(),
            "\"ga:sessions\"\n44\n".to_string()
        )
    }

    #[test]
    fn single_dimension_and_metric() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/single_dimension_and_metric.json"
        ))).unwrap();

        assert_eq!(
            to_delimited(&data, "|delimiter|").unwrap(),
            indoc!(
                r#""ga:deviceCategory"|delimiter|"ga:sessions"
                "desktop"|delimiter|43
                "mobile"|delimiter|1
                "#
            ).to_string()
        )
    }

    #[test]
    fn multiple_dimensions_and_metrics() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/multiple_dimensions_and_metrics.json"
        ))).unwrap();

        assert_eq!(
            to_delimited(&data, ",").unwrap(),
            indoc!(
                r#""ga:deviceCategory","ga:country","ga:sessions","ga:bounces"
                "desktop","Australia",1,1
                "desktop","France",39,21
                "desktop","United States",3,1
                "mobile","Brazil",1,0
                "#
            ).to_string()
        )
    }
}
