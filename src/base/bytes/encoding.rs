use prebindgen_proc_macro::prebindgen;

use crate::Encoding;

/// Numeric id of the encoding (u16 widened to i32 for JVM).
#[prebindgen]
pub fn encoding_id(e: &Encoding) -> i32 {
    e.id() as i32
}

/// Clone an encoding into an owned handle (cheap; materializes an owned
/// encoding from a borrowed/predefined one).
#[prebindgen]
pub fn encoding_clone(e: &Encoding) -> Encoding {
    e.clone()
}

/// Optional textual schema attached to the encoding.
#[prebindgen]
pub fn encoding_schema(e: &Encoding) -> Option<String> {
    e.schema()
        .and_then(|s| std::str::from_utf8(s).ok().map(str::to_string))
}

/// Canonical display string for a [`Encoding`] (upstream `Display` impl).
#[prebindgen]
pub fn encoding_to_string(e: &Encoding) -> String {
    e.to_string()
}

/// Parse a textual encoding into a [`Encoding`] (upstream `From<String>`:
/// known names resolve to their canonical id; everything else is preserved
/// under the custom-encoding id).
#[prebindgen]
pub fn encoding_from_string(s: String) -> Encoding {
    Encoding::from(s)
}

/// Build a [`Encoding`] from its numeric id + optional schema (upstream
/// `Encoding::new`) — the inverse of [`encoding_id`] / [`encoding_schema`],
/// for adapters that carry encodings as `(id, schema)` pairs.
#[prebindgen]
pub fn encoding_from_id(id: i32, schema: Option<String>) -> Encoding {
    Encoding::new(id as u16, schema.map(|s| s.into_bytes().into()))
}

/// Return a copy of `e` with `schema` attached (upstream `with_schema`). Zenoh
/// leaves schema semantics to the application (e.g. `utf-8` for `text/plain`).
#[prebindgen]
pub fn encoding_with_schema(e: &Encoding, schema: String) -> Encoding {
    e.clone().with_schema(schema)
}

// ── Predefined-constant accessors ─────────────────────────────────────────
// Each predefined encoding is stored as a `static` backed by the upstream
// `const` value so that callers receive a `&'static Encoding` — a permanent,
// shared pointer that must NOT be freed.

pub static ENCODING_ZENOH_BYTES: Encoding = Encoding::ZENOH_BYTES;
/// Predefined `ZENOH_BYTES` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_zenoh_bytes() -> &'static Encoding {
    &ENCODING_ZENOH_BYTES
}

pub static ENCODING_ZENOH_STRING: Encoding = Encoding::ZENOH_STRING;
/// Predefined `ZENOH_STRING` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_zenoh_string() -> &'static Encoding {
    &ENCODING_ZENOH_STRING
}

pub static ENCODING_ZENOH_SERIALIZED: Encoding = Encoding::ZENOH_SERIALIZED;
/// Predefined `ZENOH_SERIALIZED` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_zenoh_serialized() -> &'static Encoding {
    &ENCODING_ZENOH_SERIALIZED
}

pub static ENCODING_APPLICATION_OCTET_STREAM: Encoding = Encoding::APPLICATION_OCTET_STREAM;
/// Predefined `APPLICATION_OCTET_STREAM` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_octet_stream() -> &'static Encoding {
    &ENCODING_APPLICATION_OCTET_STREAM
}

pub static ENCODING_TEXT_PLAIN: Encoding = Encoding::TEXT_PLAIN;
/// Predefined `TEXT_PLAIN` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_plain() -> &'static Encoding {
    &ENCODING_TEXT_PLAIN
}

pub static ENCODING_APPLICATION_JSON: Encoding = Encoding::APPLICATION_JSON;
/// Predefined `APPLICATION_JSON` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_json() -> &'static Encoding {
    &ENCODING_APPLICATION_JSON
}

pub static ENCODING_TEXT_JSON: Encoding = Encoding::TEXT_JSON;
/// Predefined `TEXT_JSON` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_json() -> &'static Encoding {
    &ENCODING_TEXT_JSON
}

