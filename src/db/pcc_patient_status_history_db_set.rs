#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PccPatientStatusHistory;

pub struct PccPatientStatusHistorySet;

impl PccPatientStatusHistorySet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PccPatientStatusHistory>> {
        query_as::<_, PccPatientStatusHistory>(r#"SELECT * FROM "pcc_athena_interop"."pcc_patient_status_history""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_patient_status_history_id<'e, E: PgExecutor<'e>>(&self, executor: E, patient_status_history_id: i64) -> Result<PccPatientStatusHistory> {
        query_as::<_, PccPatientStatusHistory>(r#"SELECT * FROM "pcc_athena_interop"."pcc_patient_status_history" WHERE "patient_status_history_id" = $1"#)
            .bind(patient_status_history_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_patient_status_history_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, patient_status_history_id_list: Vec<i64>) -> Result<Vec<PccPatientStatusHistory>> {
        query_as::<_, PccPatientStatusHistory>(r#"SELECT * FROM "pcc_athena_interop"."pcc_patient_status_history" WHERE "patient_status_history_id" = ANY($1)"#)
            .bind(patient_status_history_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_patient_status_history_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, patient_status_history_id: i64) -> Result<Option<PccPatientStatusHistory>> {
        query_as::<_, PccPatientStatusHistory>(r#"SELECT * FROM "pcc_athena_interop"."pcc_patient_status_history" WHERE "patient_status_history_id" = $1"#)
            .bind(patient_status_history_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_patient_status_history: PccPatientStatusHistory) -> Result<PccPatientStatusHistory> {
        query_as::<_, PccPatientStatusHistory>(r#"INSERT INTO "pcc_patient_status_history" ("patient_status_history_id", "patient_id", "organization_id", "facility_id", "patient_omp_id", "patient_status", "patient_status_date_time", "created_at") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(pcc_patient_status_history.patient_status_history_id)
            .bind(pcc_patient_status_history.patient_id)
            .bind(pcc_patient_status_history.organization_id)
            .bind(pcc_patient_status_history.facility_id)
            .bind(pcc_patient_status_history.patient_omp_id)
            .bind(pcc_patient_status_history.patient_status)
            .bind(pcc_patient_status_history.patient_status_date_time)
            .bind(pcc_patient_status_history.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_patient_status_history: PccPatientStatusHistory) -> Result<PccPatientStatusHistory> {
        query_as::<_, PccPatientStatusHistory>(r#"UPDATE "pcc_patient_status_history" SET "patient_id" = $2, "organization_id" = $3, "facility_id" = $4, "patient_omp_id" = $5, "patient_status" = $6, "patient_status_date_time" = $7, "created_at" = $8 WHERE "patient_status_history_id" = $1 RETURNING *;"#)
            .bind(pcc_patient_status_history.patient_status_history_id)
            .bind(pcc_patient_status_history.patient_id)
            .bind(pcc_patient_status_history.organization_id)
            .bind(pcc_patient_status_history.facility_id)
            .bind(pcc_patient_status_history.patient_omp_id)
            .bind(pcc_patient_status_history.patient_status)
            .bind(pcc_patient_status_history.patient_status_date_time)
            .bind(pcc_patient_status_history.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."pcc_patient_status_history" WHERE "patient_status_history_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
