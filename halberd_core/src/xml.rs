//! xmlファイル関係のモジュール
use std::error::Error;
use std::fmt;
use std::io::Write;

use log::{debug, info};
use time::{Duration, OffsetDateTime, format_description};
use xml::common::XmlVersion;
use xml::writer::{EmitterConfig, XmlEvent};

use crate::unitsubrip::UnitSubRip;

impl Error for XmlError {}

#[derive(Debug)]
struct XmlError(String);

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XmlError: {}", self.0)
    }
}

/// 引数wに対して、vecをxmlファイルとして出力する
///
/// # Errors
///
/// 出力先への書き込み、ローカル時刻または時刻形式の取得、あるいは字幕の表示時間が正でない場合にエラーを返します。
pub fn output_xml<W: Write>(
    w: &mut W,
    vec: Vec<UnitSubRip>,
    use_timestamp: bool,
    event_name: String,
) -> Result<(), Box<dyn Error>> {
    info!("Print UnitSubRips");
    debug!("input vec length: {}", vec.len());
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false)
        .create_writer(w);
    writer.write(XmlEvent::StartDocument {
        version: XmlVersion::Version10,
        encoding: Some("utf-8"),
        standalone: None,
    })?;
    let s = XmlEvent::start_element("fcpxml").attr("version", "1.8");
    writer.write(s)?;
    let resources_s = XmlEvent::start_element("resources");
    writer.write(resources_s)?;

    let format_s = XmlEvent::start_element("format")
        .attr("id", "r1")
        .attr("name", "FFVideoFormat1080p2398")
        .attr("frameDuration", "1001/24000s")
        .attr("width", "1920")
        .attr("height", "1080")
        .attr("colorSpace", "1-1-1 (Rec. 709)");
    writer.write(format_s)?;
    let format_e = XmlEvent::end_element();
    writer.write(format_e)?;

    let effect_s = XmlEvent::start_element("effect")
        .attr("id", "r2")
        .attr("name", "text")
        .attr(
            "uid",
            ".../Titles.localized/Build In:Out.localized/Custom.localized/Custom.moti",
        );
    writer.write(effect_s)?;

    let effect_e = XmlEvent::end_element();
    writer.write(effect_e)?;

    let resources_e = XmlEvent::end_element();
    writer.write(resources_e)?;

    let library_s = XmlEvent::start_element("library");
    writer.write(library_s)?;
    let mut event = event_name;
    if use_timestamp {
        let now = OffsetDateTime::now_local()?;
        let format = format_description::parse_borrowed::<1>(
            "-[year]-[month]-[day]-[hour]-[minute]-[second]",
        )?;
        let timestamp = now.format(&format)?;
        event += timestamp.as_str();
    }

    let event_s = XmlEvent::start_element("event").attr("name", event.as_str());
    writer.write(event_s)?;

    let project_s = XmlEvent::start_element("project").attr("name", event.as_str());
    writer.write(project_s)?;

    let sequence_s = XmlEvent::start_element("sequence").attr("format", "r1");
    writer.write(sequence_s)?;

    let spine_s = XmlEvent::start_element("spine");
    writer.write(spine_s)?;

    // 字幕
    let mut cursor = Duration::milliseconds(0);
    for (counter, i) in (1..).zip(vec.iter()) {
        let offset = format!("{}/1000s", (cursor.as_seconds_f64() * 1000.0) as i64);
        let name = format!("{counter}");
        let duration = format!("{}/1000s", (i.duration.as_seconds_f64() * 1000.0) as i64);
        let title_s = XmlEvent::start_element("title")
            .attr("ref", "r2")
            .attr("offset", &offset)
            .attr("name", &name)
            .attr("start", &offset)
            .attr("duration", &duration);
        writer.write(title_s)?;

        let text_s = XmlEvent::start_element("text");
        writer.write(text_s)?;
        let text_style_s = XmlEvent::start_element("text-style").attr("ref", "ts1");
        writer.write(text_style_s)?;
        let text = XmlEvent::characters(&i.serif);
        writer.write(text)?;
        let text_style_e = XmlEvent::end_element();
        writer.write(text_style_e)?;
        let text_e = XmlEvent::end_element();
        writer.write(text_e)?;

        if counter == 1 {
            let text_style_def_s = XmlEvent::start_element("text-style-def").attr("id", "ts1");
            writer.write(text_style_def_s)?;
            let text_style_s = XmlEvent::start_element("text-style")
                .attr("font", "Helvetica")
                .attr("fontSize", "72")
                .attr("fontFace", "Regular")
                .attr("fontColor", "1 0.999974 0.999991 1")
                .attr("alignment", "center");
            writer.write(text_style_s)?;
            let text_style_e = XmlEvent::end_element();
            writer.write(text_style_e)?;
            let text_style_def_e = XmlEvent::end_element();
            writer.write(text_style_def_e)?;
        }

        let title_e = XmlEvent::end_element();
        writer.write(title_e)?;
        if i.duration > time::Duration::ZERO {
            cursor = cursor + i.duration + Duration::milliseconds(1);
        } else {
            return Err(Box::new(XmlError(
                "invalid duration: duration must be positive".into(),
            )));
        }
    }

    let spine_e = XmlEvent::end_element();
    writer.write(spine_e)?;
    let sequence_e = XmlEvent::end_element();
    writer.write(sequence_e)?;
    let project_e = XmlEvent::end_element();
    writer.write(project_e)?;

    let event_e = XmlEvent::end_element();
    writer.write(event_e)?;

    let library_e = XmlEvent::end_element();
    writer.write(library_e)?;

    let e = XmlEvent::end_element();
    writer.write(e)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    // 外部のxmlファイルを使いたいので正常系テストは結合テストで実施する

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("write failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn error_xml_write_failure() {
        let mut writer = FailingWriter;
        let result = output_xml(&mut writer, vec![], false, "event".to_string());

        assert!(result.is_err());
    }

    #[test]
    // 不正な値(負の値)が入る
    fn error_xml_invalid_duration() {
        let mut buf = Vec::<u8>::new();
        let input = vec![UnitSubRip {
            duration: time::Duration::seconds(-1),
            serif: String::from("negative duration"),
        }];
        assert!(
            output_xml(&mut buf, input, false, "event".to_string()).is_err(),
            "invalid duration: duration must be positive"
        );
    }
}
