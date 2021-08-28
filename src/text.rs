use std::fs;
use std::path::PathBuf;

pub fn generate_subtitle_from_same_name_txt(path: PathBuf) -> Result<String, &'static str> {
    info!("Genarate subtitle from txt file");
    info!("input path: {}", &path.to_str().unwrap());
    let mut text_path = path;
    text_path.set_extension("txt");
    info!("Open {}", &text_path.to_str().unwrap());
    let text = fs::read_to_string(text_path.as_path()).expect("can't open txt file");
    Ok(text)
}

pub fn generate_subtitle_from_same_name_txt_shift_jis(
    path: PathBuf,
) -> Result<String, &'static str> {
    info!("Genarate subtitle from Shidt_JIS txt file");
    info!("input path: {}", &path.to_str().unwrap());
    let mut text_path = path;
    text_path.set_extension("txt");
    info!("Open {}", &text_path.to_str().unwrap());
    let rawtxt = fs::read(text_path.as_path()).expect("can't open txt file");
    // VOICEROIDで出力される文字列はShift_JISのためutf-8に変換する
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&rawtxt);
    let text = res.into_owned();
    Ok(text)
}
