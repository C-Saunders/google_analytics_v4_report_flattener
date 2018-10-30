This library exposes two public functions that accept a [Google Analytics Core Reporting v4 response string](https://developers.google.com/analytics/devguides/reporting/core/v4/samples) (string of the JSON data) and tranform the data into something easier to use.

### Report to delimited (e.g. TSV, CSV)

`to_delimited(raw_report_response: &str, delimiter: &str) -> Result<Vec<String>, serde_json::Error>`

Converts a report response to a collection of delimited reports. You can specify any delimiter string you'd like.

```
["\"ga:deviceCategory\",\"ga:sessions\"\n\"desktop\",43\n\"mobile\",1\n"]
```

### Report to flat JSON (flat array of row data)

`to_flat_json(raw_report: &str) -> Result<serde_json::value::Value, serde_json::Error>`

Converts a report response to an array of "flat JSON" responses.

```json
[
  [{
    "ga:deviceCategory": "desktop",
    "ga:sessions": 21
  }, {
    "ga:deviceCategory": "mobile",
    "ga:sessions": 84
  }],
  [{
    "ga:country": "Mexico",
    "ga:bounces": 9213
  }, {
    "ga:country": "Canada",
    "ga:bounces": 407
  }]
]
```

### Limitations
* [Pivots](https://developers.google.com/analytics/devguides/reporting/core/v4/samples#pivots) are not supported

## Contributing
Issues and pull requests welcome. Please be nice.

* Run tests with `cargo test`
* Run benchmarks with `cargo bench`
* Format with `rustfmt`

## License
[MIT](https://opensource.org/licenses/MIT)
