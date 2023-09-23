#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Practitioners {
  pub practitioner_id: i64,
  pub cybx_customer_code: String,
  pub athena_practice_id: String,
  pub first_name: String,
  pub last_name: String,
  pub npi: Option<String>,
}
