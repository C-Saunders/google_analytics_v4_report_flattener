use serde_json::value::{Number, Value};
use serde_json::Map;
use std::slice::Iter;
use std::str::FromStr;
use types::*;

pub fn response_to_row_array(response: &ReportResponse) -> Value {
    response
        .reports
        .iter()
        .map(|report| report_to_row_array(&report))
        .collect()
}

fn report_to_row_array(report: &Report) -> Value {
    let report_rows = &report.data.rows;
    if report_rows.is_empty() {
        return json!([]);
    }

    let dimension_headers = &report.column_header.dimensions;
    let metric_headers = report.get_metric_headers();

    let result = report_rows
        .iter()
        .map(|row| {
            let mut current: Map<String, Value> = Map::new();

            insert_dimension_data(&mut current, row, dimension_headers);
            insert_metric_data(&mut current, row, &metric_headers.iter());

            Value::Object(current)
        }).collect();

    Value::Array(result)
}

fn insert_dimension_data(
    current: &mut Map<String, Value>,
    row: &ReportRow,
    dimension_headers: &[String],
) {
    for (i, header) in dimension_headers.iter().enumerate() {
        current.insert(
            header.to_string(),
            Value::String(row.dimensions[i].to_string()),
        );
    }
}

fn insert_metric_data(
    current: &mut Map<String, Value>,
    row: &ReportRow,
    metric_header_iter: &Iter<MetricHeaderEntry>,
) {
    let value_iterator = row.flat_value_iterator();

    for (header, value) in metric_header_iter.clone().zip(value_iterator) {
        current.insert(
            header.name.clone(),
            Value::Number(Number::from_str(value).unwrap()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::response_to_row_array;
    use serde_json;
    use std::fs;
    use std::path::PathBuf;
    use types::ReportResponse;

    #[test]
    fn no_rows() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/no_rows.json"
        ))).unwrap();

        let parsed_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(response_to_row_array(&parsed_response), json!([[]]))
    }

    #[test]
    fn no_dimensions() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/no_dimensions.json"
        ))).unwrap();

        let parsed_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_row_array(&parsed_response),
            json!([[{"ga:sessions": 44}]])
        )
    }

    #[test]
    fn single_dimension_and_metric() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/single_dimension_and_metric.json"
        ))).unwrap();

        let parsed_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_row_array(&parsed_response),
            json!([
                [{
                    "ga:deviceCategory": "desktop",
                    "ga:sessions": 43,
                },
                {
                    "ga:deviceCategory": "mobile",
                    "ga:sessions": 1,
                }]
            ])
        )
    }

    #[test]
    fn multiple_dimensions_and_metrics() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/multiple_dimensions_and_metrics.json"
        ))).unwrap();

        let parsed_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_row_array(&parsed_response),
            json!([
                [{
                    "ga:deviceCategory": "desktop",
                    "ga:country": "Australia",
                    "ga:sessions": 1,
                    "ga:bounces": 1,
                }, {
                    "ga:deviceCategory": "desktop",
                    "ga:country": "France",
                    "ga:sessions": 39,
                    "ga:bounces": 21,
                }, {
                    "ga:deviceCategory": "desktop",
                    "ga:country": "United States",
                    "ga:sessions": 3,
                    "ga:bounces": 1,
                }, {
                    "ga:deviceCategory": "mobile",
                    "ga:country": "Brazil",
                    "ga:sessions": 1,
                    "ga:bounces": 0,
                }]
            ])
        )
    }

    #[test]
    fn large_report() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/large_report.json"
        ))).unwrap();

        let parsed_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert!(response_to_row_array(&parsed_response).as_array().is_some())
    }

    #[test]
    fn multiple_reports() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/multiple_reports.json"
        ))).unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_row_array(&deserialized_response),
            json!([
                [{
                  "ga:deviceCategory": "desktop",
                  "ga:sessions": 25,
                  "ga:bounces": 17,
                }, {
                  "ga:deviceCategory": "mobile",
                  "ga:sessions": 2,
                  "ga:bounces": 2,
                }],
                [{
                  "ga:country": "Azerbaijan",
                  "ga:sessions": 1,
                  "ga:bounces": 0,
                }, {
                  "ga:country": "France",
                  "ga:sessions": 18,
                  "ga:bounces": 11,
                }, {
                  "ga:country": "Japan",
                  "ga:sessions": 4,
                  "ga:bounces": 4,
                }, {
                  "ga:country": "Switzerland",
                  "ga:sessions": 1,
                  "ga:bounces": 1,
                }, {
                  "ga:country": "United States",
                  "ga:sessions": 3,
                  "ga:bounces": 3,
                }]
            ])
        )
    }

    #[test]
    fn multiple_date_ranges() {
        let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/multiple_date_ranges.json"
        ))).unwrap();

        let parsed_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_row_array(&parsed_response),
            json!([
                [{
                    "ga:browser": "Chrome",
                    "ga:avgTimeOnPage": 108.1733,
                    "ga:pageviewsPerSession": 2.93126,
                    "ga:avgTimeOnPage_2": 129.7071651,
                    "ga:pageviewsPerSession_2": 3.60975609,
                }, {
                    "ga:browser": "Edge",
                    "ga:avgTimeOnPage": 51.794117,
                    "ga:pageviewsPerSession": 6.6666667,
                    "ga:avgTimeOnPage_2": 210.866667,
                    "ga:pageviewsPerSession_2": 2.875,
                }, {
                    "ga:browser": "Firefox",
                    "ga:avgTimeOnPage": 123.657142,
                    "ga:pageviewsPerSession": 2.09375,
                    "ga:avgTimeOnPage_2": 75.333333,
                    "ga:pageviewsPerSession_2": 1.5,
                }]
            ])
        )
    }
}
