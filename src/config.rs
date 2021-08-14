pub struct Config {
    pub softwaretalk_type: String,
    pub dirname: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let softwaretalk_type = match args.next() {
            Some(arg) => arg,
            // softwaretalkの種類がinputされていない
            None => return Err("Didn't get a directory name"),
        };

        let dirname = match args.next() {
            Some(arg) => arg,
            // ディレクトリ名を指定していない
            None => return Err("Didn't get a directory name"),
        };
        Ok(Config {
            softwaretalk_type,
            dirname,
        })
    }
}
