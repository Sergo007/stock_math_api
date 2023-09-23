#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PccMedicationsNoMapped {
  pub pcc_medication_no_mapped_id: i64,
  pub pcc_description: String,
  pub rxnorm: String,
  pub updated_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
