#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MedicationsNoMapped {
  pub medication_no_mapped_id: i64,
  pub practiceid: i64,
  pub pcc_description: String,
  pub rxnorm: String,
  pub updated_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
