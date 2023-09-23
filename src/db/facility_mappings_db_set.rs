#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::FacilityMappings;

pub struct FacilityMappingsSet;

impl FacilityMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<FacilityMappings>> {
        query_as::<_, FacilityMappings>(r#"SELECT * FROM "pcc_athena_interop"."facility_mappings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_facility_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, facility_mapping_id: i64) -> Result<FacilityMappings> {
        query_as::<_, FacilityMappings>(r#"SELECT * FROM "pcc_athena_interop"."facility_mappings" WHERE "facility_mapping_id" = $1"#)
            .bind(facility_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_facility_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, facility_mapping_id_list: Vec<i64>) -> Result<Vec<FacilityMappings>> {
        query_as::<_, FacilityMappings>(r#"SELECT * FROM "pcc_athena_interop"."facility_mappings" WHERE "facility_mapping_id" = ANY($1)"#)
            .bind(facility_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_facility_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, facility_mapping_id: i64) -> Result<Option<FacilityMappings>> {
        query_as::<_, FacilityMappings>(r#"SELECT * FROM "pcc_athena_interop"."facility_mappings" WHERE "facility_mapping_id" = $1"#)
            .bind(facility_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, facility_mappings: FacilityMappings) -> Result<FacilityMappings> {
        query_as::<_, FacilityMappings>(r#"INSERT INTO "facility_mappings" ("facility_mapping_id", "cybx_customer_code", "facility_name", "athena_practice_id", "athena_department_id", "pcc_organization_id", "pcc_facility_id", "facility_status", "time_zone", "interfaces", "athena_department_name", "pcc_facility_name", "practitioner_filtering", "demographics", "diagnostic_reports") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) RETURNING *;"#)
            .bind(facility_mappings.facility_mapping_id)
            .bind(facility_mappings.cybx_customer_code)
            .bind(facility_mappings.facility_name)
            .bind(facility_mappings.athena_practice_id)
            .bind(facility_mappings.athena_department_id)
            .bind(facility_mappings.pcc_organization_id)
            .bind(facility_mappings.pcc_facility_id)
            .bind(facility_mappings.facility_status)
            .bind(facility_mappings.time_zone)
            .bind(facility_mappings.interfaces)
            .bind(facility_mappings.athena_department_name)
            .bind(facility_mappings.pcc_facility_name)
            .bind(facility_mappings.practitioner_filtering)
            .bind(facility_mappings.demographics)
            .bind(facility_mappings.diagnostic_reports)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, facility_mappings: FacilityMappings) -> Result<FacilityMappings> {
        query_as::<_, FacilityMappings>(r#"UPDATE "facility_mappings" SET "cybx_customer_code" = $2, "facility_name" = $3, "athena_practice_id" = $4, "athena_department_id" = $5, "pcc_organization_id" = $6, "pcc_facility_id" = $7, "facility_status" = $8, "time_zone" = $9, "interfaces" = $10, "athena_department_name" = $11, "pcc_facility_name" = $12, "practitioner_filtering" = $13, "demographics" = $14, "diagnostic_reports" = $15 WHERE "facility_mapping_id" = $1 RETURNING *;"#)
            .bind(facility_mappings.facility_mapping_id)
            .bind(facility_mappings.cybx_customer_code)
            .bind(facility_mappings.facility_name)
            .bind(facility_mappings.athena_practice_id)
            .bind(facility_mappings.athena_department_id)
            .bind(facility_mappings.pcc_organization_id)
            .bind(facility_mappings.pcc_facility_id)
            .bind(facility_mappings.facility_status)
            .bind(facility_mappings.time_zone)
            .bind(facility_mappings.interfaces)
            .bind(facility_mappings.athena_department_name)
            .bind(facility_mappings.pcc_facility_name)
            .bind(facility_mappings.practitioner_filtering)
            .bind(facility_mappings.demographics)
            .bind(facility_mappings.diagnostic_reports)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."facility_mappings" WHERE "facility_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
