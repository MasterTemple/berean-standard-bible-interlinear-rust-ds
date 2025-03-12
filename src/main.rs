pub mod interlinear;
pub mod greek;
pub mod hebrew_parsing;

use calamine::{deserialize_as_f64_or_none, open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    metric: String,
    #[serde(deserialize_with = "deserialize_as_f64_or_none")]
    value: Option<f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let path = format!("{}/tests/excel.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut excel: Xlsx<_> = open_workbook("/home/dgmastertemple/Documents/bsb/bsb_tables_greek.xlsx")?;

    let range = excel
        .worksheet_range("biblosinterlinear96")
        .map_err(|_| calamine::Error::Msg("Cannot find Sheet1"))?;

    let iter_records =
        RangeDeserializerBuilder::with_headers(&["metric", "value"]).from_range(&range)?;

    for result in iter_records {
        let record: Record = result?;
        println!("metric={:?}, value={:?}", record.metric, record.value);
    }

    Ok(())
}
