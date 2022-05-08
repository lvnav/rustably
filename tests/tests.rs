mod tests {
    use std::{fs::{read_dir, File, ReadDir}, io::Read, path::Path};

    use rustably::parser::Parser;

    #[test]
    fn compare_against_references_files() {
        let comparisons_filepaths = read_dir("./tests/comparison_files").expect("Error during scanning ./comparison_files maybe folder is missing");

    }

    fn get_files(paths: ReadDir) -> Vec<String> {
        let mut comparison_files : Vec<String> = Vec::new();
        for path in paths {
            let path = path.expect("error during iteration over filepaths");
            let mut comparison_file = File::open(path.path()).expect("Error during file opening");
            let mut content = String::new();
            comparison_file.read_to_string(&mut content).expect("error during file read");
            comparison_files.push(content);
        }

        comparison_files
    }

}
