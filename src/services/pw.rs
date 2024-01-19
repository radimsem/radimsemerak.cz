use anyhow::{Result, Ok, bail};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

pub fn complete_db_uri(db_uri: &mut String, pw: String) -> Result<String> {
    let encoded_pw = utf8_percent_encode(pw.as_str(), NON_ALPHANUMERIC).to_string();

    let mid = db_uri.as_bytes().iter().position(|chr| *chr as char == '@');
    match mid {
        Some(idx) => db_uri.insert_str(idx, encoded_pw.as_str()),
        None => bail!("Database URI has invalid content!")
    }

    Ok(db_uri.to_string())
}