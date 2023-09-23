#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PccPanelNameToAthenaDocumentType {
  pub pcc_panel_to_athena_document_type_mapping_id: i64,
  pub pcc_panel_name: String,
  pub cybx_customer_code: String,
  pub athena_practice_id: String,
  pub athena_document_type_id: String,
  pub athena_document_type_name: String,
}
