use joinery::Joinable;
use types::*;

pub fn get_flattened(report: &Report, delimiter: &str) -> String {
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

    result
}
