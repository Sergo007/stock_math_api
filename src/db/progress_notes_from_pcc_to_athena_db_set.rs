#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ProgressNotesFromPccToAthena;

pub struct ProgressNotesFromPccToAthenaSet;

impl ProgressNotesFromPccToAthenaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ProgressNotesFromPccToAthena>> {
        query_as::<_, ProgressNotesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_from_pcc_to_athena""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_progress_note_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, progress_note_mapping_id: i64) -> Result<ProgressNotesFromPccToAthena> {
        query_as::<_, ProgressNotesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_from_pcc_to_athena" WHERE "progress_note_mapping_id" = $1"#)
            .bind(progress_note_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_progress_note_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, progress_note_mapping_id_list: Vec<i64>) -> Result<Vec<ProgressNotesFromPccToAthena>> {
        query_as::<_, ProgressNotesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_from_pcc_to_athena" WHERE "progress_note_mapping_id" = ANY($1)"#)
            .bind(progress_note_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_progress_note_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, progress_note_mapping_id: i64) -> Result<Option<ProgressNotesFromPccToAthena>> {
        query_as::<_, ProgressNotesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_from_pcc_to_athena" WHERE "progress_note_mapping_id" = $1"#)
            .bind(progress_note_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, progress_notes_from_pcc_to_athena: ProgressNotesFromPccToAthena) -> Result<ProgressNotesFromPccToAthena> {
        query_as::<_, ProgressNotesFromPccToAthena>(r#"INSERT INTO "progress_notes_from_pcc_to_athena" ("progress_note_mapping_id", "patient_mapping_id", "pcc_progress_note_id", "pcc_progress_note_type", "athena_progress_note_id") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(progress_notes_from_pcc_to_athena.progress_note_mapping_id)
            .bind(progress_notes_from_pcc_to_athena.patient_mapping_id)
            .bind(progress_notes_from_pcc_to_athena.pcc_progress_note_id)
            .bind(progress_notes_from_pcc_to_athena.pcc_progress_note_type)
            .bind(progress_notes_from_pcc_to_athena.athena_progress_note_id)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, progress_notes_from_pcc_to_athena: ProgressNotesFromPccToAthena) -> Result<ProgressNotesFromPccToAthena> {
        query_as::<_, ProgressNotesFromPccToAthena>(r#"UPDATE "progress_notes_from_pcc_to_athena" SET "patient_mapping_id" = $2, "pcc_progress_note_id" = $3, "pcc_progress_note_type" = $4, "athena_progress_note_id" = $5 WHERE "progress_note_mapping_id" = $1 RETURNING *;"#)
            .bind(progress_notes_from_pcc_to_athena.progress_note_mapping_id)
            .bind(progress_notes_from_pcc_to_athena.patient_mapping_id)
            .bind(progress_notes_from_pcc_to_athena.pcc_progress_note_id)
            .bind(progress_notes_from_pcc_to_athena.pcc_progress_note_type)
            .bind(progress_notes_from_pcc_to_athena.athena_progress_note_id)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."progress_notes_from_pcc_to_athena" WHERE "progress_note_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
