use serde_json::value::Value;
use serde_json::Map;
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
    if report_rows.is_none() {
        return json!("[]");
    }

    let dimension_headers = report.column_header.dimensions.as_ref();
    let metric_header_iter = report.get_metric_header_iterator();

    let result = report_rows
        .as_ref()
        .unwrap()
        .iter()
        .map(|row| {
            let mut current: Map<String, Value> = Map::new();

            if dimension_headers.is_some() {
                let dimension_data = row.dimensions.as_ref().unwrap();
                for (i, header) in dimension_headers.unwrap().iter().enumerate() {
                    current.insert(
                        header.to_string(),
                        Value::String(dimension_data[i].to_string()),
                    );
                }
            }

            for (header, value) in metric_header_iter.clone().zip(row.metrics[0].values.iter()) {
                current.insert(header.name.to_string(), Value::String(value.to_string()));
            }

            Value::Object(current)
        })
        .collect();

    Value::Array(result)
}

#[cfg(test)]
mod tests {
    use super::response_to_row_array;
    use serde_json;
    use std::fs;
    use std::path::PathBuf;
    use types::ReportResponse;

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
            json!([[{"ga:sessions": "44"}]])
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
                    "ga:sessions": "43",
                },
                {
                    "ga:deviceCategory": "mobile",
                    "ga:sessions": "1",
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
                    "ga:sessions": "1",
                    "ga:bounces": "1",
                }, {
                    "ga:deviceCategory": "desktop",
                    "ga:country": "France",
                    "ga:sessions": "39",
                    "ga:bounces": "21",
                }, {
                    "ga:deviceCategory": "desktop",
                    "ga:country": "United States",
                    "ga:sessions": "3",
                    "ga:bounces": "1",
                }, {
                    "ga:deviceCategory": "mobile",
                    "ga:country": "Brazil",
                    "ga:sessions": "1",
                    "ga:bounces": "0",
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
                  "ga:sessions": "25",
                  "ga:bounces": "17",
                }, {
                  "ga:deviceCategory": "mobile",
                  "ga:sessions": "2",
                  "ga:bounces": "2",
                }],
                [{
                  "ga:country": "Azerbaijan",
                  "ga:sessions": "1",
                  "ga:bounces": "0",
                }, {
                  "ga:country": "France",
                  "ga:sessions": "18",
                  "ga:bounces": "11",
                }, {
                  "ga:country": "Japan",
                  "ga:sessions": "4",
                  "ga:bounces": "4",
                }, {
                  "ga:country": "Switzerland",
                  "ga:sessions": "1",
                  "ga:bounces": "1",
                }, {
                  "ga:country": "United States",
                  "ga:sessions": "3",
                  "ga:bounces": "3",
                }]
            ])
        )
    }
}
