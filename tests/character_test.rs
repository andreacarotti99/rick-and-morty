#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    #[test]
    fn test_fetch_all_characters() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("fetch-all-characters")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command did not run successfully");

        let output_str = str::from_utf8(&output.stdout).expect("Output is not valid UTF-8");

        assert!(output_str.contains("Rick Sanchez"), "Rick Sanchez was not found in the characters list");
        assert!(output_str.contains("Morty Smith"), "Morty Smith was not found in the characters list");
        assert!(output_str.contains("Summer Smith"), "Morty Smith was not found in the characters list");
        assert!(!output_str.is_empty(), "The characters list should not be empty");
    }

    #[test]
    fn test_fetch_single_character() {
        let character_id = "1"; 
        let expected_character_name = "Rick Sanchez"; 

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("fetch-single-character")
            .arg(character_id)
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command did not run successfully");

        let output_str = str::from_utf8(&output.stdout).expect("Output is not valid UTF-8");

        assert!(output_str.contains(expected_character_name), "The expected character was not found in the command output");
    }

    #[test]
    fn test_fetch_filtered_characters() {
        let status_filter = "Alive";
        let species_filter = "Human";
        let excluded_status = "Dead";
        let excluded_species = "Robot";

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("fetch-filtered-characters")
            .arg("--status")
            .arg(status_filter)
            .arg("--species")
            .arg(species_filter)
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command did not run successfully");

        let output_str = str::from_utf8(&output.stdout).expect("Output is not valid UTF-8");

        
        assert!(output_str.contains(status_filter), "Filtered characters do not match the status filter");
        assert!(output_str.contains(species_filter), "Filtered characters do not match the species filter");
        assert!(!output_str.contains(excluded_status), "Output unexpectedly contains characters with status 'Dead'");
        assert!(!output_str.contains(excluded_species), "Output unexpectedly contains characters of species 'Robot'");
    }

    #[test]
    fn test_fetch_multiple_characters() {
        let character_ids = "1,826"; 
        let expected_names = ["Rick Sanchez", "Butter Robot"]; 

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("fetch-multiple-characters")
            .arg(character_ids)
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command did not run successfully");

        let output_str = str::from_utf8(&output.stdout).expect("Output is not valid UTF-8");

        assert!(output_str.contains(expected_names[0]), "Not found Rick Sanchez'");
        assert!(output_str.contains(expected_names[1]), "Not found Butter Robot");
    }
    
    #[test]
    fn test_fetch_multiple_characters_out_of_bounds() {
        let character_ids = "827,828"; 

        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("fetch-multiple-characters")
            .arg(character_ids)
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command did not run successfully");

        let output_str = str::from_utf8(&output.stdout).expect("Output is not valid UTF-8");
        assert!(output_str.contains("[]"), "Expected an empty list of characters but found some");

    }
}
