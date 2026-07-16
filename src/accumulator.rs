use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageExtraction {
    pub page_number: usize,
    pub visible_provider: Option<String>,
    pub visible_date: Option<String>,
    pub facility: Option<String>,
    pub encounter_type: Option<String>,
    pub has_structural_header: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EncounterSegment {
    pub provider: Option<String>,
    pub date_of_service: Option<String>,
    pub facility: Option<String>,
    pub encounter_type: Option<String>,
    pub page_numbers: Vec<usize>,
}

pub fn assemble_encounters(extractions: Vec<PageExtraction>) -> Vec<EncounterSegment> {
    let mut encounters: Vec<EncounterSegment> = Vec::new();
    let mut current_encounter: Option<EncounterSegment> = None;

    // Ensure extractions are sorted by page_number
    let mut sorted_extractions = extractions;
    sorted_extractions.sort_by_key(|e| e.page_number);

    for extraction in sorted_extractions {
        let is_boundary = if current_encounter.is_none() {
            true
        } else {
            let active = current_encounter.as_ref().unwrap();
            let date_changed = if let (Some(active_date), Some(new_date)) = (&active.date_of_service, &extraction.visible_date) {
                active_date != new_date
            } else {
                false
            };
            extraction.page_number == 1 || date_changed || extraction.has_structural_header
        };

        if is_boundary {
            if let Some(enc) = current_encounter.take() {
                encounters.push(enc);
            }

            current_encounter = Some(EncounterSegment {
                provider: extraction.visible_provider.clone(),
                date_of_service: extraction.visible_date.clone(),
                facility: extraction.facility.clone(),
                encounter_type: extraction.encounter_type.clone(),
                page_numbers: vec![extraction.page_number],
            });
        } else {
            if let Some(mut active) = current_encounter.take() {
                active.page_numbers.push(extraction.page_number);

                // Merge metadata: carry forward/backward missing fields
                if active.provider.is_none() {
                    active.provider = extraction.visible_provider.clone();
                }
                if active.date_of_service.is_none() {
                    active.date_of_service = extraction.visible_date.clone();
                }
                if active.facility.is_none() {
                    active.facility = extraction.facility.clone();
                }
                if active.encounter_type.is_none() {
                    active.encounter_type = extraction.encounter_type.clone();
                }

                current_encounter = Some(active);
            }
        }
    }

    if let Some(enc) = current_encounter.take() {
        encounters.push(enc);
    }

    encounters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18_pages_accumulation() {
        let mut extractions = Vec::new();

        // Encounter 1 (Pages 1 to 5)
        extractions.push(PageExtraction {
            page_number: 1,
            visible_provider: Some("Dr. Smith".to_string()),
            visible_date: Some("2026-01-10".to_string()),
            facility: Some("Evergreen Clinic".to_string()),
            encounter_type: Some("Routine Progress Note".to_string()),
            has_structural_header: true,
        });
        for p in 2..=5 {
            extractions.push(PageExtraction {
                page_number: p,
                visible_provider: None,
                visible_date: None,
                facility: None,
                encounter_type: None,
                has_structural_header: false,
            });
        }

        // Encounter 2 (Pages 6 to 10)
        extractions.push(PageExtraction {
            page_number: 6,
            visible_provider: Some("Dr. Smith".to_string()),
            visible_date: Some("2026-01-15".to_string()),
            facility: Some("Evergreen Clinic".to_string()),
            encounter_type: Some("Routine Progress Note".to_string()),
            has_structural_header: true,
        });
        for p in 7..=10 {
            extractions.push(PageExtraction {
                page_number: p,
                visible_provider: None,
                visible_date: None,
                facility: None,
                encounter_type: None,
                has_structural_header: false,
            });
        }

        // Encounter 3 (Pages 11 to 14)
        extractions.push(PageExtraction {
            page_number: 11,
            visible_provider: Some("Dr. Adams".to_string()),
            visible_date: Some("2026-01-20".to_string()),
            facility: Some("St. Jude".to_string()),
            encounter_type: Some("Initial Intake".to_string()),
            has_structural_header: true,
        });
        for p in 12..=14 {
            extractions.push(PageExtraction {
                page_number: p,
                visible_provider: None,
                visible_date: None,
                facility: None,
                encounter_type: None,
                has_structural_header: false,
            });
        }

        // Encounter 4 (Pages 15 to 18)
        extractions.push(PageExtraction {
            page_number: 15,
            visible_provider: Some("Dr. Adams".to_string()),
            visible_date: Some("2026-01-20".to_string()), // same date but has_structural_header starts a new one
            facility: Some("St. Jude".to_string()),
            encounter_type: Some("Routine Progress Note".to_string()),
            has_structural_header: true,
        });
        for p in 16..=18 {
            extractions.push(PageExtraction {
                page_number: p,
                visible_provider: None,
                visible_date: None,
                facility: None,
                encounter_type: None,
                has_structural_header: false,
            });
        }

        let encounters = assemble_encounters(extractions);
        assert_eq!(encounters.len(), 4);

        // Encounter 1 verification
        assert_eq!(encounters[0].page_numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(encounters[0].date_of_service, Some("2026-01-10".to_string()));
        assert_eq!(encounters[0].provider, Some("Dr. Smith".to_string()));

        // Encounter 2 verification
        assert_eq!(encounters[1].page_numbers, vec![6, 7, 8, 9, 10]);
        assert_eq!(encounters[1].date_of_service, Some("2026-01-15".to_string()));

        // Encounter 3 verification
        assert_eq!(encounters[2].page_numbers, vec![11, 12, 13, 14]);
        assert_eq!(encounters[2].date_of_service, Some("2026-01-20".to_string()));
        assert_eq!(encounters[2].provider, Some("Dr. Adams".to_string()));

        // Encounter 4 verification
        assert_eq!(encounters[3].page_numbers, vec![15, 16, 17, 18]);
        assert_eq!(encounters[3].date_of_service, Some("2026-01-20".to_string()));
    }
}
