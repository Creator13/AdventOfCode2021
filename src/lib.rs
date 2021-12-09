pub mod challenge1;
pub mod challenge2;
pub mod challenge3;
pub mod challenge4;
pub mod challenge5;
pub mod challenge6;
pub mod challenge7;
pub mod challenge8;
pub mod challenge9;

pub mod input {
    use std::error::Error;
    use std::fs;
    use std::str::FromStr;
    use std::string::String;

    pub fn read_lines(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
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

    pub fn read_numbers<T: FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn Error + 'static>>
    where
        T::Err: Error + 'static,
    {
        let mut contents = fs::read_to_string(filename)?;
        contents = contents.trim().to_string();
        contents
            .split(",")
            .map(|val| val.parse().map_err(|e: T::Err| e.into()))
            .collect()
    }
}
