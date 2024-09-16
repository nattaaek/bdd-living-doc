use gherkin::{Feature, GherkinEnv};
use std::{env, fs};
use std::path::Path;

pub fn parse_feature_files() -> Vec<Feature> {
    let mut features = Vec::new();

    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current working directory: {:?}", current_dir);
    
    let feature_files_dir = current_dir.parent().unwrap().join("feature-files");
    println!("Feature files directory: {:?}", feature_files_dir);
    
    let paths = fs::read_dir(&feature_files_dir)
        .unwrap_or_else(|_| panic!("Failed to read directory: {:?}", feature_files_dir));

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("feature") {
            let feature = parse_feature_file(&path);
            if let Some(f) = feature {
                features.push(f);
            }
        }
    }

    features
}

fn parse_feature_file(path: &Path) -> Option<Feature> {
    let content = fs::read_to_string(path).ok()?;
    let env = GherkinEnv::default(); // Create a default parsing environment
    let feature = Feature::parse(&content, env).ok()?;
    Some(feature)
}

