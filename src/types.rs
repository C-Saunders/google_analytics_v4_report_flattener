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
        self.data.rows.is_empty()
    }

    // we want sets of headers like x, y, x_2, y_2, ..., x_n, y_n
    // to match the shape of the data
    pub fn get_metric_headers(&self) -> Vec<MetricHeaderEntry> {
        let base_items = &self.column_header.metric_header.metric_header_entries;

        let mut result = base_items.clone();

        for date_range_num in 1..self.number_of_date_ranges() {
            for entry in base_items.iter() {
                let temp_entry = MetricHeaderEntry {
                    name: format!("{}_{}", &entry.name, date_range_num + 1),
                    metric_type: (*entry).metric_type.clone(),
                };
                result.push(temp_entry);
            }
        }

        result
    }

    fn number_of_date_ranges(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.data.rows[0].metrics.len()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnHeader {
    #[serde(default)]
    pub dimensions: Vec<String>,
    pub metric_header: MetricHeader,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetricHeader {
    pub metric_header_entries: Vec<MetricHeaderEntry>,
    // pivotHeaders: Vec<PivotHeader>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricHeaderEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub metric_type: MetricType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(default)]
    pub rows: Vec<ReportRow>,
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
    #[serde(default)]
    pub dimensions: Vec<String>,
    pub metrics: Vec<DateRangeValue>,
}

impl ReportRow {
    pub fn flat_value_iterator<'a>(&'a self) -> impl Iterator<Item = &String> {
        self.metrics
            .iter()
            .flat_map(|value: &'a DateRangeValue| value.values.iter())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateRangeValue {
    pub values: Vec<String>,
    // pivotValueRegions: Vec<PivotValueRegion>,
}
