#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PccPanelNameToAthenaDocumentType;

pub struct PccPanelNameToAthenaDocumentTypeSet;

impl PccPanelNameToAthenaDocumentTypeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PccPanelNameToAthenaDocumentType>> {
        query_as::<_, PccPanelNameToAthenaDocumentType>(r#"SELECT * FROM "pcc_athena_interop"."pcc_panel_name_to_athena_document_type""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pcc_panel_to_athena_document_type_mapping_id<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_panel_to_athena_document_type_mapping_id: i64) -> Result<PccPanelNameToAthenaDocumentType> {
        query_as::<_, PccPanelNameToAthenaDocumentType>(r#"SELECT * FROM "pcc_athena_interop"."pcc_panel_name_to_athena_document_type" WHERE "pcc_panel_to_athena_document_type_mapping_id" = $1"#)
            .bind(pcc_panel_to_athena_document_type_mapping_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_pcc_panel_to_athena_document_type_mapping_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_panel_to_athena_document_type_mapping_id_list: Vec<i64>) -> Result<Vec<PccPanelNameToAthenaDocumentType>> {
        query_as::<_, PccPanelNameToAthenaDocumentType>(r#"SELECT * FROM "pcc_athena_interop"."pcc_panel_name_to_athena_document_type" WHERE "pcc_panel_to_athena_document_type_mapping_id" = ANY($1)"#)
            .bind(pcc_panel_to_athena_document_type_mapping_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_pcc_panel_to_athena_document_type_mapping_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_panel_to_athena_document_type_mapping_id: i64) -> Result<Option<PccPanelNameToAthenaDocumentType>> {
        query_as::<_, PccPanelNameToAthenaDocumentType>(r#"SELECT * FROM "pcc_athena_interop"."pcc_panel_name_to_athena_document_type" WHERE "pcc_panel_to_athena_document_type_mapping_id" = $1"#)
            .bind(pcc_panel_to_athena_document_type_mapping_id)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_panel_name_to_athena_document_type: PccPanelNameToAthenaDocumentType) -> Result<PccPanelNameToAthenaDocumentType> {
        query_as::<_, PccPanelNameToAthenaDocumentType>(r#"INSERT INTO "pcc_panel_name_to_athena_document_type" ("pcc_panel_to_athena_document_type_mapping_id", "pcc_panel_name", "cybx_customer_code", "athena_practice_id", "athena_document_type_id", "athena_document_type_name") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(pcc_panel_name_to_athena_document_type.pcc_panel_to_athena_document_type_mapping_id)
            .bind(pcc_panel_name_to_athena_document_type.pcc_panel_name)
            .bind(pcc_panel_name_to_athena_document_type.cybx_customer_code)
            .bind(pcc_panel_name_to_athena_document_type.athena_practice_id)
            .bind(pcc_panel_name_to_athena_document_type.athena_document_type_id)
            .bind(pcc_panel_name_to_athena_document_type.athena_document_type_name)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, pcc_panel_name_to_athena_document_type: PccPanelNameToAthenaDocumentType) -> Result<PccPanelNameToAthenaDocumentType> {
        query_as::<_, PccPanelNameToAthenaDocumentType>(r#"UPDATE "pcc_panel_name_to_athena_document_type" SET "pcc_panel_name" = $2, "cybx_customer_code" = $3, "athena_practice_id" = $4, "athena_document_type_id" = $5, "athena_document_type_name" = $6 WHERE "pcc_panel_to_athena_document_type_mapping_id" = $1 RETURNING *;"#)
            .bind(pcc_panel_name_to_athena_document_type.pcc_panel_to_athena_document_type_mapping_id)
            .bind(pcc_panel_name_to_athena_document_type.pcc_panel_name)
            .bind(pcc_panel_name_to_athena_document_type.cybx_customer_code)
            .bind(pcc_panel_name_to_athena_document_type.athena_practice_id)
            .bind(pcc_panel_name_to_athena_document_type.athena_document_type_id)
            .bind(pcc_panel_name_to_athena_document_type.athena_document_type_name)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "pcc_athena_interop"."pcc_panel_name_to_athena_document_type" WHERE "pcc_panel_to_athena_document_type_mapping_id" = $1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
