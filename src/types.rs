#[derive(Serialize, Deserialize, Debug)]
pub struct ReportResponse {
    pub reports: Vec<Report>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub column_header: ColumnHeader,
    pub data: ReportData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnHeader {
    pub dimensions: Vec<String>,
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
    pub rows: Vec<ReportRow>,
    pub totals: Vec<DateRangeValues>,
    pub row_count: u32,
    pub minimums: Vec<DateRangeValues>,
    pub maximums: Vec<DateRangeValues>,
    pub samples_read_counts: Option<Vec<String>>,
    pub sampling_space_sizes: Option<Vec<String>>,
    pub is_data_golden: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportRow {
    pub dimensions: Vec<String>,
    pub metrics: Vec<DateRangeValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateRangeValues {
    pub values: Vec<String>,
    // pivotValueRegions: Vec<PivotValueRegion>,
}
