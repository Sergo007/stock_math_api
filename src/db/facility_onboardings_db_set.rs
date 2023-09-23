#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::FacilityOnboardings;

pub struct FacilityOnboardingsSet;

impl FacilityOnboardingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<FacilityOnboardings>> {
        query_as::<_, FacilityOnboardings>(r#"SELECT * FROM "pcc_athena_interop"."facility_onboardings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_facility_onboarding_id<'e, E: PgExecutor<'e>>(&self, executor: E, facility_onboarding_id: i64) -> Result<FacilityOnboardings> {
        query_as::<_, FacilityOnboardings>(r#"SELECT * FROM "pcc_athena_interop"."facility_onboardings" WHERE "facility_onboarding_id" = $1"#)
            .bind(facility_onboarding_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_facility_onboarding_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, facility_onboarding_id_list: Vec<i64>) -> Result<Vec<FacilityOnboardings>> {
        query_as::<_, FacilityOnboardings>(r#"SELECT * FROM "pcc_athena_interop"."facility_onboardings" WHERE "facility_onboarding_id" = ANY($1)"#)
            .bind(facility_onboarding_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_facility_onboarding_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, facility_onboarding_id: i64) -> Result<Option<FacilityOnboardings>> {
        query_as::<_, FacilityOnboardings>(r#"SELECT * FROM "pcc_athena_interop"."facility_onboardings" WHERE "facility_onboarding_id" = $1"#)
            .bind(facility_onboarding_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, facility_onboardings: FacilityOnboardings) -> Result<FacilityOnboardings> {
        query_as::<_, FacilityOnboardings>(r#"INSERT INTO "facility_onboardings" ("facility_onboarding_id", "facility_mapping_id", "pcc_patients_ids", "interfaces", "all_pcc_patients_ids", "loaded_pcc_patients_ids", "onboarding_status") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(facility_onboardings.facility_onboarding_id)
            .bind(facility_onboardings.facility_mapping_id)
            .bind(facility_onboardings.pcc_patients_ids)
            .bind(facility_onboardings.interfaces)
            .bind(facility_onboardings.all_pcc_patients_ids)
            .bind(facility_onboardings.loaded_pcc_patients_ids)
            .bind(facility_onboardings.onboarding_status)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, facility_onboardings: FacilityOnboardings) -> Result<FacilityOnboardings> {
        query_as::<_, FacilityOnboardings>(r#"UPDATE "facility_onboardings" SET "facility_mapping_id" = $2, "pcc_patients_ids" = $3, "interfaces" = $4, "all_pcc_patients_ids" = $5, "loaded_pcc_patients_ids" = $6, "onboarding_status" = $7 WHERE "facility_onboarding_id" = $1 RETURNING *;"#)
            .bind(facility_onboardings.facility_onboarding_id)
            .bind(facility_onboardings.facility_mapping_id)
            .bind(facility_onboardings.pcc_patients_ids)
            .bind(facility_onboardings.interfaces)
            .bind(facility_onboardings.all_pcc_patients_ids)
            .bind(facility_onboardings.loaded_pcc_patients_ids)
            .bind(facility_onboardings.onboarding_status)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."facility_onboardings" WHERE "facility_onboarding_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
