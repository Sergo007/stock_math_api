#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MedicationsIdsRxnormMappings;

pub struct MedicationsIdsRxnormMappingsSet;

impl MedicationsIdsRxnormMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MedicationsIdsRxnormMappings>> {
        query_as::<_, MedicationsIdsRxnormMappings>(r#"SELECT * FROM "pcc_athena_interop"."medications_ids_rxnorm_mappings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medications_ids_rxnorm_mappings_id<'e, E: PgExecutor<'e>>(&self, executor: E, medications_ids_rxnorm_mappings_id: i64) -> Result<MedicationsIdsRxnormMappings> {
        query_as::<_, MedicationsIdsRxnormMappings>(r#"SELECT * FROM "pcc_athena_interop"."medications_ids_rxnorm_mappings" WHERE "medications_ids_rxnorm_mappings_id" = $1"#)
            .bind(medications_ids_rxnorm_mappings_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medications_ids_rxnorm_mappings_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, medications_ids_rxnorm_mappings_id_list: Vec<i64>) -> Result<Vec<MedicationsIdsRxnormMappings>> {
        query_as::<_, MedicationsIdsRxnormMappings>(r#"SELECT * FROM "pcc_athena_interop"."medications_ids_rxnorm_mappings" WHERE "medications_ids_rxnorm_mappings_id" = ANY($1)"#)
            .bind(medications_ids_rxnorm_mappings_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medications_ids_rxnorm_mappings_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medications_ids_rxnorm_mappings_id: i64) -> Result<Option<MedicationsIdsRxnormMappings>> {
        query_as::<_, MedicationsIdsRxnormMappings>(r#"SELECT * FROM "pcc_athena_interop"."medications_ids_rxnorm_mappings" WHERE "medications_ids_rxnorm_mappings_id" = $1"#)
            .bind(medications_ids_rxnorm_mappings_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medications_ids_rxnorm_mappings: MedicationsIdsRxnormMappings) -> Result<MedicationsIdsRxnormMappings> {
        query_as::<_, MedicationsIdsRxnormMappings>(r#"INSERT INTO "medications_ids_rxnorm_mappings" ("medications_ids_rxnorm_mappings_id", "practiceid", "medicationid", "athena_description", "rxnorm", "updated_at") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(medications_ids_rxnorm_mappings.medications_ids_rxnorm_mappings_id)
            .bind(medications_ids_rxnorm_mappings.practiceid)
            .bind(medications_ids_rxnorm_mappings.medicationid)
            .bind(medications_ids_rxnorm_mappings.athena_description)
            .bind(medications_ids_rxnorm_mappings.rxnorm)
            .bind(medications_ids_rxnorm_mappings.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medications_ids_rxnorm_mappings: MedicationsIdsRxnormMappings) -> Result<MedicationsIdsRxnormMappings> {
        query_as::<_, MedicationsIdsRxnormMappings>(r#"UPDATE "medications_ids_rxnorm_mappings" SET "practiceid" = $2, "medicationid" = $3, "athena_description" = $4, "rxnorm" = $5, "updated_at" = $6 WHERE "medications_ids_rxnorm_mappings_id" = $1 RETURNING *;"#)
            .bind(medications_ids_rxnorm_mappings.medications_ids_rxnorm_mappings_id)
            .bind(medications_ids_rxnorm_mappings.practiceid)
            .bind(medications_ids_rxnorm_mappings.medicationid)
            .bind(medications_ids_rxnorm_mappings.athena_description)
            .bind(medications_ids_rxnorm_mappings.rxnorm)
            .bind(medications_ids_rxnorm_mappings.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."medications_ids_rxnorm_mappings" WHERE "medications_ids_rxnorm_mappings_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
