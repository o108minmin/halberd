use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use encoding_rs::SHIFT_JIS;

#[derive(Debug)]
struct TextError(String);

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextError: {}", self.0)
    }
}

impl Error for TextError {}

pub fn generate_subtitle_from_same_name_txt(path: PathBuf) -> Result<String, Box<dyn Error>> {
    info!("Genarate subtitle from txt file");
    info!("input path: {}", &path.to_str().unwrap());
    let mut text_path = path;
    text_path.set_extension("txt");
    info!("Open {}", &text_path.to_str().unwrap());
    let text = match fs::read_to_string(text_path.as_path()) {
        Ok(s) => s,
        Err(_) => return Err(Box::new(TextError("can't open txt file".into())))
    };
    Ok(text)
}

pub fn generate_subtitle_from_same_name_txt_shift_jis(
    path: PathBuf,
) -> Result<String, Box<dyn Error>> {
    info!("Genarate subtitle from Shidt_JIS txt file");
    info!("input path: {}", &path.to_str().unwrap());
    let mut text_path = path;
    text_path.set_extension("txt");
    info!("Open {}", &text_path.to_str().unwrap());
    let rawtxt = match fs::read(text_path.as_path()) {
        Ok(s) => s,
        Err(_) => return Err(Box::new(TextError("can't open txt file".into())))
    };

    // VOICEROIDで出力される文字列はShift_JISのためutf-8に変換する
    let (res, _, _) = SHIFT_JIS.decode(&rawtxt);
    let text = res.into_owned();
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn normal_generate_subtitle_from_same_name_txt() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.txt");
        let mut file = File::create(&input).unwrap();
        let expected = "test for generate_subtitle_from_same_name_txt";
        write!(file, "{}", &expected).unwrap();
        let result = generate_subtitle_from_same_name_txt(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn error_generate_subtitle_from_same_name_txt_not_found() {
        let dir = tempdir().unwrap();
        let result = generate_subtitle_from_same_name_txt(dir.path().join("test.txt"));
        assert!(result.is_err());
        dir.close().unwrap();
    }

    #[test]
    fn normal_generate_subtitle_from_same_name_txt_shift_jis() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.txt");
        let mut file = File::create(&input).unwrap();
        let expected = "test for generate_subtitle_from_same_name_txt_shift_jis";
        let (expected_shift_jis, _, _) = SHIFT_JIS.encode(&expected);
        file.write_all(&expected_shift_jis.into_owned()).unwrap();
        let result = generate_subtitle_from_same_name_txt_shift_jis(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn normal_generate_subtitle_from_same_name_txt_shift_jis_multi_byte() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.txt");
        let mut file = File::create(&input).unwrap();
        let expected = "テスト";
        let (expected_shift_jis, _, _) = SHIFT_JIS.encode(&expected);
        file.write_all(&expected_shift_jis.into_owned()).unwrap();
        let result = generate_subtitle_from_same_name_txt_shift_jis(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn normal_generate_subtitle_from_same_name_txt_shift_jis_unicode() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.txt");
        let mut file = File::create(&input).unwrap();
        let expected = "test for generate_subtitle_from_same_name_txt_shift_jis_unicode";
        write!(file, "{}", &expected).unwrap();
        let result = generate_subtitle_from_same_name_txt_shift_jis(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn normal_generate_subtitle_from_same_name_txt_shift_jis_unicode_multi_byte() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.txt");
        let mut file = File::create(&input).unwrap();
        let mb = "テスト";
        write!(file, "{}", &mb).unwrap();
        let result = generate_subtitle_from_same_name_txt_shift_jis(input);
        assert!(result.is_ok());
        let expected = "繝�せ繝�";
        assert_eq!(result.unwrap(), expected);
        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn error_generate_subtitle_from_same_name_txt_shift_jis_not_found() {
        let dir = tempdir().unwrap();
        let result = generate_subtitle_from_same_name_txt_shift_jis(dir.path().join("test.txt"));
        assert!(result.is_err());
        dir.close().unwrap();
    }
}
