#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PccPatientStatusHistory {
  pub patient_status_history_id: i64,
  pub patient_id: String,
  pub organization_id: uuid::Uuid,
  pub facility_id: String,
  pub patient_omp_id: String,
  pub patient_status: String,
  pub patient_status_date_time: String,
  pub created_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
