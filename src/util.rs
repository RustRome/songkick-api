use url::percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET};

use url::define_encode_set;

define_encode_set! {
    pub SK_ENCODE_SET = [QUERY_ENCODE_SET] | {'-'}
}

pub fn encode(src: &str) -> String {
    utf8_percent_encode(src, SK_ENCODE_SET).collect::<String>()
}

pub mod json {
    use crate::error::SkError;
    use crate::SkResult;
    use serde_json::value::Map;
    use serde_json::Value;

    pub fn get_str(obj: &Map<String, Value>, field: &str) -> SkResult<String> {
        obj.get(field)
            .and_then(|val| val.as_str())
            .map(String::from)
            .ok_or(SkError::JsonError(format!(
                "Failed to deserialize JSON artist object: missing field {}",
                field
            )))
    }

    pub fn get_u64(obj: &Map<String, Value>, field: &str) -> SkResult<u64> {
        obj.get(field)
            .and_then(|val| val.as_u64())
            .ok_or(SkError::JsonError(format!(
                "Failed to deserialize JSON artist object: missing field {}",
                field
            )))
    }

    pub fn get_f64(obj: &Map<String, Value>, field: &str) -> SkResult<f64> {
        obj.get(field)
            .and_then(|val| val.as_f64())
            .ok_or(SkError::JsonError(format!(
                "Failed to deserialize JSON artist object: missing field {}",
                field
            )))
    }

    pub fn get_arr<'a>(obj: &'a Map<String, Value>, field: &str) -> SkResult<&'a Vec<Value>> {
        obj.get(field)
            .and_then(|val| val.as_array())
            .ok_or(SkError::JsonError(format!(
                "Failed to deserialize JSON artist object: missing field {}",
                field
            )))
    }
}