pub static ENCODING_APPLICATION_CDR: Encoding = Encoding::APPLICATION_CDR;
/// Predefined `APPLICATION_CDR` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_cdr() -> &'static Encoding {
    &ENCODING_APPLICATION_CDR
}

pub static ENCODING_APPLICATION_CBOR: Encoding = Encoding::APPLICATION_CBOR;
/// Predefined `APPLICATION_CBOR` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_cbor() -> &'static Encoding {
    &ENCODING_APPLICATION_CBOR
}

pub static ENCODING_APPLICATION_YAML: Encoding = Encoding::APPLICATION_YAML;
/// Predefined `APPLICATION_YAML` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_yaml() -> &'static Encoding {
    &ENCODING_APPLICATION_YAML
}

pub static ENCODING_TEXT_YAML: Encoding = Encoding::TEXT_YAML;
/// Predefined `TEXT_YAML` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_yaml() -> &'static Encoding {
    &ENCODING_TEXT_YAML
}

pub static ENCODING_TEXT_JSON5: Encoding = Encoding::TEXT_JSON5;
/// Predefined `TEXT_JSON5` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_json5() -> &'static Encoding {
    &ENCODING_TEXT_JSON5
}

pub static ENCODING_APPLICATION_PYTHON_SERIALIZED_OBJECT: Encoding =
    Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT;
/// Predefined `APPLICATION_PYTHON_SERIALIZED_OBJECT` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_python_serialized_object() -> &'static Encoding {
    &ENCODING_APPLICATION_PYTHON_SERIALIZED_OBJECT
}

pub static ENCODING_APPLICATION_PROTOBUF: Encoding = Encoding::APPLICATION_PROTOBUF;
/// Predefined `APPLICATION_PROTOBUF` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_protobuf() -> &'static Encoding {
    &ENCODING_APPLICATION_PROTOBUF
}

pub static ENCODING_APPLICATION_JAVA_SERIALIZED_OBJECT: Encoding =
    Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT;
/// Predefined `APPLICATION_JAVA_SERIALIZED_OBJECT` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_java_serialized_object() -> &'static Encoding {
    &ENCODING_APPLICATION_JAVA_SERIALIZED_OBJECT
}

pub static ENCODING_APPLICATION_OPENMETRICS_TEXT: Encoding =
    Encoding::APPLICATION_OPENMETRICS_TEXT;
/// Predefined `APPLICATION_OPENMETRICS_TEXT` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_openmetrics_text() -> &'static Encoding {
    &ENCODING_APPLICATION_OPENMETRICS_TEXT
}

pub static ENCODING_IMAGE_PNG: Encoding = Encoding::IMAGE_PNG;
/// Predefined `IMAGE_PNG` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_image_png() -> &'static Encoding {
    &ENCODING_IMAGE_PNG
}

pub static ENCODING_IMAGE_JPEG: Encoding = Encoding::IMAGE_JPEG;
/// Predefined `IMAGE_JPEG` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_image_jpeg() -> &'static Encoding {
    &ENCODING_IMAGE_JPEG
}

pub static ENCODING_IMAGE_GIF: Encoding = Encoding::IMAGE_GIF;
/// Predefined `IMAGE_GIF` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_image_gif() -> &'static Encoding {
    &ENCODING_IMAGE_GIF
}

pub static ENCODING_IMAGE_BMP: Encoding = Encoding::IMAGE_BMP;
/// Predefined `IMAGE_BMP` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_image_bmp() -> &'static Encoding {
    &ENCODING_IMAGE_BMP
}

pub static ENCODING_IMAGE_WEBP: Encoding = Encoding::IMAGE_WEBP;
/// Predefined `IMAGE_WEBP` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_image_webp() -> &'static Encoding {
    &ENCODING_IMAGE_WEBP
}

pub static ENCODING_APPLICATION_XML: Encoding = Encoding::APPLICATION_XML;
/// Predefined `APPLICATION_XML` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_xml() -> &'static Encoding {
    &ENCODING_APPLICATION_XML
}

