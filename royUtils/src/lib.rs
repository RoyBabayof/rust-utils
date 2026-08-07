use std::fs::OpenOptions;
use std::io::Write;

// To Use library declare file path under your dependencies section in your Cargo.toml file.
// By convention use the royUtils in the name declaration.
// like so - royUtils = { git = "https://git@github.com/RoyBabayof/rust-utils.git", package = "royUtils" }

pub fn write_to_file(file_path: &String, data: &String) -> std::io::Result<()>{
    let mut file = OpenOptions::new().append(true).create(true).open(file_path)?;
    writeln!(file, "Hello, world!")?;
    Ok(()) // function returns an err or ok signal
}

pub fn delete_from_file(file_path: &String) -> std::io::Result<()>{
    std::fs::remove_file(file_path)?;
    Ok(())
}


#[cfg(test)]
mod test{
    #[test]
    fn test_write_to_file(){
        let r = write_to_file(&String::from("path.txt"), &String::from("Hello, world!"));
        assert!(r.is_ok());
    }

    #[test]
    fn test_delete_from_file(){
        let r = delete_from_file(&String::from("path.txt"));
        assert!(r.is_ok());
    }
}