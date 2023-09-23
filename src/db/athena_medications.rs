#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AthenaMedications {
  pub medicationid: i64,
  pub medicationname: String,
  pub rxnorm: String,
  pub ndc: String,
  pub updated_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
