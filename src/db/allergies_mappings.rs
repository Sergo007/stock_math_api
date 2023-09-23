#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AllergiesMappings {
  pub allergy_mapping_id: i64,
  pub patient_mapping_id: i64,
  pub pcc_allergy_intolerance_id: i64,
  pub allergen_name: String,
  pub athena_allergen_id: Option<i64>,
}
