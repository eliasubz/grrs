use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let path = "test_fi le/test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    println!("file content: {}", content);
    Ok(())
}

// fn main() {
//     let args = Cli::parse();
//     let result = std::fs::read_to_string("test.txt");
//     match result {
//         Ok(content) => {
//             println!("File content: {}", content);
//         }
//         Err(error) => {
//             println!("Oh noes: {}", error);
//         }
//     }

//     let content = std::fs::read_to_string(&args.path).expect("could not read file");
//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }

//     println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
// }
