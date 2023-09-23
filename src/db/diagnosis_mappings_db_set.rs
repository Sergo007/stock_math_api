#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::DiagnosisMappings;

pub struct DiagnosisMappingsSet;

impl DiagnosisMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<DiagnosisMappings>> {
        query_as::<_, DiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnosis_mappings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_diagnosis_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, diagnosis_mapping_id: i64) -> Result<DiagnosisMappings> {
        query_as::<_, DiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnosis_mappings" WHERE "diagnosis_mapping_id" = $1"#)
            .bind(diagnosis_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_diagnosis_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, diagnosis_mapping_id_list: Vec<i64>) -> Result<Vec<DiagnosisMappings>> {
        query_as::<_, DiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnosis_mappings" WHERE "diagnosis_mapping_id" = ANY($1)"#)
            .bind(diagnosis_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_diagnosis_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, diagnosis_mapping_id: i64) -> Result<Option<DiagnosisMappings>> {
        query_as::<_, DiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnosis_mappings" WHERE "diagnosis_mapping_id" = $1"#)
            .bind(diagnosis_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, diagnosis_mappings: DiagnosisMappings) -> Result<DiagnosisMappings> {
        query_as::<_, DiagnosisMappings>(r#"INSERT INTO "diagnosis_mappings" ("diagnosis_mapping_id", "pcc_condition_id", "snomed_code", "patient_mapping_id", "athena_problem_id") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(diagnosis_mappings.diagnosis_mapping_id)
            .bind(diagnosis_mappings.pcc_condition_id)
            .bind(diagnosis_mappings.snomed_code)
            .bind(diagnosis_mappings.patient_mapping_id)
            .bind(diagnosis_mappings.athena_problem_id)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, diagnosis_mappings: DiagnosisMappings) -> Result<DiagnosisMappings> {
        query_as::<_, DiagnosisMappings>(r#"UPDATE "diagnosis_mappings" SET "pcc_condition_id" = $2, "snomed_code" = $3, "patient_mapping_id" = $4, "athena_problem_id" = $5 WHERE "diagnosis_mapping_id" = $1 RETURNING *;"#)
            .bind(diagnosis_mappings.diagnosis_mapping_id)
            .bind(diagnosis_mappings.pcc_condition_id)
            .bind(diagnosis_mappings.snomed_code)
            .bind(diagnosis_mappings.patient_mapping_id)
            .bind(diagnosis_mappings.athena_problem_id)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."diagnosis_mappings" WHERE "diagnosis_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
