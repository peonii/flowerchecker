use std::env;

pub fn first_install() {
    //! Creates the necessary directories and files for flowerchecker to work.
    //! 
    //! Creates the directories `flowers` and `flowers/projects` in the current
    //! working directory, and generates a `config.yml` file in the ~./config/flowers
    //! directory.
    //! 
    //! ```
    //! first_install()
    //! ```
    let current_dir = env::current_dir().unwrap();

    println!("Installing in {}", current_dir.display());

    // Create a flowers directory
    let flowers_dir = current_dir.join("flowers");

    if !flowers_dir.exists() {
        println!("Creating flowers directory");
        std::fs::create_dir(&flowers_dir).unwrap();
    }

    let default_config = String::from("
# Which repository to use when generating project 
default_repo = \"https://github.com/raisadesu/oisuite-files.git\"
");
    
    let home_dir = dirs::home_dir().unwrap();

    // Create a ~/.config/flowers/config.yml file 
    let config_file = home_dir.join(".config").join("flowers").join("config.toml");
    let flowers_config_path = home_dir.join(".config").join("flowers");
    if !config_file.exists() {
        print!("\rCreating config.yml");
        std::fs::create_dir_all(&flowers_config_path).unwrap();
        std::fs::write(config_file, default_config).unwrap();
    }

    // Create a flowers/projects dir 
    let projects_dir = flowers_dir.join("projects");
    if !projects_dir.exists() {
        print!("\rCreating projects directory");
        std::fs::create_dir(&projects_dir).unwrap();
    }

    println!("Successfully installed flowerchecker!");
}