pub static ENCODING_APPLICATION_X_WWW_FORM_URLENCODED: Encoding =
    Encoding::APPLICATION_X_WWW_FORM_URLENCODED;
/// Predefined `APPLICATION_X_WWW_FORM_URLENCODED` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_x_www_form_urlencoded() -> &'static Encoding {
    &ENCODING_APPLICATION_X_WWW_FORM_URLENCODED
}

pub static ENCODING_TEXT_HTML: Encoding = Encoding::TEXT_HTML;
/// Predefined `TEXT_HTML` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_html() -> &'static Encoding {
    &ENCODING_TEXT_HTML
}

pub static ENCODING_TEXT_XML: Encoding = Encoding::TEXT_XML;
/// Predefined `TEXT_XML` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_xml() -> &'static Encoding {
    &ENCODING_TEXT_XML
}

pub static ENCODING_TEXT_CSS: Encoding = Encoding::TEXT_CSS;
/// Predefined `TEXT_CSS` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_css() -> &'static Encoding {
    &ENCODING_TEXT_CSS
}

pub static ENCODING_TEXT_JAVASCRIPT: Encoding = Encoding::TEXT_JAVASCRIPT;
/// Predefined `TEXT_JAVASCRIPT` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_javascript() -> &'static Encoding {
    &ENCODING_TEXT_JAVASCRIPT
}

pub static ENCODING_TEXT_MARKDOWN: Encoding = Encoding::TEXT_MARKDOWN;
/// Predefined `TEXT_MARKDOWN` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_markdown() -> &'static Encoding {
    &ENCODING_TEXT_MARKDOWN
}

pub static ENCODING_TEXT_CSV: Encoding = Encoding::TEXT_CSV;
/// Predefined `TEXT_CSV` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_text_csv() -> &'static Encoding {
    &ENCODING_TEXT_CSV
}

pub static ENCODING_APPLICATION_SQL: Encoding = Encoding::APPLICATION_SQL;
/// Predefined `APPLICATION_SQL` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_sql() -> &'static Encoding {
    &ENCODING_APPLICATION_SQL
}

pub static ENCODING_APPLICATION_COAP_PAYLOAD: Encoding = Encoding::APPLICATION_COAP_PAYLOAD;
/// Predefined `APPLICATION_COAP_PAYLOAD` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_coap_payload() -> &'static Encoding {
    &ENCODING_APPLICATION_COAP_PAYLOAD
}

pub static ENCODING_APPLICATION_JSON_PATCH_JSON: Encoding =
    Encoding::APPLICATION_JSON_PATCH_JSON;
/// Predefined `APPLICATION_JSON_PATCH_JSON` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_json_patch_json() -> &'static Encoding {
    &ENCODING_APPLICATION_JSON_PATCH_JSON
}

pub static ENCODING_APPLICATION_JSON_SEQ: Encoding = Encoding::APPLICATION_JSON_SEQ;
/// Predefined `APPLICATION_JSON_SEQ` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_json_seq() -> &'static Encoding {
    &ENCODING_APPLICATION_JSON_SEQ
}

pub static ENCODING_APPLICATION_JSONPATH: Encoding = Encoding::APPLICATION_JSONPATH;
/// Predefined `APPLICATION_JSONPATH` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_jsonpath() -> &'static Encoding {
    &ENCODING_APPLICATION_JSONPATH
}

pub static ENCODING_APPLICATION_JWT: Encoding = Encoding::APPLICATION_JWT;
/// Predefined `APPLICATION_JWT` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_jwt() -> &'static Encoding {
    &ENCODING_APPLICATION_JWT
}

pub static ENCODING_APPLICATION_MP4: Encoding = Encoding::APPLICATION_MP4;
/// Predefined `APPLICATION_MP4` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_mp4() -> &'static Encoding {
    &ENCODING_APPLICATION_MP4
}

pub static ENCODING_APPLICATION_SOAP_XML: Encoding = Encoding::APPLICATION_SOAP_XML;
/// Predefined `APPLICATION_SOAP_XML` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_soap_xml() -> &'static Encoding {
    &ENCODING_APPLICATION_SOAP_XML
}

