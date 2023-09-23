#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct UnmappedDiagnosisMappings {
  pub unmapped_diagnosis_mapping_id: i64,
  pub patient_mapping_id: i64,
  pub pcc_condition_id: i64,
  pub pcc_condition_snomed_code: Option<String>,
  pub pcc_condition_icd10: Option<String>,
  pub pcc_condition_onset_date: Option<String>,
  pub pcc_condition_resolve_date: Option<String>,
  pub created_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
