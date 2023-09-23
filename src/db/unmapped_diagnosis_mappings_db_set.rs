#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::UnmappedDiagnosisMappings;

pub struct UnmappedDiagnosisMappingsSet;

impl UnmappedDiagnosisMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<UnmappedDiagnosisMappings>> {
        query_as::<_, UnmappedDiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."unmapped_diagnosis_mappings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_unmapped_diagnosis_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, unmapped_diagnosis_mapping_id: i64) -> Result<UnmappedDiagnosisMappings> {
        query_as::<_, UnmappedDiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."unmapped_diagnosis_mappings" WHERE "unmapped_diagnosis_mapping_id" = $1"#)
            .bind(unmapped_diagnosis_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_unmapped_diagnosis_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, unmapped_diagnosis_mapping_id_list: Vec<i64>) -> Result<Vec<UnmappedDiagnosisMappings>> {
        query_as::<_, UnmappedDiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."unmapped_diagnosis_mappings" WHERE "unmapped_diagnosis_mapping_id" = ANY($1)"#)
            .bind(unmapped_diagnosis_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_unmapped_diagnosis_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, unmapped_diagnosis_mapping_id: i64) -> Result<Option<UnmappedDiagnosisMappings>> {
        query_as::<_, UnmappedDiagnosisMappings>(r#"SELECT * FROM "pcc_athena_interop"."unmapped_diagnosis_mappings" WHERE "unmapped_diagnosis_mapping_id" = $1"#)
            .bind(unmapped_diagnosis_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, unmapped_diagnosis_mappings: UnmappedDiagnosisMappings) -> Result<UnmappedDiagnosisMappings> {
        query_as::<_, UnmappedDiagnosisMappings>(r#"INSERT INTO "unmapped_diagnosis_mappings" ("unmapped_diagnosis_mapping_id", "patient_mapping_id", "pcc_condition_id", "pcc_condition_snomed_code", "pcc_condition_icd10", "pcc_condition_onset_date", "pcc_condition_resolve_date", "created_at") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(unmapped_diagnosis_mappings.unmapped_diagnosis_mapping_id)
            .bind(unmapped_diagnosis_mappings.patient_mapping_id)
            .bind(unmapped_diagnosis_mappings.pcc_condition_id)
            .bind(unmapped_diagnosis_mappings.pcc_condition_snomed_code)
            .bind(unmapped_diagnosis_mappings.pcc_condition_icd10)
            .bind(unmapped_diagnosis_mappings.pcc_condition_onset_date)
            .bind(unmapped_diagnosis_mappings.pcc_condition_resolve_date)
            .bind(unmapped_diagnosis_mappings.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, unmapped_diagnosis_mappings: UnmappedDiagnosisMappings) -> Result<UnmappedDiagnosisMappings> {
        query_as::<_, UnmappedDiagnosisMappings>(r#"UPDATE "unmapped_diagnosis_mappings" SET "patient_mapping_id" = $2, "pcc_condition_id" = $3, "pcc_condition_snomed_code" = $4, "pcc_condition_icd10" = $5, "pcc_condition_onset_date" = $6, "pcc_condition_resolve_date" = $7, "created_at" = $8 WHERE "unmapped_diagnosis_mapping_id" = $1 RETURNING *;"#)
            .bind(unmapped_diagnosis_mappings.unmapped_diagnosis_mapping_id)
            .bind(unmapped_diagnosis_mappings.patient_mapping_id)
            .bind(unmapped_diagnosis_mappings.pcc_condition_id)
            .bind(unmapped_diagnosis_mappings.pcc_condition_snomed_code)
            .bind(unmapped_diagnosis_mappings.pcc_condition_icd10)
            .bind(unmapped_diagnosis_mappings.pcc_condition_onset_date)
            .bind(unmapped_diagnosis_mappings.pcc_condition_resolve_date)
            .bind(unmapped_diagnosis_mappings.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."unmapped_diagnosis_mappings" WHERE "unmapped_diagnosis_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
