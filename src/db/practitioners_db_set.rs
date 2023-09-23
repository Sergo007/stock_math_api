#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Practitioners;

pub struct PractitionersSet;

impl PractitionersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Practitioners>> {
        query_as::<_, Practitioners>(r#"SELECT * FROM "pcc_athena_interop"."practitioners""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_practitioner_id<'e, E: PgExecutor<'e>>(&self, executor: E, practitioner_id: i64) -> Result<Practitioners> {
        query_as::<_, Practitioners>(r#"SELECT * FROM "pcc_athena_interop"."practitioners" WHERE "practitioner_id" = $1"#)
            .bind(practitioner_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_practitioner_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, practitioner_id_list: Vec<i64>) -> Result<Vec<Practitioners>> {
        query_as::<_, Practitioners>(r#"SELECT * FROM "pcc_athena_interop"."practitioners" WHERE "practitioner_id" = ANY($1)"#)
            .bind(practitioner_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_practitioner_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, practitioner_id: i64) -> Result<Option<Practitioners>> {
        query_as::<_, Practitioners>(r#"SELECT * FROM "pcc_athena_interop"."practitioners" WHERE "practitioner_id" = $1"#)
            .bind(practitioner_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, practitioners: Practitioners) -> Result<Practitioners> {
        query_as::<_, Practitioners>(r#"INSERT INTO "practitioners" ("practitioner_id", "cybx_customer_code", "athena_practice_id", "first_name", "last_name", "npi") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(practitioners.practitioner_id)
            .bind(practitioners.cybx_customer_code)
            .bind(practitioners.athena_practice_id)
            .bind(practitioners.first_name)
            .bind(practitioners.last_name)
            .bind(practitioners.npi)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, practitioners: Practitioners) -> Result<Practitioners> {
        query_as::<_, Practitioners>(r#"UPDATE "practitioners" SET "cybx_customer_code" = $2, "athena_practice_id" = $3, "first_name" = $4, "last_name" = $5, "npi" = $6 WHERE "practitioner_id" = $1 RETURNING *;"#)
            .bind(practitioners.practitioner_id)
            .bind(practitioners.cybx_customer_code)
            .bind(practitioners.athena_practice_id)
            .bind(practitioners.first_name)
            .bind(practitioners.last_name)
            .bind(practitioners.npi)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."practitioners" WHERE "practitioner_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
