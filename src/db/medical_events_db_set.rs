#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MedicalEvents;

pub struct MedicalEventsSet;

impl MedicalEventsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MedicalEvents>> {
        query_as::<_, MedicalEvents>(r#"SELECT * FROM "pcc_athena_interop"."medical_events""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medical_event_id<'e, E: PgExecutor<'e>>(&self, executor: E, medical_event_id: i64) -> Result<MedicalEvents> {
        query_as::<_, MedicalEvents>(r#"SELECT * FROM "pcc_athena_interop"."medical_events" WHERE "medical_event_id" = $1"#)
            .bind(medical_event_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medical_event_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, medical_event_id_list: Vec<i64>) -> Result<Vec<MedicalEvents>> {
        query_as::<_, MedicalEvents>(r#"SELECT * FROM "pcc_athena_interop"."medical_events" WHERE "medical_event_id" = ANY($1)"#)
            .bind(medical_event_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medical_event_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medical_event_id: i64) -> Result<Option<MedicalEvents>> {
        query_as::<_, MedicalEvents>(r#"SELECT * FROM "pcc_athena_interop"."medical_events" WHERE "medical_event_id" = $1"#)
            .bind(medical_event_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medical_events: MedicalEvents) -> Result<MedicalEvents> {
        query_as::<_, MedicalEvents>(r#"INSERT INTO "medical_events" ("medical_event_id", "webhook_id", "facility_mapping_id", "pcc_patient_id", "interface_name", "transaction_name", "athena_requests_count", "athena_requests", "updated_at") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(medical_events.medical_event_id)
            .bind(medical_events.webhook_id)
            .bind(medical_events.facility_mapping_id)
            .bind(medical_events.pcc_patient_id)
            .bind(medical_events.interface_name)
            .bind(medical_events.transaction_name)
            .bind(medical_events.athena_requests_count)
            .bind(medical_events.athena_requests)
            .bind(medical_events.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medical_events: MedicalEvents) -> Result<MedicalEvents> {
        query_as::<_, MedicalEvents>(r#"UPDATE "medical_events" SET "webhook_id" = $2, "facility_mapping_id" = $3, "pcc_patient_id" = $4, "interface_name" = $5, "transaction_name" = $6, "athena_requests_count" = $7, "athena_requests" = $8, "updated_at" = $9 WHERE "medical_event_id" = $1 RETURNING *;"#)
            .bind(medical_events.medical_event_id)
            .bind(medical_events.webhook_id)
            .bind(medical_events.facility_mapping_id)
            .bind(medical_events.pcc_patient_id)
            .bind(medical_events.interface_name)
            .bind(medical_events.transaction_name)
            .bind(medical_events.athena_requests_count)
            .bind(medical_events.athena_requests)
            .bind(medical_events.updated_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."medical_events" WHERE "medical_event_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
