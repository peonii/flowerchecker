use std::env;

mod install;
mod project;
mod testing;

fn main() {

    // Get arguments passed in
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: flowerchecker <command>");
        println!("Commands: install, project");
        std::process::exit(1);
    }
    
    let command = args[1].as_str();

    match command {
        "install" => install::first_install(),
        "new" => {
            if args.len() == 2 {
                println!("Usage: flowerchecker new <project_name>");
                std::process::exit(1);
            }
            project::generate_project(&args[2]);
        },
        "generate" => {
            if args.len() == 4 {
                println!("Usage: flowerchecker generate <package_name> <testcase_number> <max_time>");
                std::process::exit(1);
            }

            testing::generate_testcases(
                args[3].parse::<i32>().unwrap(),
                &args[2], 
                args[4].parse::<u64>().unwrap()
            );
        },
        "test" => {
            if args.len() == 2 {
                println!("Usage: flowerchecker test <package_name>");
                std::process::exit(1);
            }
            testing::test(&args[2]);
        }
        _ => println!("Unknown command"),
    }

}
