use std::{process, env};
use process::Command;

pub fn generate_testcases(n: i32, package: &str, timelimit: u64) {
    //! Generates testcases for a specified package.
    //! 
    //! Generates testcases for a specified package, based on the files
    //! `generate_tests.cpp` and `brute.cpp`. It reads the stdout stream
    //! from these two files and puts it in the files `index.in` and `index.out`,
    //! where `index` is the test number.
    //! It also provides a time limit (in ms).
    //! 
    //! ```
    //! generate_testcases(30, "main", 4000)
    //! ```
     
    let current_dir = env::current_dir().unwrap();
    println!("Generating testcases in {}", current_dir.display());
    let testcases_dir = current_dir.join("testcases");
    if !testcases_dir.exists() {
        println!("Creating testcases directory");
        std::fs::create_dir(&testcases_dir).unwrap();
    }

    // Compile the test generation algorithms
    Command::new("g++")
        .arg("-std=c++17")
        .arg("-o")
        .arg("test_generator")
        .arg("generate_tests.cpp")
        .status()
        .expect("Failed to compile the input test generation algorithm!");

    Command::new("g++")
        .arg("-std=c++17")
        .arg("-o")
        .arg("output_generator")
        .arg("brute.cpp")
        .status()
        .expect("Failed to compile the output test generation algorithm!");

    // Print empty newline to replace it with following print statements
    println!("");
    // Start generating tests
    for i in 1..=n {
        print!("\rGenerating testcase {}/{}", i, n);
        // Generate input test
        let gent = process::Command::new("./test_generator")
            .arg(i.to_string())
            .arg(n.to_string())
            .output()
            .expect("Failed to generate input tests!");
        
        // Convert the output from stdout to a String
        let gent_stdout = String::from_utf8(gent.stdout).unwrap();

        let testcase_file = testcases_dir
            .join(package)
            .join(format!("{}.in", i));
        
        // Write the output to a file <index>.in
        std::fs::write(&testcase_file, gent_stdout).unwrap();

        // Re-fetch the output as a File, to pipe it to the output generator
        let output_input = std::fs::File::open(testcase_file).unwrap();

        // Generate output test
        let mut out = process::Command::new("./output_generator");
        out.stdin(output_input);

        let out_output = out.output().unwrap();

        // Convert the output to a String
        let out_stdout = String::from_utf8(out_output.stdout).unwrap();

        let out_file = testcases_dir
            .join(package)
            .join(format!("{}.out", i));

        // Write the output file
        std::fs::write(out_file, out_stdout).unwrap();
    }

    // Write the testcases metadata file
    // The metadata file contains the test #, and the time limit (in ms)
    let testcases_file = testcases_dir
        .join(package)
        .join("testcases.txt");

    let testcases_content = format!("{}\n{}", n, timelimit);
    std::fs::write(testcases_file, testcases_content).unwrap();

    // Clean up the executables
    std::fs::remove_file("./test_generator").unwrap();
    std::fs::remove_file("./output_generator").unwrap();

    println!("Successfully generated testcases!");
}

pub fn test(package: &str) {
    //! Test main.cpp using the specified package.
    //! 
    //! Test main.cpp using the specified package called `package`.
    //! Test packages are generated using [`generate_tests()`].
    //! 
    //! ```
    //! test("main")
    //! ```
    let current_dir = env::current_dir().unwrap();

    println!("Testing in {}", current_dir.display());

    let testcases_dir = current_dir
        .join(package)
        .join("testcases");

    let testcases_file = testcases_dir.join("testcases.txt");
    // Read the testcase package metadata
    let testcases_content = std::fs::read_to_string(testcases_file).unwrap();

    // Convert it into a Vec<&str> (split by lines)
    let testcases_content = testcases_content.split("\n").collect::<Vec<&str>>();

    // Get # of testcases and time limit
    let n = testcases_content[0].parse::<i32>().unwrap();
    let timelimit = testcases_content[1].parse::<u64>().unwrap();

    // Compile main program
    process::Command::new("g++")
        .arg("-std=c++17")
        .arg("-o")
        .arg("main")
        .arg("main.cpp")
        .status()
        .expect("Main program failed to compile!");

    // Variable to keep track of how many testcases passed
    let mut passed = 0;

    // Test the program using the testcases
    for i in 1..=n {
        println!("Testing testcase {}", i);
        // Open the input and output files for comparison
        let testcase_file = testcases_dir.join(format!("{}.in", i));
        let testcase_content = std::fs::File::open(testcase_file).unwrap();

        let out_file = testcases_dir.join(format!("{}.out", i));
        let out_content = std::fs::read_to_string(out_file).unwrap();
        let testcase_output = out_content.trim();
        
        // Start timing the program
        let start = std::time::Instant::now();
        
        // Run the program and save the output
        let test_output = process::Command::new("./main")
            .stdin(testcase_content)
            .output()
            .unwrap();
        
        // End timing the program
        let end = std::time::Instant::now();

        // Calculate if the program timed out (running time greater than time limit)
        let did_timeout = end - start > std::time::Duration::from_millis(timelimit);

        let test_output = String::from_utf8(test_output.stdout).unwrap();

        // Check if test output is the same as the program output
        if test_output == testcase_output {
            if !did_timeout {
                print!("\rTestcase {} passed! ({}ms)", i, end.duration_since(start).as_millis());
                passed += 1;
            } else {
                print!("\rTestcase {} passed but took too long! ({}ms)", i, end.duration_since(start).as_millis());
            }
        } else {
            print!("\rTestcase {} failed!", i);
        }
    }

    // Clean the executable up
    std::fs::remove_file("./main").unwrap();

    println!("{}/{} testcases passed!", passed, n);
}
