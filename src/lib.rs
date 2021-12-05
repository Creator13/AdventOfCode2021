pub mod challenge1;
pub mod challenge2;
pub mod challenge3;
pub mod challenge4;
pub mod challenge5;

pub mod input {
    use std::error::Error;
    use std::fs;
    use std::str::FromStr;
    use std::string::String;

    pub fn read(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;
        Ok(contents.lines().map(|line| line.to_string()).collect())
    }

    pub fn read_and_parse<T: FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn Error + 'static>>
    where
        T::Err: Error + 'static,
    {
        let contents = fs::read_to_string(filename)?;

        contents
            .lines()
            .map(|line| line.parse().map_err(|e: T::Err| e.into()))
            .collect()
    }
}
