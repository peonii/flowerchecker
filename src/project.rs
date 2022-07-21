use std::env;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    default_repo: String,
}

pub fn generate_project(name: &str) {
    //! Generate a project using the specified name.
    //! 
    //! Clone the default repository specified in [`~/.config/flowers/config.yml`]
    //! to the directory `name`.
    //! 
    //! ```
    //! generate_project("project")
    //! ```
    
    // Get the current working directory
    let current_dir = env::current_dir().unwrap();

    println!("Generating project in {}", current_dir.display());

    // Clone the default repository
    let home_dir = dirs::home_dir().unwrap();
    let config_file = home_dir.join(".config").join("flowers").join("config.toml");
    let config: Config = toml::from_str(&std::fs::read_to_string(config_file).unwrap()).unwrap();
    let default_repo = config.default_repo;

    let project_dir = current_dir.join(name);
    println!("Cloning default repo");
    std::process::Command::new("git")
        .arg("clone")
        .arg(default_repo)
        .arg(&project_dir)
        .status()
        .expect("Failed to clone repository!");

    // Remove the .git files
    rm_rf::remove(project_dir.join(".git")).expect("Error removing .git!");
    match rm_rf::remove(project_dir.join(".gitignore")) {
        Ok(_) => {},
        Err(_) => {}
    }

    // Reinitialize git repository for version control
    std::process::Command::new("git")
        .arg("init")
        .arg(project_dir.as_path())
        .status()
        .expect("Failed to re-initialize repository!");
    
    println!("Successfully generated project!");
}
