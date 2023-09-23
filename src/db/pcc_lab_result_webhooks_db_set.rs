#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PccLabResultWebhooks;

pub struct PccLabResultWebhooksSet;

impl PccLabResultWebhooksSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PccLabResultWebhooks>> {
        query_as::<_, PccLabResultWebhooks>(r#"SELECT * FROM "pcc_athena_interop"."pcc_lab_result_webhooks""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_message_id<'e, E: PgExecutor<'e>>(&self, executor: E, message_id: String) -> Result<PccLabResultWebhooks> {
        query_as::<_, PccLabResultWebhooks>(r#"SELECT * FROM "pcc_athena_interop"."pcc_lab_result_webhooks" WHERE "message_id" = $1"#)
            .bind(message_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_message_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, message_id_list: Vec<String>) -> Result<Vec<PccLabResultWebhooks>> {
        query_as::<_, PccLabResultWebhooks>(r#"SELECT * FROM "pcc_athena_interop"."pcc_lab_result_webhooks" WHERE "message_id" = ANY($1)"#)
            .bind(message_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_message_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, message_id: String) -> Result<Option<PccLabResultWebhooks>> {
        query_as::<_, PccLabResultWebhooks>(r#"SELECT * FROM "pcc_athena_interop"."pcc_lab_result_webhooks" WHERE "message_id" = $1"#)
            .bind(message_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_lab_result_webhooks: PccLabResultWebhooks) -> Result<PccLabResultWebhooks> {
        query_as::<_, PccLabResultWebhooks>(r#"INSERT INTO "pcc_lab_result_webhooks" ("message_id", "org_uuid", "patient_id", "resource_id", "event_type", "updated_at") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(pcc_lab_result_webhooks.message_id)
            .bind(pcc_lab_result_webhooks.org_uuid)
            .bind(pcc_lab_result_webhooks.patient_id)
            .bind(pcc_lab_result_webhooks.resource_id)
            .bind(pcc_lab_result_webhooks.event_type)
            .bind(pcc_lab_result_webhooks.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_lab_result_webhooks: PccLabResultWebhooks) -> Result<PccLabResultWebhooks> {
        query_as::<_, PccLabResultWebhooks>(r#"UPDATE "pcc_lab_result_webhooks" SET "org_uuid" = $2, "patient_id" = $3, "resource_id" = $4, "event_type" = $5, "updated_at" = $6 WHERE "message_id" = $1 RETURNING *;"#)
            .bind(pcc_lab_result_webhooks.message_id)
            .bind(pcc_lab_result_webhooks.org_uuid)
            .bind(pcc_lab_result_webhooks.patient_id)
            .bind(pcc_lab_result_webhooks.resource_id)
            .bind(pcc_lab_result_webhooks.event_type)
            .bind(pcc_lab_result_webhooks.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."pcc_lab_result_webhooks" WHERE "message_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
