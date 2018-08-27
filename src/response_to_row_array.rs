use serde_json::value::Value;
use serde_json::Map;
use types::*;

pub fn response_to_row_array(response: &ReportResponse) -> Value {
    let mut result: Vec<Value> = Vec::with_capacity(response.reports.len());
    for report in response.reports.iter() {
        result.push(report_to_row_array(&report));
    }

    Value::Array(result)
}

fn report_to_row_array(report: &Report) -> Value {
    let report_rows = &report.data.rows;
    if report_rows.is_none() {
        return json!("[]");
    }

    let dimension_headers = report.column_header.dimensions.as_ref();
    let metric_header_iter = report.get_metric_header_iterator();
    let mut result: Vec<Value> = Vec::with_capacity(report_rows.as_ref().unwrap().len());

    for row in report_rows.as_ref().unwrap() {
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

        result.push(Value::Object(current));
    }

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
}
