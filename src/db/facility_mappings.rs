#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct FacilityMappings {
  pub facility_mapping_id: i64,
  pub cybx_customer_code: String,
  pub facility_name: String,
  pub athena_practice_id: i64,
  pub athena_department_id: i64,
  pub pcc_organization_id: uuid::Uuid,
  pub pcc_facility_id: i64,
  pub facility_status: String,
  pub time_zone: Option<String>,
  pub interfaces: Option<String>,
  pub athena_department_name: Option<String>,
  pub pcc_facility_name: Option<String>,
  pub practitioner_filtering: String,
  pub demographics: Option<String>,
  pub diagnostic_reports: Option<String>,
}
