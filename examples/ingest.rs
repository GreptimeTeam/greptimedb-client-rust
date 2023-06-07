use greptimedb_client::api::v1::column::*;
use greptimedb_client::api::v1::*;
use greptimedb_client::{Client, Database, DEFAULT_SCHEMA_NAME};

#[tokio::main]
async fn main() {
    let greptimedb_endpoint =
        std::env::var("GREPTIMEDB_ENDPOINT").unwrap_or_else(|_| "localhost:4001".to_owned());
    let greptimedb_dbname =
        std::env::var("GREPTIMEDB_DBNAME").unwrap_or_else(|_| DEFAULT_SCHEMA_NAME.to_owned());

    let grpc_client = Client::with_urls(vec![&greptimedb_endpoint]);
    let client = Database::new_with_dbname(greptimedb_dbname, grpc_client);

    let result = client.insert(vec![generate_data()]).await;
    match result {
        Ok(rows) => {
            println!("Rows written: {rows}");
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    };
}

/// This function generates some random data and bundle them into a
/// `InsertRequest`.
///
/// Data structure:
///
/// - `ts`: a timestamp column
/// - `collector`: a tag column
/// - `temperature`: a value field of f32
/// - `humidity`: a value field of i32
///
fn generate_data() -> InsertRequest {
    // in this example we use fixed timestamps
    let timestamp_millis = vec![
        1686109527000,
        1686023127000,
        1685936727000,
        1686109527000,
        1686023127000,
        1685936727000,
    ];
    let collectors = vec!["c1", "c1", "c1", "c2", "c2", "c2"];
    let temp = vec![26.4, 29.3, 31.8, 20.4, 18.0, 19.2];
    let humidity = vec![15, 20, 13, 67, 74, 81];

    let rows = timestamp_millis.len();

    let columns = vec![
        // timestamp column: `ts`
        Column {
            column_name: "ts".to_owned(),
            values: Some(column::Values {
                ts_millisecond_values: timestamp_millis,
                ..Default::default()
            }),
            semantic_type: SemanticType::Timestamp as i32,
            datatype: ColumnDataType::TimestampMillisecond as i32,
            ..Default::default()
        },
        // tag column: collectors
        Column {
            column_name: "collector".to_owned(),
            values: Some(column::Values {
                string_values: collectors.into_iter().map(|s| s.to_owned()).collect(),
                ..Default::default()
            }),
            semantic_type: SemanticType::Tag as i32,
            datatype: ColumnDataType::String as i32,
            ..Default::default()
        },
        // field column: temperature
        Column {
            column_name: "temperature".to_owned(),
            values: Some(column::Values {
                f32_values: temp,
                ..Default::default()
            }),
            semantic_type: SemanticType::Field as i32,
            datatype: ColumnDataType::Float32 as i32,
            ..Default::default()
        },
        // field column: humidity
        Column {
            column_name: "humidity".to_owned(),
            values: Some(column::Values {
                i32_values: humidity,
                ..Default::default()
            }),
            semantic_type: SemanticType::Field as i32,
            datatype: ColumnDataType::Int32 as i32,
            ..Default::default()
        },
    ];

    InsertRequest {
        table_name: "weather_demo".to_owned(),
        columns,
        row_count: rows as u32,
        ..Default::default()
    }
}