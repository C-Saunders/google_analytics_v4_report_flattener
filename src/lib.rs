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

pub fn to_delimited(raw_report: &str, delimiter: &str) -> Result<String, Error> {
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

    #[test]
    fn empty_raw_report() {
        assert_eq!(to_delimited("", ",").unwrap(), "")
    }

    #[test]
    fn rejects_reports_containing_unsupported_features() {
        let data = r#"{
  "reports": [
    {
      "columnHeader": {
        "dimensions": [
          "ga:deviceCategory"
        ],
        "metricHeader": {
          "pivotHeaders": [
            {
              "pivotHeaderEntries": [
                {
                  "dimensionNames": [
                    "ga:yearWeek"
                  ],
                  "dimensionValues": [
                    "201831"
                  ],
                  "metric": {
                    "name": "ga:sessions",
                    "type": "INTEGER"
                  }
                }
              ],
              "totalPivotGroupsCount": 1
            }
          ]
        }
      },
      "data": {
        "rows": [
          {
            "dimensions": [
              "desktop"
            ],
            "metrics": [
              {
                "pivotValueRegions": [
                  {
                    "values": [
                      "43"
                    ]
                  }
                ]
              }
            ]
          },
          {
            "dimensions": [
              "mobile"
            ],
            "metrics": [
              {
                "pivotValueRegions": [
                  {
                    "values": [
                      "1"
                    ]
                  }
                ]
              }
            ]
          }
        ],
        "totals": [
          {
            "pivotValueRegions": [
              {
                "values": [
                  "44"
                ]
              }
            ]
          }
        ],
        "rowCount": 2,
        "minimums": [
          {
            "pivotValueRegions": [
              {
                "values": [
                  "1"
                ]
              }
            ]
          }
        ],
        "maximums": [
          {
            "pivotValueRegions": [
              {
                "values": [
                  "43"
                ]
              }
            ]
          }
        ],
        "isDataGolden": true
      }
    }
  ]
}
"#;

        assert!(to_delimited(data, ",").is_err())
    }

    #[test]
    fn rejects_multiple_reports() {
        let data = r#"{
  "reports": [
    {
      "columnHeader": {
        "dimensions": [
          "ga:deviceCategory"
        ],
        "metricHeader": {
          "metricHeaderEntries": [
            {
              "name": "ga:sessions",
              "type": "INTEGER"
            }
          ]
        }
      },
      "data": {
        "rows": [
          {
            "dimensions": [
              "desktop"
            ],
            "metrics": [
              {
                "values": [
                  "43"
                ]
              }
            ]
          },
          {
            "dimensions": [
              "mobile"
            ],
            "metrics": [
              {
                "values": [
                  "1"
                ]
              }
            ]
          }
        ],
        "totals": [
          {
            "values": [
              "44"
            ]
          }
        ],
        "rowCount": 2,
        "minimums": [
          {
            "values": [
              "1"
            ]
          }
        ],
        "maximums": [
          {
            "values": [
              "43"
            ]
          }
        ],
        "isDataGolden": true
      }
    },
    {
      "columnHeader": {
        "dimensions": [
          "ga:deviceCategory"
        ],
        "metricHeader": {
          "pivotHeaders": [
            {
              "pivotHeaderEntries": [
                {
                  "dimensionNames": [
                    "ga:yearWeek"
                  ],
                  "dimensionValues": [
                    "201831"
                  ],
                  "metric": {
                    "name": "ga:sessions",
                    "type": "INTEGER"
                  }
                }
              ],
              "totalPivotGroupsCount": 1
            }
          ]
        }
      },
      "data": {
        "rows": [
          {
            "dimensions": [
              "desktop"
            ],
            "metrics": [
              {
                "pivotValueRegions": [
                  {
                    "values": [
                      "43"
                    ]
                  }
                ]
              }
            ]
          },
          {
            "dimensions": [
              "mobile"
            ],
            "metrics": [
              {
                "pivotValueRegions": [
                  {
                    "values": [
                      "1"
                    ]
                  }
                ]
              }
            ]
          }
        ],
        "totals": [
          {
            "pivotValueRegions": [
              {
                "values": [
                  "44"
                ]
              }
            ]
          }
        ],
        "rowCount": 2,
        "minimums": [
          {
            "pivotValueRegions": [
              {
                "values": [
                  "1"
                ]
              }
            ]
          }
        ],
        "maximums": [
          {
            "pivotValueRegions": [
              {
                "values": [
                  "43"
                ]
              }
            ]
          }
        ],
        "isDataGolden": true
      }
    }
  ]
}
"#;

        assert!(to_delimited(data, ",").is_err())
    }

    #[test]
    fn no_rows() {
        let data = r#"{
  "reports": [
    {
      "columnHeader": {
        "dimensions": [
          "ga:deviceCategory"
        ],
        "metricHeader": {
          "metricHeaderEntries": [
            {
              "name": "ga:sessions",
              "type": "INTEGER"
            }
          ]
        }
      },
      "data": {
        "totals": [
          {
            "values": [
              "0"
            ]
          }
        ],
        "isDataGolden": true
      }
    }
  ]
}
"#;
        assert_eq!(to_delimited(data, ",").unwrap(), "".to_string())
    }

    #[test]
    fn no_dimensions() {
        let data = r#"{
  "reports": [
    {
      "columnHeader": {
        "metricHeader": {
          "metricHeaderEntries": [
            {
              "name": "ga:sessions",
              "type": "INTEGER"
            }
          ]
        }
      },
      "data": {
        "rows": [
          {
            "metrics": [
              {
                "values": [
                  "44"
                ]
              }
            ]
          }
        ],
        "totals": [
          {
            "values": [
              "44"
            ]
          }
        ],
        "rowCount": 1,
        "minimums": [
          {
            "values": [
              "44"
            ]
          }
        ],
        "maximums": [
          {
            "values": [
              "44"
            ]
          }
        ],
        "isDataGolden": true
      }
    }
  ]
}"#;
        assert_eq!(
            to_delimited(data, ",").unwrap(),
            "\"ga:sessions\"\n44\n".to_string()
        )
    }

    #[test]
    fn single_dimension_and_metric() {
        let data = r#"{
  "reports": [
    {
      "columnHeader": {
        "dimensions": [
          "ga:deviceCategory"
        ],
        "metricHeader": {
          "metricHeaderEntries": [
            {
              "name": "ga:sessions",
              "type": "INTEGER"
            }
          ]
        }
      },
      "data": {
        "rows": [
          {
            "dimensions": ["desktop"],
            "metrics": [{"values": ["43"]}]
          },
          {
            "dimensions": ["mobile"],
            "metrics": [{"values": ["1"]}]
          }
        ],
        "totals": [{"values": ["44"]}],
        "rowCount": 2,
        "minimums": [{"values": ["1"]}],
        "maximums": [{"values": ["43"]}],
        "isDataGolden": true
      }
    }
  ]
}"#;
        assert_eq!(
            to_delimited(data, ",").unwrap(),
            "\"ga:deviceCategory\",\"ga:sessions\"\n\"desktop\",43\n\"mobile\",1\n".to_string()
        )
    }
}
