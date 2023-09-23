#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct FacilityOnboardings {
  pub facility_onboarding_id: i64,
  pub facility_mapping_id: i64,
  pub pcc_patients_ids: String,
  pub interfaces: Option<String>,
  pub all_pcc_patients_ids: String,
  pub loaded_pcc_patients_ids: String,
  pub onboarding_status: String,
}
