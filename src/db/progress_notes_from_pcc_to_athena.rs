#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ProgressNotesFromPccToAthena {
  pub progress_note_mapping_id: i64,
  pub patient_mapping_id: i64,
  pub pcc_progress_note_id: i64,
  pub pcc_progress_note_type: Option<String>,
  pub athena_progress_note_id: i64,
}
