pub mod challenge1;

pub mod input {
    use std::error::Error;
    use std::fs;
    use std::str::FromStr;

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
