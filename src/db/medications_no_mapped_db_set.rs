#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MedicationsNoMapped;

pub struct MedicationsNoMappedSet;

impl MedicationsNoMappedSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MedicationsNoMapped>> {
        query_as::<_, MedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."medications_no_mapped""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medication_no_mapped_id<'e, E: PgExecutor<'e>>(&self, executor: E, medication_no_mapped_id: i64) -> Result<MedicationsNoMapped> {
        query_as::<_, MedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."medications_no_mapped" WHERE "medication_no_mapped_id" = $1"#)
            .bind(medication_no_mapped_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medication_no_mapped_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, medication_no_mapped_id_list: Vec<i64>) -> Result<Vec<MedicationsNoMapped>> {
        query_as::<_, MedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."medications_no_mapped" WHERE "medication_no_mapped_id" = ANY($1)"#)
            .bind(medication_no_mapped_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medication_no_mapped_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medication_no_mapped_id: i64) -> Result<Option<MedicationsNoMapped>> {
        query_as::<_, MedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."medications_no_mapped" WHERE "medication_no_mapped_id" = $1"#)
            .bind(medication_no_mapped_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medications_no_mapped: MedicationsNoMapped) -> Result<MedicationsNoMapped> {
        query_as::<_, MedicationsNoMapped>(r#"INSERT INTO "medications_no_mapped" ("medication_no_mapped_id", "practiceid", "pcc_description", "rxnorm", "updated_at") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(medications_no_mapped.medication_no_mapped_id)
            .bind(medications_no_mapped.practiceid)
            .bind(medications_no_mapped.pcc_description)
            .bind(medications_no_mapped.rxnorm)
            .bind(medications_no_mapped.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medications_no_mapped: MedicationsNoMapped) -> Result<MedicationsNoMapped> {
        query_as::<_, MedicationsNoMapped>(r#"UPDATE "medications_no_mapped" SET "practiceid" = $2, "pcc_description" = $3, "rxnorm" = $4, "updated_at" = $5 WHERE "medication_no_mapped_id" = $1 RETURNING *;"#)
            .bind(medications_no_mapped.medication_no_mapped_id)
            .bind(medications_no_mapped.practiceid)
            .bind(medications_no_mapped.pcc_description)
            .bind(medications_no_mapped.rxnorm)
            .bind(medications_no_mapped.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."medications_no_mapped" WHERE "medication_no_mapped_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
