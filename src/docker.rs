use std::process::Command;
use std::vec::Vec;
use std::error::Error;

pub fn images() -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("docker")
        .arg("images")
        .output()
        .expect("failed to get list of images");

    let output_str = String::from_utf8(output.stdout)?;
    Ok(output_str.split('\n').map(String::from).collect::<Vec<String>>())
}

#[cfg(test)]
mod test {
    use super::images;
    use std::error::Error;

    #[test]
    fn test_can_run_images() -> Result<(), Box<dyn Error>>{
        images();
        Ok(())
    }
}
