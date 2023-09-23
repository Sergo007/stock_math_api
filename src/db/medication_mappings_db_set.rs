#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use super::MedicationMappings;
use sqlx::{query, query_as, PgExecutor, Result};

pub struct MedicationMappingsSet;

impl MedicationMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MedicationMappings>> {
        query_as::<_, MedicationMappings>(
            r#"SELECT * FROM "pcc_athena_interop"."medication_mappings""#,
        )
        .fetch_all(executor)
        .await
    }

    pub async fn by_medication_mapping_id<'e, E: PgExecutor<'e>>(
        &self,
        executor: E,
        medication_mapping_id: i64,
    ) -> Result<MedicationMappings> {
        query_as::<_, MedicationMappings>(r#"SELECT * FROM "pcc_athena_interop"."medication_mappings" WHERE "medication_mapping_id" = $1"#)
            .bind(medication_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medication_mapping_id_list<'e, E: PgExecutor<'e>>(
        &self,
        executor: E,
        medication_mapping_id_list: Vec<i64>,
    ) -> Result<Vec<MedicationMappings>> {
        query_as::<_, MedicationMappings>(r#"SELECT * FROM "pcc_athena_interop"."medication_mappings" WHERE "medication_mapping_id" = ANY($1)"#)
            .bind(medication_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medication_mapping_id_optional<'e, E: PgExecutor<'e>>(
        &self,
        executor: E,
        medication_mapping_id: i64,
    ) -> Result<Option<MedicationMappings>> {
        query_as::<_, MedicationMappings>(
            r#"SELECT * FROM "medication_mappings" WHERE "medication_mapping_id" = $1"#,
        )
        .bind(medication_mapping_id)
        .fetch_optional(executor)
        .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(
        &self,
        executor: E,
        medication_mappings: MedicationMappings,
    ) -> Result<MedicationMappings> {
        query_as::<_, MedicationMappings>(r#"INSERT INTO "medication_mappings" ("medication_mapping_id", "pcc_order_id", "patient_mapping_id", "athena_medication_id", "athena_medicationentryid", "medication_external_id") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(medication_mappings.medication_mapping_id)
            .bind(medication_mappings.pcc_order_id)
            .bind(medication_mappings.patient_mapping_id)
            .bind(medication_mappings.athena_medication_id)
            .bind(medication_mappings.athena_medicationentryid)
            .bind(medication_mappings.medication_external_id)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(
        &self,
        executor: E,
        medication_mappings: MedicationMappings,
    ) -> Result<MedicationMappings> {
        query_as::<_, MedicationMappings>(r#"UPDATE "medication_mappings" SET "pcc_order_id" = $2, "patient_mapping_id" = $3, "athena_medication_id" = $4, "athena_medicationentryid" = $5, "medication_external_id" = $6 WHERE "medication_mapping_id" = $1 RETURNING *;"#)
            .bind(medication_mappings.medication_mapping_id)
            .bind(medication_mappings.pcc_order_id)
            .bind(medication_mappings.patient_mapping_id)
            .bind(medication_mappings.athena_medication_id)
            .bind(medication_mappings.athena_medicationentryid)
            .bind(medication_mappings.medication_external_id)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."medication_mappings" WHERE "medication_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }
}
