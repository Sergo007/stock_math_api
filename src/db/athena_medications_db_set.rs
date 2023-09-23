#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AthenaMedications;

pub struct AthenaMedicationsSet;

impl AthenaMedicationsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AthenaMedications>> {
        query_as::<_, AthenaMedications>(r#"SELECT * FROM "pcc_athena_interop"."athena_medications""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medicationid<'e, E: PgExecutor<'e>>(&self, executor: E, medicationid: i64) -> Result<AthenaMedications> {
        query_as::<_, AthenaMedications>(r#"SELECT * FROM "pcc_athena_interop"."athena_medications" WHERE "medicationid" = $1"#)
            .bind(medicationid)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medicationid_list<'e, E: PgExecutor<'e>>(&self, executor: E, medicationid_list: Vec<i64>) -> Result<Vec<AthenaMedications>> {
        query_as::<_, AthenaMedications>(r#"SELECT * FROM "pcc_athena_interop"."athena_medications" WHERE "medicationid" = ANY($1)"#)
            .bind(medicationid_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medicationid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medicationid: i64) -> Result<Option<AthenaMedications>> {
        query_as::<_, AthenaMedications>(r#"SELECT * FROM "pcc_athena_interop"."athena_medications" WHERE "medicationid" = $1"#)
            .bind(medicationid)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, athena_medications: AthenaMedications) -> Result<AthenaMedications> {
        query_as::<_, AthenaMedications>(r#"INSERT INTO "athena_medications" ("medicationid", "medicationname", "rxnorm", "ndc", "updated_at") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(athena_medications.medicationid)
            .bind(athena_medications.medicationname)
            .bind(athena_medications.rxnorm)
            .bind(athena_medications.ndc)
            .bind(athena_medications.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, athena_medications: AthenaMedications) -> Result<AthenaMedications> {
        query_as::<_, AthenaMedications>(r#"UPDATE "athena_medications" SET "medicationname" = $2, "rxnorm" = $3, "ndc" = $4, "updated_at" = $5 WHERE "medicationid" = $1 RETURNING *;"#)
            .bind(athena_medications.medicationid)
            .bind(athena_medications.medicationname)
            .bind(athena_medications.rxnorm)
            .bind(athena_medications.ndc)
            .bind(athena_medications.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."athena_medications" WHERE "medicationid" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
