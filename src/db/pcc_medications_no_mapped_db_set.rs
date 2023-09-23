#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PccMedicationsNoMapped;

pub struct PccMedicationsNoMappedSet;

impl PccMedicationsNoMappedSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PccMedicationsNoMapped>> {
        query_as::<_, PccMedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."pcc_medications_no_mapped""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pcc_medication_no_mapped_id<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_medication_no_mapped_id: i64) -> Result<PccMedicationsNoMapped> {
        query_as::<_, PccMedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."pcc_medications_no_mapped" WHERE "pcc_medication_no_mapped_id" = $1"#)
            .bind(pcc_medication_no_mapped_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pcc_medication_no_mapped_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_medication_no_mapped_id_list: Vec<i64>) -> Result<Vec<PccMedicationsNoMapped>> {
        query_as::<_, PccMedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."pcc_medications_no_mapped" WHERE "pcc_medication_no_mapped_id" = ANY($1)"#)
            .bind(pcc_medication_no_mapped_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pcc_medication_no_mapped_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_medication_no_mapped_id: i64) -> Result<Option<PccMedicationsNoMapped>> {
        query_as::<_, PccMedicationsNoMapped>(r#"SELECT * FROM "pcc_athena_interop"."pcc_medications_no_mapped" WHERE "pcc_medication_no_mapped_id" = $1"#)
            .bind(pcc_medication_no_mapped_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_medications_no_mapped: PccMedicationsNoMapped) -> Result<PccMedicationsNoMapped> {
        query_as::<_, PccMedicationsNoMapped>(r#"INSERT INTO "pcc_medications_no_mapped" ("pcc_medication_no_mapped_id", "pcc_description", "rxnorm", "updated_at") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(pcc_medications_no_mapped.pcc_medication_no_mapped_id)
            .bind(pcc_medications_no_mapped.pcc_description)
            .bind(pcc_medications_no_mapped.rxnorm)
            .bind(pcc_medications_no_mapped.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_medications_no_mapped: PccMedicationsNoMapped) -> Result<PccMedicationsNoMapped> {
        query_as::<_, PccMedicationsNoMapped>(r#"UPDATE "pcc_medications_no_mapped" SET "pcc_description" = $2, "rxnorm" = $3, "updated_at" = $4 WHERE "pcc_medication_no_mapped_id" = $1 RETURNING *;"#)
            .bind(pcc_medications_no_mapped.pcc_medication_no_mapped_id)
            .bind(pcc_medications_no_mapped.pcc_description)
            .bind(pcc_medications_no_mapped.rxnorm)
            .bind(pcc_medications_no_mapped.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."pcc_medications_no_mapped" WHERE "pcc_medication_no_mapped_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
