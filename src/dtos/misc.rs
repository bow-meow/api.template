use base64::{alphabet::STANDARD, engine::{self, general_purpose::PAD}, Engine};
use chrono::{DateTime, NaiveDateTime, Utc};
use tiberius::{numeric::Decimal, ColumnType, Row, Uuid};

pub fn sql_to_string(i: usize, column_type: ColumnType, row: &Row) -> Option<String> {
        match column_type {
            ColumnType::Bitn |
            ColumnType::Bit => row
                .try_get::<bool, _>(i)
                .ok()
                .flatten()
                .map(|b| b.to_string()),
            ColumnType::Int1 => row
                .try_get::<u8, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Int2 => row
                .try_get::<i16, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Int4 => row
                .try_get::<i32, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Intn => row
                .try_get::<i64, _>(i)
                .or_else(|_| {
                    row.try_get::<i32, _>(i)
                        .map(|v| v.map(|i| i as i64))
                })
                .or_else(|_| {
                    row.try_get::<i16, _>(i)
                        .map(|v| v.map(|i| i as i64))
                })
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Int8 => row
                .try_get::<i64, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Float4 => row
                .try_get::<f32, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Float8 | ColumnType::Floatn => row
                .try_get::<f64, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Guid => row
                .try_get::<Uuid, _>(i)
                .ok()
                .flatten()
                .map(|uuid| uuid.to_string()),
            ColumnType::NVarchar
            | ColumnType::NChar
            | ColumnType::BigVarChar
            | ColumnType::BigChar
            | ColumnType::Text
            | ColumnType::NText => row
                .try_get::<&str, _>(i)
                .ok()
                .flatten()
                .map(|s| s.to_string()),
            ColumnType::Numericn | ColumnType::Decimaln => row
                .try_get::<Decimal, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::Datetime
            | ColumnType::Datetime2
            | ColumnType::DatetimeOffsetn
            | ColumnType::Datetime4
            | ColumnType::Daten
            | ColumnType::Timen
            | ColumnType::Datetimen => row
                .try_get::<NaiveDateTime, _>(i)
                .ok()
                .flatten()
                .map(|dt|  DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).to_rfc3339().to_string()),
            ColumnType::Null => None,
            ColumnType::Money | ColumnType::Money4 => row
                .try_get::<f64, _>(i)
                .ok()
                .flatten()
                .map(|v| v.to_string()),
            ColumnType::BigVarBin | ColumnType::BigBinary | ColumnType::Image => row
                .try_get::<&[u8], _>(i)
                .ok()
                .flatten()
                .map(|bytes| engine::GeneralPurpose::new(&STANDARD, PAD).encode(bytes)),
            ColumnType::Xml | ColumnType::Udt => row
                .try_get::<&str, _>(i)
                .ok()
                .flatten()
                .map(|s| s.to_string()),
            ColumnType::SSVariant => {
                None
            }
        }
}