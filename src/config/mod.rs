#[derive(Debug, PartialEq)]
pub struct Config {
    pub text: String,
    pub shift: i8,
    pub should_decrypt: bool,
    pub should_export: bool
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str> 
    where
        T: Iterator<Item = String>
    {
        args.next();

        let should_decrypt = match args.next() {
            Some(arg) => match arg.as_str() {
                "-en" => false,
                "-de" => true,
                _ => return Err("Only one of '-en' or '-de' should be provided.")
            }
            None => return Err("-en or -de must be supplied.") 
        };

        let text = match args.next() {
            Some(arg) => arg,
            None => return Err("A text to encrypt should be provided.")
        };

        let shift = match args.next() {
            Some(arg) => {
                match arg.parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Failed to parse shift distance")
                }
            },
            None => return Err("A shift distance should be specified.")
        };

        let should_export = match args.next() {
            Some(arg) => if arg == "-e" { true } else { false },
            None => false
        };

        Ok(Config {
            should_decrypt,
            text,
            shift,
            should_export
        })
    }
}