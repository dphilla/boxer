#[cfg(test)]
mod integration_tests {
    use std::process::Command;
    use std::fs::File;
    use std::io::{Write, BufReader, BufRead};
    use std::path::Path;

    // A utility function to create a temporary Dockerfile with given content and return its path
    fn create_temp_dockerfile(content: &str) -> String {
        let path = "temp_Dockerfile";
        let mut file = File::create(path).expect("Failed to create temporary Dockerfile.");
        writeln!(file, "{}", content).expect("Failed to write to temporary Dockerfile.");
        path.to_string()
    }

    // Read the output of a file
    fn read_file_output(path: &str) -> String {
        let file = File::open(path).expect("Failed to open file.");
        let reader = BufReader::new(file);
        reader.lines().collect::<Result<Vec<_>, _>>().expect("Failed to read lines.").join("\n")
    }

    // Test parsing valid directives
    #[test]
    fn test_valid_directives() {
        let dockerfile_content = "FROM ubuntu\nRUN echo 'hello world'";
        let dockerfile_path = create_temp_dockerfile(dockerfile_content);
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(&dockerfile_path)
            .output()
            .expect("Failed to execute command.");

        let expected_output = "FROM: ()Initializes a new build stage and sets the base layer.\nRUN: Executes commands in a new layer on top of the current layer and commits the results.";
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
    }

    // Test parsing unrecognized directives
    #[test]
    fn test_unrecognized_directives() {
        let dockerfile_content = "UNKNOWN ubuntu";
        let dockerfile_path = create_temp_dockerfile(dockerfile_content);
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(&dockerfile_path)
            .output()
            .expect("Failed to execute command.");

        assert!(output.status.success() == false);
        assert!(String::from_utf8_lossy(&output.stderr).contains("Directive UNKNOWN not recognized"));
    }

    // Test handling of comments and empty lines
    #[test]
    fn test_comments_and_empty_lines() {
        let dockerfile_content = "# This is a comment\n\nFROM ubuntu";
        let dockerfile_path = create_temp_dockerfile(dockerfile_content);
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg(&dockerfile_path)
            .output()
            .expect("Failed to execute command.");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "FROM: ()Initializes a new build stage and sets the base layer.");
    }

    // Test behavior with no file path provided
    #[test]
    fn test_no_file_path() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .output()
            .expect("Failed to execute command.");

        assert!(output.status.success() == false);
        assert!(String::from_utf8_lossy(&output.stderr).contains("Usage:"));
    }

    // Test behavior with a non-existent file path
    #[test]
    fn test_non_existent_file_path() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("non_existent_file")
            .output()
            .expect("Failed to execute command.");

        assert!(output.status.success() == false);
        assert!(String::from_utf8_lossy(&output.stderr).contains("No such file or directory"));
    }

    // Cleanup any created temporary files
    #[test]
    fn cleanup() {
        std::fs::remove_file("temp_Dockerfile").expect("Failed to remove temporary Dockerfile.");
    }
}

