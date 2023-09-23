#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct DiagnosisMappings {
  pub diagnosis_mapping_id: i64,
  pub pcc_condition_id: i64,
  pub snomed_code: Option<i64>,
  pub patient_mapping_id: i64,
  pub athena_problem_id: i64,
}
