#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PatientMappings;

pub struct PatientMappingsSet;

impl PatientMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PatientMappings>> {
        query_as::<_, PatientMappings>(r#"SELECT * FROM "pcc_athena_interop"."patient_mappings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_patient_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, patient_mapping_id: i64) -> Result<PatientMappings> {
        query_as::<_, PatientMappings>(r#"SELECT * FROM "pcc_athena_interop"."patient_mappings" WHERE "patient_mapping_id" = $1"#)
            .bind(patient_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_patient_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, patient_mapping_id_list: Vec<i64>) -> Result<Vec<PatientMappings>> {
        query_as::<_, PatientMappings>(r#"SELECT * FROM "pcc_athena_interop"."patient_mappings" WHERE "patient_mapping_id" = ANY($1)"#)
            .bind(patient_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_patient_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, patient_mapping_id: i64) -> Result<Option<PatientMappings>> {
        query_as::<_, PatientMappings>(r#"SELECT * FROM "pcc_athena_interop"."patient_mappings" WHERE "patient_mapping_id" = $1"#)
            .bind(patient_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, patient_mappings: PatientMappings) -> Result<PatientMappings> {
        query_as::<_, PatientMappings>(r#"INSERT INTO "patient_mappings" ("patient_mapping_id", "facility_mapping_id", "pcc_patient_id", "athena_patient_id", "athena_practice_id", "pcc_patient_status", "athena_current_department_id", "pcc_patient_omp_id", "pcc_patient_admission_date_time", "pcc_patient_discharge_date_time", "created_at") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *;"#)
            .bind(patient_mappings.patient_mapping_id)
            .bind(patient_mappings.facility_mapping_id)
            .bind(patient_mappings.pcc_patient_id)
            .bind(patient_mappings.athena_patient_id)
            .bind(patient_mappings.athena_practice_id)
            .bind(patient_mappings.pcc_patient_status)
            .bind(patient_mappings.athena_current_department_id)
            .bind(patient_mappings.pcc_patient_omp_id)
            .bind(patient_mappings.pcc_patient_admission_date_time)
            .bind(patient_mappings.pcc_patient_discharge_date_time)
            .bind(patient_mappings.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, patient_mappings: PatientMappings) -> Result<PatientMappings> {
        query_as::<_, PatientMappings>(r#"UPDATE "patient_mappings" SET "facility_mapping_id" = $2, "pcc_patient_id" = $3, "athena_patient_id" = $4, "athena_practice_id" = $5, "pcc_patient_status" = $6, "athena_current_department_id" = $7, "pcc_patient_omp_id" = $8, "pcc_patient_admission_date_time" = $9, "pcc_patient_discharge_date_time" = $10, "created_at" = $11 WHERE "patient_mapping_id" = $1 RETURNING *;"#)
            .bind(patient_mappings.patient_mapping_id)
            .bind(patient_mappings.facility_mapping_id)
            .bind(patient_mappings.pcc_patient_id)
            .bind(patient_mappings.athena_patient_id)
            .bind(patient_mappings.athena_practice_id)
            .bind(patient_mappings.pcc_patient_status)
            .bind(patient_mappings.athena_current_department_id)
            .bind(patient_mappings.pcc_patient_omp_id)
            .bind(patient_mappings.pcc_patient_admission_date_time)
            .bind(patient_mappings.pcc_patient_discharge_date_time)
            .bind(patient_mappings.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."patient_mappings" WHERE "patient_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
