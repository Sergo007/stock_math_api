#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MedicationMappings {
  pub medication_mapping_id: i64,
  pub pcc_order_id: i64,
  pub patient_mapping_id: i64,
  pub athena_medication_id: Option<i64>,
  pub athena_medicationentryid: Option<String>,
  pub medication_external_id: Option<String>,
}
