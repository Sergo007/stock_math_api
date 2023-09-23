#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct DiagnosticReportsMappings {
  pub diagnostic_report_mapping_id: i64,
  pub patient_mapping_id: i64,
  pub pcc_diagnostic_report_id: String,
  pub athena_lab_result_id: i64,
  pub pcc_panel_name: String,
}
