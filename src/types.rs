use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReportResponse {
    pub reports: Vec<Report>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub column_header: ColumnHeader,
    pub data: ReportData,
    pub next_page_token: Option<String>,
}

impl Report {
    pub fn is_empty(&self) -> bool {
        self.data.rows.is_none()
    }

    pub fn get_metric_header_iterator(&self) -> Iter<MetricHeaderEntry> {
        self.column_header
            .metric_header
            .metric_header_entries
            .iter()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnHeader {
    pub dimensions: Option<Vec<String>>,
    pub metric_header: MetricHeader,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetricHeader {
    pub metric_header_entries: Vec<MetricHeaderEntry>,
    // pivotHeaders: Vec<PivotHeader>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricHeaderEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub metric_type: MetricType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetricType {
    MetricTypeUnspecified,
    Integer,
    Float,
    Currency,
    Percent,
    Time,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportData {
    pub rows: Option<Vec<ReportRow>>,
    pub totals: Vec<DateRangeValue>,
    pub row_count: Option<u32>,
    pub minimums: Option<Vec<DateRangeValue>>,
    pub maximums: Option<Vec<DateRangeValue>>,
    pub samples_read_counts: Option<Vec<String>>,
    pub sampling_space_sizes: Option<Vec<String>>,
    pub is_data_golden: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportRow {
    pub dimensions: Option<Vec<String>>,
    pub metrics: Vec<DateRangeValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateRangeValue {
    pub values: Vec<String>,
    // pivotValueRegions: Vec<PivotValueRegion>,
}
