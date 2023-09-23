#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MedicationsIdsRxnormMappings {
  pub medications_ids_rxnorm_mappings_id: i64,
  pub practiceid: i64,
  pub medicationid: i64,
  pub athena_description: String,
  pub rxnorm: String,
  pub updated_at: Option<chrono::DateTime<chrono::Utc>/*timestamp*/>,
}