pub static ENCODING_APPLICATION_YANG: Encoding = Encoding::APPLICATION_YANG;
/// Predefined `APPLICATION_YANG` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_application_yang() -> &'static Encoding {
    &ENCODING_APPLICATION_YANG
}

pub static ENCODING_AUDIO_AAC: Encoding = Encoding::AUDIO_AAC;
/// Predefined `AUDIO_AAC` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_audio_aac() -> &'static Encoding {
    &ENCODING_AUDIO_AAC
}

pub static ENCODING_AUDIO_FLAC: Encoding = Encoding::AUDIO_FLAC;
/// Predefined `AUDIO_FLAC` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_audio_flac() -> &'static Encoding {
    &ENCODING_AUDIO_FLAC
}

pub static ENCODING_AUDIO_MP4: Encoding = Encoding::AUDIO_MP4;
/// Predefined `AUDIO_MP4` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_audio_mp4() -> &'static Encoding {
    &ENCODING_AUDIO_MP4
}

pub static ENCODING_AUDIO_OGG: Encoding = Encoding::AUDIO_OGG;
/// Predefined `AUDIO_OGG` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_audio_ogg() -> &'static Encoding {
    &ENCODING_AUDIO_OGG
}

pub static ENCODING_AUDIO_VORBIS: Encoding = Encoding::AUDIO_VORBIS;
/// Predefined `AUDIO_VORBIS` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_audio_vorbis() -> &'static Encoding {
    &ENCODING_AUDIO_VORBIS
}

pub static ENCODING_VIDEO_H261: Encoding = Encoding::VIDEO_H261;
/// Predefined `VIDEO_H261` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_h261() -> &'static Encoding {
    &ENCODING_VIDEO_H261
}

pub static ENCODING_VIDEO_H263: Encoding = Encoding::VIDEO_H263;
/// Predefined `VIDEO_H263` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_h263() -> &'static Encoding {
    &ENCODING_VIDEO_H263
}

pub static ENCODING_VIDEO_H264: Encoding = Encoding::VIDEO_H264;
/// Predefined `VIDEO_H264` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_h264() -> &'static Encoding {
    &ENCODING_VIDEO_H264
}

pub static ENCODING_VIDEO_H265: Encoding = Encoding::VIDEO_H265;
/// Predefined `VIDEO_H265` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_h265() -> &'static Encoding {
    &ENCODING_VIDEO_H265
}

pub static ENCODING_VIDEO_H266: Encoding = Encoding::VIDEO_H266;
/// Predefined `VIDEO_H266` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_h266() -> &'static Encoding {
    &ENCODING_VIDEO_H266
}

pub static ENCODING_VIDEO_MP4: Encoding = Encoding::VIDEO_MP4;
/// Predefined `VIDEO_MP4` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_mp4() -> &'static Encoding {
    &ENCODING_VIDEO_MP4
}

pub static ENCODING_VIDEO_OGG: Encoding = Encoding::VIDEO_OGG;
/// Predefined `VIDEO_OGG` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_ogg() -> &'static Encoding {
    &ENCODING_VIDEO_OGG
}

pub static ENCODING_VIDEO_RAW: Encoding = Encoding::VIDEO_RAW;
/// Predefined `VIDEO_RAW` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_raw() -> &'static Encoding {
    &ENCODING_VIDEO_RAW
}

pub static ENCODING_VIDEO_VP8: Encoding = Encoding::VIDEO_VP8;
/// Predefined `VIDEO_VP8` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_vp8() -> &'static Encoding {
    &ENCODING_VIDEO_VP8
}

pub static ENCODING_VIDEO_VP9: Encoding = Encoding::VIDEO_VP9;
/// Predefined `VIDEO_VP9` encoding (borrowed static; shared, never freed).
#[prebindgen]
pub fn encoding_video_vp9() -> &'static Encoding {
    &ENCODING_VIDEO_VP9
}
