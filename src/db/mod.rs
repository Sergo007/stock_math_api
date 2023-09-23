#![allow(dead_code)]
// Generated with sql-gen
//https://github.com/jayy-lmao/sql-gen

pub mod alembic_version;
pub use alembic_version::AlembicVersion;
pub mod alembic_version_db_set;
pub use alembic_version_db_set::AlembicVersionSet;

pub mod allergies_mappings;
pub use allergies_mappings::AllergiesMappings;
pub mod allergies_mappings_db_set;
pub use allergies_mappings_db_set::AllergiesMappingsSet;

pub mod athena_medications;
pub use athena_medications::AthenaMedications;
pub mod athena_medications_db_set;
pub use athena_medications_db_set::AthenaMedicationsSet;

pub mod diagnosis_mappings;
pub use diagnosis_mappings::DiagnosisMappings;
pub mod diagnosis_mappings_db_set;
pub use diagnosis_mappings_db_set::DiagnosisMappingsSet;

pub mod diagnostic_reports_mappings;
pub use diagnostic_reports_mappings::DiagnosticReportsMappings;
pub mod diagnostic_reports_mappings_db_set;
pub use diagnostic_reports_mappings_db_set::DiagnosticReportsMappingsSet;

pub mod facility_mappings;
pub use facility_mappings::FacilityMappings;
pub mod facility_mappings_db_set;
pub use facility_mappings_db_set::FacilityMappingsSet;

pub mod facility_onboardings;
pub use facility_onboardings::FacilityOnboardings;
pub mod facility_onboardings_db_set;
pub use facility_onboardings_db_set::FacilityOnboardingsSet;

pub mod medical_events;
pub use medical_events::MedicalEvents;
pub mod medical_events_db_set;
pub use medical_events_db_set::MedicalEventsSet;

pub mod medication_mappings;
pub use medication_mappings::MedicationMappings;
pub mod medication_mappings_db_set;
pub use medication_mappings_db_set::MedicationMappingsSet;

pub mod medications_ids_rxnorm_mappings;
pub use medications_ids_rxnorm_mappings::MedicationsIdsRxnormMappings;
pub mod medications_ids_rxnorm_mappings_db_set;
pub use medications_ids_rxnorm_mappings_db_set::MedicationsIdsRxnormMappingsSet;

pub mod medications_no_mapped;
pub use medications_no_mapped::MedicationsNoMapped;
pub mod medications_no_mapped_db_set;
pub use medications_no_mapped_db_set::MedicationsNoMappedSet;

pub mod patient_mappings;
pub use patient_mappings::PatientMappings;
pub mod patient_mappings_db_set;
pub use patient_mappings_db_set::PatientMappingsSet;

pub mod pcc_lab_result_webhooks;
pub use pcc_lab_result_webhooks::PccLabResultWebhooks;
pub mod pcc_lab_result_webhooks_db_set;
pub use pcc_lab_result_webhooks_db_set::PccLabResultWebhooksSet;

pub mod pcc_medications_no_mapped;
pub use pcc_medications_no_mapped::PccMedicationsNoMapped;
pub mod pcc_medications_no_mapped_db_set;
pub use pcc_medications_no_mapped_db_set::PccMedicationsNoMappedSet;

pub mod pcc_panel_name_to_athena_document_type;
pub use pcc_panel_name_to_athena_document_type::PccPanelNameToAthenaDocumentType;
pub mod pcc_panel_name_to_athena_document_type_db_set;
pub use pcc_panel_name_to_athena_document_type_db_set::PccPanelNameToAthenaDocumentTypeSet;

pub mod pcc_patient_status_history;
pub use pcc_patient_status_history::PccPatientStatusHistory;
pub mod pcc_patient_status_history_db_set;
pub use pcc_patient_status_history_db_set::PccPatientStatusHistorySet;

pub mod practitioners;
pub use practitioners::Practitioners;
pub mod practitioners_db_set;
pub use practitioners_db_set::PractitionersSet;

pub mod progress_notes_from_pcc_to_athena;
pub use progress_notes_from_pcc_to_athena::ProgressNotesFromPccToAthena;
pub mod progress_notes_from_pcc_to_athena_db_set;
pub use progress_notes_from_pcc_to_athena_db_set::ProgressNotesFromPccToAthenaSet;

pub mod progress_notes_types_from_pcc_to_athena;
pub use progress_notes_types_from_pcc_to_athena::ProgressNotesTypesFromPccToAthena;
pub mod progress_notes_types_from_pcc_to_athena_db_set;
pub use progress_notes_types_from_pcc_to_athena_db_set::ProgressNotesTypesFromPccToAthenaSet;

pub mod unmapped_diagnosis_mappings;
pub use unmapped_diagnosis_mappings::UnmappedDiagnosisMappings;
pub mod unmapped_diagnosis_mappings_db_set;
pub use unmapped_diagnosis_mappings_db_set::UnmappedDiagnosisMappingsSet;


pub struct PccAthenaInteropDevContext;

impl PccAthenaInteropDevContext {
  pub fn alembic_version(&self) -> AlembicVersionSet { AlembicVersionSet }

  pub fn allergies_mappings(&self) -> AllergiesMappingsSet { AllergiesMappingsSet }

  pub fn athena_medications(&self) -> AthenaMedicationsSet { AthenaMedicationsSet }

  pub fn diagnosis_mappings(&self) -> DiagnosisMappingsSet { DiagnosisMappingsSet }

  pub fn diagnostic_reports_mappings(&self) -> DiagnosticReportsMappingsSet { DiagnosticReportsMappingsSet }

  pub fn facility_mappings(&self) -> FacilityMappingsSet { FacilityMappingsSet }

  pub fn facility_onboardings(&self) -> FacilityOnboardingsSet { FacilityOnboardingsSet }

  pub fn medical_events(&self) -> MedicalEventsSet { MedicalEventsSet }

  pub fn medication_mappings(&self) -> MedicationMappingsSet { MedicationMappingsSet }

  pub fn medications_ids_rxnorm_mappings(&self) -> MedicationsIdsRxnormMappingsSet { MedicationsIdsRxnormMappingsSet }

  pub fn medications_no_mapped(&self) -> MedicationsNoMappedSet { MedicationsNoMappedSet }

  pub fn patient_mappings(&self) -> PatientMappingsSet { PatientMappingsSet }

  pub fn pcc_lab_result_webhooks(&self) -> PccLabResultWebhooksSet { PccLabResultWebhooksSet }

  pub fn pcc_medications_no_mapped(&self) -> PccMedicationsNoMappedSet { PccMedicationsNoMappedSet }

  pub fn pcc_panel_name_to_athena_document_type(&self) -> PccPanelNameToAthenaDocumentTypeSet { PccPanelNameToAthenaDocumentTypeSet }

  pub fn pcc_patient_status_history(&self) -> PccPatientStatusHistorySet { PccPatientStatusHistorySet }

  pub fn practitioners(&self) -> PractitionersSet { PractitionersSet }

  pub fn progress_notes_from_pcc_to_athena(&self) -> ProgressNotesFromPccToAthenaSet { ProgressNotesFromPccToAthenaSet }

  pub fn progress_notes_types_from_pcc_to_athena(&self) -> ProgressNotesTypesFromPccToAthenaSet { ProgressNotesTypesFromPccToAthenaSet }

  pub fn unmapped_diagnosis_mappings(&self) -> UnmappedDiagnosisMappingsSet { UnmappedDiagnosisMappingsSet }

}