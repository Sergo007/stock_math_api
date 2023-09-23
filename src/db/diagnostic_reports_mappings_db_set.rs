#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::DiagnosticReportsMappings;

pub struct DiagnosticReportsMappingsSet;

impl DiagnosticReportsMappingsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<DiagnosticReportsMappings>> {
        query_as::<_, DiagnosticReportsMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnostic_reports_mappings""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_diagnostic_report_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, diagnostic_report_mapping_id: i64) -> Result<DiagnosticReportsMappings> {
        query_as::<_, DiagnosticReportsMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnostic_reports_mappings" WHERE "diagnostic_report_mapping_id" = $1"#)
            .bind(diagnostic_report_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_diagnostic_report_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, diagnostic_report_mapping_id_list: Vec<i64>) -> Result<Vec<DiagnosticReportsMappings>> {
        query_as::<_, DiagnosticReportsMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnostic_reports_mappings" WHERE "diagnostic_report_mapping_id" = ANY($1)"#)
            .bind(diagnostic_report_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_diagnostic_report_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, diagnostic_report_mapping_id: i64) -> Result<Option<DiagnosticReportsMappings>> {
        query_as::<_, DiagnosticReportsMappings>(r#"SELECT * FROM "pcc_athena_interop"."diagnostic_reports_mappings" WHERE "diagnostic_report_mapping_id" = $1"#)
            .bind(diagnostic_report_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, diagnostic_reports_mappings: DiagnosticReportsMappings) -> Result<DiagnosticReportsMappings> {
        query_as::<_, DiagnosticReportsMappings>(r#"INSERT INTO "diagnostic_reports_mappings" ("diagnostic_report_mapping_id", "patient_mapping_id", "pcc_diagnostic_report_id", "athena_lab_result_id", "pcc_panel_name") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(diagnostic_reports_mappings.diagnostic_report_mapping_id)
            .bind(diagnostic_reports_mappings.patient_mapping_id)
            .bind(diagnostic_reports_mappings.pcc_diagnostic_report_id)
            .bind(diagnostic_reports_mappings.athena_lab_result_id)
            .bind(diagnostic_reports_mappings.pcc_panel_name)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, diagnostic_reports_mappings: DiagnosticReportsMappings) -> Result<DiagnosticReportsMappings> {
        query_as::<_, DiagnosticReportsMappings>(r#"UPDATE "diagnostic_reports_mappings" SET "patient_mapping_id" = $2, "pcc_diagnostic_report_id" = $3, "athena_lab_result_id" = $4, "pcc_panel_name" = $5 WHERE "diagnostic_report_mapping_id" = $1 RETURNING *;"#)
            .bind(diagnostic_reports_mappings.diagnostic_report_mapping_id)
            .bind(diagnostic_reports_mappings.patient_mapping_id)
            .bind(diagnostic_reports_mappings.pcc_diagnostic_report_id)
            .bind(diagnostic_reports_mappings.athena_lab_result_id)
            .bind(diagnostic_reports_mappings.pcc_panel_name)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."diagnostic_reports_mappings" WHERE "diagnostic_report_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
