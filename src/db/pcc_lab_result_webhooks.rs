#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PccLabResultWebhooks {
  pub message_id: String,
  pub org_uuid: String,
  pub patient_id: String,
  pub resource_id: String,
  pub event_type: String,
  pub updated_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
