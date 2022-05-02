//! txtファイル関係のモジュール
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use encoding_rs::{SHIFT_JIS, UTF_16LE, UTF_8};

#[derive(Debug)]
struct TextError(String);

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextError: {}", self.0)
    }
}

impl Error for TextError {}

/// 引数pathと同じ名前のtxtファイル(utf-8)の中身を読む
/// * `path` - 対象のファイル
/// ok path = "01.wav" かつ 01.txtが存在する
pub fn generate_subtitle_from_same_name_txt(path: PathBuf) -> Result<String, Box<dyn Error>> {
    info!("Genarate subtitle from txt file");
    info!("input path: {}", &path.to_str().unwrap());
    let mut text_path = path;
    text_path.set_extension("txt");
    info!("Open {}", &text_path.to_str().unwrap());
    let rawtxt = match fs::read(text_path.as_path()) {
        Ok(s) => s,
        Err(_) => return Err(Box::new(TextError("can't open txt file".into()))),
    };
    let (res, _, _) = UTF_8.decode(&rawtxt);
    let ans = res.into_owned();
    Ok(ans)
}

/// 引数pathと同じ名前のtxtファイル(Shift_JIS))の中身を読む
/// * `path` - 対象のファイル
/// ok path = "01.wav" かつ 01.txtが存在する
pub fn generate_subtitle_from_same_name_txt_shift_jis(
    path: PathBuf,
) -> Result<String, Box<dyn Error>> {
    info!("Genarate subtitle from Shift_JIS txt file");
    info!("input path: {}", &path.to_str().unwrap());
    let mut text_path = path;
    text_path.set_extension("txt");
    info!("Open {}", &text_path.to_str().unwrap());
    let rawtxt = match fs::read(text_path.as_path()) {
        Ok(s) => s,
        Err(_) => return Err(Box::new(TextError("can't open txt file".into()))),
    };

    let (res, _, _) = SHIFT_JIS.decode(&rawtxt);
    let text = res.into_owned();
    Ok(text)
}

/// 引数pathと同じ名前のtxtファイル(UTF-16 LE))の中身を読む
/// * `path` - 対象のファイル
/// ok path = "01.wav" かつ 01.txtが存在する
pub fn generate_subtitle_from_same_name_txt_utf_16le(
    path: PathBuf,
) -> Result<String, Box<dyn Error>> {
    info!("Genarate subtitle from UTF_16LE txt file");
    info!("input path: {}", &path.to_str().unwrap());
    let mut text_path = path;
    text_path.set_extension("txt");
    info!("Open {}", &text_path.to_str().unwrap());
    let rawtxt = match fs::read(text_path.as_path()) {
        Ok(s) => s,
        Err(_) => return Err(Box::new(TextError("can't open txt file".into()))),
    };
    let (res, _, _) = UTF_16LE.decode(&rawtxt);
    let answer = res.into_owned();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    // 入力値が正常だったとき
    fn normal_generate_subtitle_from_same_name_txt() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.wav");
        let input_text = dir.path().join("test.txt");
        let mut file = File::create(&input_text).unwrap();
        let expected = "test for generate_subtitle_from_same_name_txt";
        write!(file, "{}", &expected).unwrap();
        let result = generate_subtitle_from_same_name_txt(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        drop(file);
        dir.close().unwrap();
    }

    #[test]
    // 入力値が異常だったとき(txtファイルが存在しない)
    fn error_generate_subtitle_from_same_name_txt_not_found() {
        let dir = tempdir().unwrap();
        let result = generate_subtitle_from_same_name_txt(dir.path().join("test.txt"));
        assert!(result.is_err());
        dir.close().unwrap();
    }

    #[test]
    // 入力値が正常だったとき
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
    // 入力値が正常だったとき(txtファイルの中身がマルチバイト文字)

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
    // 入力値が正常だったとき(txtファイルの中身がutf-8)
    // 入力値がShift_JISかつ英数字の時は文字化けしない
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
    // 入力値が正常だったとき(txtファイルの中身がutf-8)
    // 入力値がutf-8かつマルチバイトの時は文字化けした状態で出力される
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
    // 入力値が異常だったとき(txtファイルが存在しない)
    fn error_generate_subtitle_from_same_name_txt_shift_jis_not_found() {
        let dir = tempdir().unwrap();
        let result = generate_subtitle_from_same_name_txt_shift_jis(dir.path().join("test.txt"));
        assert!(result.is_err());
        dir.close().unwrap();
    }
}
