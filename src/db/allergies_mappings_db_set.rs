#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AllergiesMappings;

pub struct AllergiesMappingsSet;

impl AllergiesMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AllergiesMappings>> {
        query_as::<_, AllergiesMappings>(r#"SELECT * FROM "pcc_athena_interop"."allergies_mappings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_allergy_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, allergy_mapping_id: i64) -> Result<AllergiesMappings> {
        query_as::<_, AllergiesMappings>(r#"SELECT * FROM "pcc_athena_interop"."allergies_mappings" WHERE "allergy_mapping_id" = $1"#)
            .bind(allergy_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_allergy_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, allergy_mapping_id_list: Vec<i64>) -> Result<Vec<AllergiesMappings>> {
        query_as::<_, AllergiesMappings>(r#"SELECT * FROM "pcc_athena_interop"."allergies_mappings" WHERE "allergy_mapping_id" = ANY($1)"#)
            .bind(allergy_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_allergy_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, allergy_mapping_id: i64) -> Result<Option<AllergiesMappings>> {
        query_as::<_, AllergiesMappings>(r#"SELECT * FROM "pcc_athena_interop"."allergies_mappings" WHERE "allergy_mapping_id" = $1"#)
            .bind(allergy_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, allergies_mappings: AllergiesMappings) -> Result<AllergiesMappings> {
        query_as::<_, AllergiesMappings>(r#"INSERT INTO "allergies_mappings" ("allergy_mapping_id", "patient_mapping_id", "pcc_allergy_intolerance_id", "allergen_name", "athena_allergen_id") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(allergies_mappings.allergy_mapping_id)
            .bind(allergies_mappings.patient_mapping_id)
            .bind(allergies_mappings.pcc_allergy_intolerance_id)
            .bind(allergies_mappings.allergen_name)
            .bind(allergies_mappings.athena_allergen_id)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, allergies_mappings: AllergiesMappings) -> Result<AllergiesMappings> {
        query_as::<_, AllergiesMappings>(r#"UPDATE "allergies_mappings" SET "patient_mapping_id" = $2, "pcc_allergy_intolerance_id" = $3, "allergen_name" = $4, "athena_allergen_id" = $5 WHERE "allergy_mapping_id" = $1 RETURNING *;"#)
            .bind(allergies_mappings.allergy_mapping_id)
            .bind(allergies_mappings.patient_mapping_id)
            .bind(allergies_mappings.pcc_allergy_intolerance_id)
            .bind(allergies_mappings.allergen_name)
            .bind(allergies_mappings.athena_allergen_id)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."allergies_mappings" WHERE "allergy_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
