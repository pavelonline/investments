pub use chrono::NaiveDate as Date;
pub use chrono::NaiveDateTime as DateTime;
pub use rust_decimal::Decimal as Decimal;

macro_rules! decs {
    ($value:expr) => {{
        use ::std::str::FromStr;
        ::rust_decimal::Decimal::from_str($value).unwrap()
    }}
}

// TODO: Switch to macro provided by the crate
macro_rules! dec {
    ($value:expr) => (::rust_decimal::Decimal::from($value))
}

macro_rules! date {
    ($day:expr, $month:expr, $year:expr) => (::chrono::NaiveDate::from_ymd($year, $month, $day))
}