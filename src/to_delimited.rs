use crate::types::*;
use itertools::Itertools;

pub fn response_to_delimited_reports(response: &ReportResponse, delimiter: &str) -> Vec<String> {
    response
        .reports
        .iter()
        .map(|report| report_to_flat(&report, delimiter))
        .collect()
}

fn report_to_flat(report: &Report, delimiter: &str) -> String {
    let dimension_header_iter = report
        .column_header
        .dimensions
        .iter()
        .map(|entry| format!("\"{}\"", entry));

    let metric_headers = report.get_metric_headers();

    let metric_header_iter = metric_headers
        .iter()
        .map(|entry: &MetricHeaderEntry| format!("\"{}\"", entry.name));

    let mut result = format!(
        "{}\n",
        dimension_header_iter
            .chain(metric_header_iter)
            .join(delimiter)
            .to_string()
    );

    report.data.rows.iter().for_each(|report_row| {
        if !report_row.dimensions.is_empty() {
            result.push_str(
                report_row
                    .dimensions
                    .iter()
                    .map(|entry| format!("\"{}\"", entry))
                    .join(delimiter)
                    .to_string()
                    .as_str(),
            );
            result.push_str(delimiter);
        };

        let metric_data = report_row.flat_value_iterator().join(delimiter).to_string();

        result.push_str(format!("{}\n", metric_data).as_str());
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReportResponse;
    use serde_json;
    use std::fs;
    use std::path::Path;

    #[test]
    fn no_rows() {
        let data: String = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("test_reports/no_rows.json"),
        )
        .unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_delimited_reports(&deserialized_response, ","),
            vec!["\"ga:deviceCategory\",\"ga:sessions\"\n"]
        )
    }

    #[test]
    fn no_dimensions() {
        let data: String = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("test_reports/no_dimensions.json"),
        )
        .unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            report_to_flat(&deserialized_response.reports[0], ","),
            "\"ga:sessions\"\n44\n".to_string()
        )
    }

    #[test]
    fn single_dimension_and_metric() {
        let data: String = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("test_reports/single_dimension_and_metric.json"),
        )
        .unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            report_to_flat(&deserialized_response.reports[0], "|delimiter|"),
            indoc!(
                r#""ga:deviceCategory"|delimiter|"ga:sessions"
                "desktop"|delimiter|43
                "mobile"|delimiter|1
                "#
            )
            .to_string()
        )
    }

    #[test]
    fn multiple_dimensions_and_metrics() {
        let data: String = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("test_reports/multiple_dimensions_and_metrics.json"),
        )
        .unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            report_to_flat(&deserialized_response.reports[0], ","),
            indoc!(
                r#""ga:deviceCategory","ga:country","ga:sessions","ga:bounces"
                "desktop","Australia",1,1
                "desktop","France",39,21
                "desktop","United States",3,1
                "mobile","Brazil",1,0
                "#
            )
            .to_string()
        )
    }

    #[test]
    fn large_report() {
        let data: String = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("test_reports/large_report.json"),
        )
        .unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert!(report_to_flat(&deserialized_response.reports[0], ",").len() > 0)
    }

    #[test]
    fn multiple_reports() {
        let data: String = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("test_reports/multiple_reports.json"),
        )
        .unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_delimited_reports(&deserialized_response, ","),
            vec![
                indoc!(
                    r#""ga:deviceCategory","ga:sessions","ga:bounces"
                    "desktop",25,17
                    "mobile",2,2
                    "#
                )
                .to_string(),
                indoc!(
                    r#""ga:country","ga:sessions","ga:bounces"
                    "Azerbaijan",1,0
                    "France",18,11
                    "Japan",4,4
                    "Switzerland",1,1
                    "United States",3,3
                    "#
                )
                .to_string(),
            ]
        )
    }

    #[test]
    fn multiple_date_ranges() {
        let data: String = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("test_reports/multiple_date_ranges.json"),
        )
        .unwrap();

        let deserialized_response: ReportResponse = serde_json::from_str(data.as_str()).unwrap();

        assert_eq!(
            response_to_delimited_reports(&deserialized_response, ","),
            vec![
                indoc!(
                    r#""ga:browser","ga:avgTimeOnPage","ga:pageviewsPerSession","ga:avgTimeOnPage_2","ga:pageviewsPerSession_2"
                    "Chrome",108.1733,2.93126,129.7071651,3.60975609
                    "Edge",51.794117,6.6666667,210.866667,2.875
                    "Firefox",123.657142,2.09375,75.333333,1.5
                    "#
                ).to_string(),
            ]
        )
    }
}
