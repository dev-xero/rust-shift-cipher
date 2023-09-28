pub struct Config {
    text: String,
    shift: i8,
    should_export: bool
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str> 
    where
        T: Iterator<Item = String>
    {
        let text = match args.next() {
            Some(arg) => arg,
            None => return Err("A text to encrypt should be provided.")
        };

        let shift = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(num) => num,
                Err(_) => return Err("Failed to parse shift distance")
            },
            None => return Err("A shift distance should be specified.")
        };

        let should_export = match args.next() {
            Some(arg) => {
                if arg == "-e" {
                    true
                } else {
                    false
                }
            },
            None => false
        };

        Ok(Config {
            text,
            shift,
            should_export
        })
    }
}