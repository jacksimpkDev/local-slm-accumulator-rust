mod accumulator;
mod bates;

use accumulator::{assemble_encounters, PageExtraction};
use bates::{generate_citation, BatesProfile};

fn main() {
    println!("=== PHASE 1: Loading Raw Page Extractions ===");
    let mut extractions = Vec::new();

    // Replicating our 18-page scenario
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
        visible_date: Some("2026-01-20".to_string()),
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

    for extraction in &extractions {
        println!(
            "Page {:02} | Provider: {:?} | Date: {:?} | Structural Header: {}",
            extraction.page_number,
            extraction.visible_provider,
            extraction.visible_date,
            extraction.has_structural_header
        );
    }

    println!("\n=== PHASE 2: Assembling Grouped Encounters (Sparse Metadata Merged) ===");
    let encounters = assemble_encounters(extractions.clone());
    for (idx, enc) in encounters.iter().enumerate() {
        println!(
            "Encounter #{} | Pages {:?} | Date: {:?} | Provider: {:?} | Facility: {:?} | Type: {:?}",
            idx + 1,
            enc.page_numbers,
            enc.date_of_service,
            enc.provider,
            enc.facility,
            enc.encounter_type
        );
    }

    println!("\n=== PHASE 3: Generating Timeline Citations (Physical Page Fallback) ===");
    let unseeded_profile = BatesProfile {
        prefix: None,
        start_num: None,
    };
    for enc in &encounters {
        let enc_date = enc.date_of_service.as_deref().unwrap_or("Unknown Date");
        let enc_type = enc.encounter_type.as_deref().unwrap_or("Unknown Type");
        for &p in &enc.page_numbers {
            let citation = generate_citation(&unseeded_profile, p, enc_date, enc_type);
            println!("  Citation: {}", citation);
        }
    }

    println!("\n=== PHASE 4: Applying Bates Profile & Generating Dynamic Citations ===");
    let seeded_profile = BatesProfile {
        prefix: Some("CHAMBERS".to_string()),
        start_num: Some(100),
    };
    for enc in &encounters {
        let enc_date = enc.date_of_service.as_deref().unwrap_or("Unknown Date");
        let enc_type = enc.encounter_type.as_deref().unwrap_or("Unknown Type");
        for &p in &enc.page_numbers {
            let citation = generate_citation(&seeded_profile, p, enc_date, enc_type);
            println!("  Citation: {}", citation);
        }
    }
}
