#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlembicVersion;

pub struct AlembicVersionSet;

impl AlembicVersionSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlembicVersion>> {
        query_as::<_, AlembicVersion>(r#"SELECT * FROM "pcc_athena_interop"."alembic_version""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_version_num<'e, E: PgExecutor<'e>>(&self, executor: E, version_num: String) -> Result<AlembicVersion> {
        query_as::<_, AlembicVersion>(r#"SELECT * FROM "pcc_athena_interop"."alembic_version" WHERE "version_num" = $1"#)
            .bind(version_num)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_version_num_list<'e, E: PgExecutor<'e>>(&self, executor: E, version_num_list: Vec<String>) -> Result<Vec<AlembicVersion>> {
        query_as::<_, AlembicVersion>(r#"SELECT * FROM "pcc_athena_interop"."alembic_version" WHERE "version_num" = ANY($1)"#)
            .bind(version_num_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_version_num_optional<'e, E: PgExecutor<'e>>(&self, executor: E, version_num: String) -> Result<Option<AlembicVersion>> {
        query_as::<_, AlembicVersion>(r#"SELECT * FROM "pcc_athena_interop"."alembic_version" WHERE "version_num" = $1"#)
            .bind(version_num)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alembic_version: AlembicVersion) -> Result<AlembicVersion> {
        query_as::<_, AlembicVersion>(r#"INSERT INTO "alembic_version" ("version_num") VALUES ($1) RETURNING *;"#)
            .bind(alembic_version.version_num)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alembic_version: AlembicVersion) -> Result<AlembicVersion> {
        query_as::<_, AlembicVersion>(r#"UPDATE "alembic_version" SET  WHERE "version_num" = $1 RETURNING *;"#)
            .bind(alembic_version.version_num)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."alembic_version" WHERE "version_num" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
