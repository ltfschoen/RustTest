use log::{warn, info};
use thiserror::Error;
use chrono::{
    DateTime,
    Utc,
};

// https://dev.to/nathan20/how-to-handle-errors-in-rust-a-comprehensive-guide-1cco
#[derive(Debug, Error)]
#[error("dispatch error")]
struct DispatchError {
    #[from]
    source: ConvertI64ToU64ForDate,
}

#[derive(Debug, Error)]
#[error("Unable to convert i64 to u64 for date error")]
struct ConvertI64ToU64ForDate;

type Date = i64;
type Balance = i128;

struct BalanceForDate {
    date: Date,
    balance: Balance,
}

fn convert_i64_to_u64_in_milliseconds(date: i64) -> Result<u64, DispatchError> {
    let date_as_u64_millis: u64;
    if let Some(_date_as_u64) = TryInto::<u64>::try_into(date).ok() {
        date_as_u64_millis = _date_as_u64;
    } else {
        return Err(DispatchError { source: ConvertI64ToU64ForDate });
    }
    return Ok(date_as_u64_millis);
}

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let now = utc.to_rfc3339();
    let utc_millis: i64 = utc.timestamp_millis();
    println!("{:?}", utc_millis);
    let _requested_date_as_u64 = convert_i64_to_u64_in_milliseconds(utc_millis.clone());
    let requested_date_as_u64;
    match _requested_date_as_u64 {
        Err(_e) => {
            log::error!("Unable to convert i64 to u64 in millis for timestamp {:?}", _e);
            return ();
        },
        Ok(ref x) => {
            requested_date_as_u64 = x;
        }
    }
    println!("{:?}", requested_date_as_u64);
}