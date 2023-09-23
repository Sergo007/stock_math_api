#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ProgressNotesTypesFromPccToAthena {
  pub progress_notes_type_id: i64,
  pub cybx_customer_code: String,
  pub athena_practice_id: i64,
  pub pcc_progress_note_type: String,
  pub status: String,
}
