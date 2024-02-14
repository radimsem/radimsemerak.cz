use anyhow::{Result, bail};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub mod db;

pub fn is_default<T>(val: &T) -> bool
where
    T: Default + PartialEq
{
    *val == T::default()
}

pub fn complete_db_uri(db_uri: &mut String, pw: String) -> Result<String> {
    let encoded_pw = utf8_percent_encode(pw.as_str(), NON_ALPHANUMERIC).to_string();

    let mid: Option<usize> = db_uri.as_bytes().iter().position(|x: &u8| *x as char == '@');
    match mid {
        Some(idx) => db_uri.insert_str(idx, encoded_pw.as_str()),
        None => bail!("Database URI has invalid content!")
    }

    Ok(db_uri.to_string())
}