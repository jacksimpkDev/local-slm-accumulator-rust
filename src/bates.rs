pub struct BatesProfile {
    pub prefix: Option<String>,
    pub start_num: Option<i32>,
}

pub fn generate_citation(
    profile: &BatesProfile,
    page_number: usize,
    encounter_date: &str,
    encounter_type: &str,
) -> String {
    if let (Some(prefix), Some(start_num)) = (&profile.prefix, profile.start_num) {
        let bates_val = start_num + (page_number as i32) - 1;
        format!("{}_{:06} [{} - {}]", prefix, bates_val, encounter_date, encounter_type)
    } else {
        format!("Physical Page {} [{} - {}]", page_number, encounter_date, encounter_type)
    }
}
