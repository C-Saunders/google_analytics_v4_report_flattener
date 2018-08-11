extern crate serde;
extern crate serde_json;
extern crate ga_v4_flattener;

use ga_v4_flattener::types::ReportResponse;

fn main() {
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
    }
  ]
}"#;
    
    let result: ReportResponse = serde_json::from_str(data).unwrap();

    println!("{:?}", result.reports[0]);
}
