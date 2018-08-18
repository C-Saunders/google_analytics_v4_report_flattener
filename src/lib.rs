extern crate joinery;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

pub mod types;

use failure::Error;
use joinery::Joinable;
use types::*;

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

    get_flattened(&report, &delimiter)
}

fn get_flattened(report: &Report, delimiter: &str) -> Result<String, Error> {
    let empty_vec = Vec::with_capacity(0);
    let dimension_header_iter = report.column_header.dimensions
        .as_ref()
        .unwrap_or(&empty_vec) // TODO; this is pretty ugly
        .iter()
        .map(|entry| format!("\"{}\"", entry));

    let metric_header_iter = report
        .column_header
        .metric_header
        .metric_header_entries
        .iter()
        .map(|entry: &MetricHeaderEntry| format!("\"{}\"", &entry.name));

    let mut result = format!(
        "{}\n",
        dimension_header_iter
            .chain(metric_header_iter)
            .join_with(delimiter)
            .to_string()
    );

    for report_row in report.data.rows.as_ref().unwrap().iter() {
        if let Some(ref dimensions) = report_row.dimensions {
            result.push_str(
                dimensions
                    .iter()
                    .map(|entry| format!("\"{}\"", entry))
                    .join_with(delimiter)
                    .to_string()
                    .as_str(),
            );
            result.push_str(delimiter);
        };
        let metric_data = report_row
            .metrics
            .iter()
            .flat_map(|date_range_value| date_range_value.values.iter())
            .join_with(delimiter)
            .to_string();

        result.push_str(format!("{}\n", metric_data).as_str());
    }

    Ok(result)
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
            r#""ga:deviceCategory"|delimiter|"ga:sessions"
"desktop"|delimiter|43
"mobile"|delimiter|1
"#.to_string()
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
            r#""ga:deviceCategory","ga:country","ga:sessions","ga:bounces"
"desktop","Australia",1,1
"desktop","France",39,21
"desktop","United States",3,1
"mobile","Brazil",1,0
"#.to_string()
        )
    }
}
