#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ProgressNotesTypesFromPccToAthena;

pub struct ProgressNotesTypesFromPccToAthenaSet;

impl ProgressNotesTypesFromPccToAthenaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ProgressNotesTypesFromPccToAthena>> {
        query_as::<_, ProgressNotesTypesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_types_from_pcc_to_athena""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_progress_notes_type_id<'e, E: PgExecutor<'e>>(&self, executor: E, progress_notes_type_id: i64) -> Result<ProgressNotesTypesFromPccToAthena> {
        query_as::<_, ProgressNotesTypesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_types_from_pcc_to_athena" WHERE "progress_notes_type_id" = $1"#)
            .bind(progress_notes_type_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_progress_notes_type_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, progress_notes_type_id_list: Vec<i64>) -> Result<Vec<ProgressNotesTypesFromPccToAthena>> {
        query_as::<_, ProgressNotesTypesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_types_from_pcc_to_athena" WHERE "progress_notes_type_id" = ANY($1)"#)
            .bind(progress_notes_type_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_progress_notes_type_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, progress_notes_type_id: i64) -> Result<Option<ProgressNotesTypesFromPccToAthena>> {
        query_as::<_, ProgressNotesTypesFromPccToAthena>(r#"SELECT * FROM "pcc_athena_interop"."progress_notes_types_from_pcc_to_athena" WHERE "progress_notes_type_id" = $1"#)
            .bind(progress_notes_type_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, progress_notes_types_from_pcc_to_athena: ProgressNotesTypesFromPccToAthena) -> Result<ProgressNotesTypesFromPccToAthena> {
        query_as::<_, ProgressNotesTypesFromPccToAthena>(r#"INSERT INTO "progress_notes_types_from_pcc_to_athena" ("progress_notes_type_id", "cybx_customer_code", "athena_practice_id", "pcc_progress_note_type", "status") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(progress_notes_types_from_pcc_to_athena.progress_notes_type_id)
            .bind(progress_notes_types_from_pcc_to_athena.cybx_customer_code)
            .bind(progress_notes_types_from_pcc_to_athena.athena_practice_id)
            .bind(progress_notes_types_from_pcc_to_athena.pcc_progress_note_type)
            .bind(progress_notes_types_from_pcc_to_athena.status)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, progress_notes_types_from_pcc_to_athena: ProgressNotesTypesFromPccToAthena) -> Result<ProgressNotesTypesFromPccToAthena> {
        query_as::<_, ProgressNotesTypesFromPccToAthena>(r#"UPDATE "progress_notes_types_from_pcc_to_athena" SET "cybx_customer_code" = $2, "athena_practice_id" = $3, "pcc_progress_note_type" = $4, "status" = $5 WHERE "progress_notes_type_id" = $1 RETURNING *;"#)
            .bind(progress_notes_types_from_pcc_to_athena.progress_notes_type_id)
            .bind(progress_notes_types_from_pcc_to_athena.cybx_customer_code)
            .bind(progress_notes_types_from_pcc_to_athena.athena_practice_id)
            .bind(progress_notes_types_from_pcc_to_athena.pcc_progress_note_type)
            .bind(progress_notes_types_from_pcc_to_athena.status)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."progress_notes_types_from_pcc_to_athena" WHERE "progress_notes_type_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
