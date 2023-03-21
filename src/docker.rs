use std::process::Command;
use std::vec::Vec;
use std::error::Error;

#[derive(Debug)]
pub struct DockerImage {
    repository: String,
    tag: String,
    image_id: String,
    size: String,
    // TODO: make this a proper date/time type
    created_at: String
}

pub fn images() -> Result<Vec<DockerImage>, Box<dyn Error>> {
    let output = Command::new("docker")
        .args(["images", "--format", "{{.ID}}:{{.Repository}}:{{.Tag}}:{{.Size}}:{{.CreatedAt}}"])
        .output()
        .expect("failed to get list of images");

    let output_str = String::from_utf8(output.stdout)?;
    let output_lines = output_str
        .split('\n')
        .map(String::from)
        .filter_map(|i: String| {
            let parts = i
                .splitn(5, ':')
                .map(String::from)
                .collect::<Vec<String>>();
            let d_struct = 
                match &parts[..] {
                    [image_id, repository, tag, size, created_at] => Some(DockerImage {
                        image_id: image_id.clone(),
                        repository: repository.clone(),
                        tag: tag.clone(),
                        size: size.clone(),
                        created_at: created_at.clone(),
                    }),
                    _ => None
                };
            d_struct
        })
        .collect::<Vec<DockerImage>>();
    println!("{:?}", output_lines);
    Ok(output_lines)
}

#[cfg(test)]
mod test {
    use super::images;
    use std::error::Error;

    #[test]
    fn test_can_run_images() -> Result<(), Box<dyn Error>>{
        let image_list = images()?;
        Ok(())
    }
}
