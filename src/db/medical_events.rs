#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MedicalEvents {
  pub medical_event_id: i64,
  pub webhook_id: String,
  pub facility_mapping_id: i64,
  pub pcc_patient_id: i64,
  pub interface_name: String,
  pub transaction_name: String,
  pub athena_requests_count: i64,
  pub athena_requests: String,
  pub updated_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
