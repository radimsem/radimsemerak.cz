use std::sync::Arc;

use anyhow::{Result, bail};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use tokio::sync::Mutex;

use crate::error::AppError;

pub mod db;

pub struct ActionRequest<T: Default> {
    pub body: T,
    pub idr: IdentifierRequest 
}

#[derive(Default, PartialEq)]
pub struct IdentifierRequest {
    pub id: Option<i32>,
    pub action: IdentifierAction
}

#[derive(Default, PartialEq)]
pub enum IdentifierAction {
    #[default]
    UPDATE,
    DELETE
}

type SharedJob<T> = Arc<Mutex<Box<T>>>;
pub type FieldJob<T> = SharedJob<dyn Fn(&mut T, &Vec<u8>) -> Result<()> + Send + Sync>;
pub type ConstructJob<T, U> = SharedJob<dyn Fn(T) -> Result<U, AppError> + Send + Sync>;

pub fn complete_db_uri(db_uri: &mut String, pw: String) -> Result<String> {
    let encoded_pw = utf8_percent_encode(pw.as_str(), NON_ALPHANUMERIC).to_string();

    let mid: Option<usize> = db_uri.as_bytes().iter().position(|x: &u8| *x as char == '@');
    match mid {
        Some(idx) => db_uri.insert_str(idx, encoded_pw.as_str()),
        None => bail!("Database URI has invalid content!")
    }

    Ok(db_uri.to_string())
}