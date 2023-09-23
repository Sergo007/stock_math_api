#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PatientMappings {
  pub patient_mapping_id: i64,
  pub facility_mapping_id: i64,
  pub pcc_patient_id: i64,
  pub athena_patient_id: i64,
  pub athena_practice_id: i64,
  pub pcc_patient_status: Option<String>,
  pub athena_current_department_id: Option<String>,
  pub pcc_patient_omp_id: Option<i64>,
  pub pcc_patient_admission_date_time: Option<String>,
  pub pcc_patient_discharge_date_time: Option<String>,
  pub created_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
