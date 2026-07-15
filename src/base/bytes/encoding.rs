use prebindgen_proc_macro::prebindgen;

use crate::Encoding;

/// Return the numeric identifier of an encoding.
#[prebindgen]
pub fn encoding_get_id(e: &Encoding) -> i32 {
    e.id() as i32
}

/// Create an independent copy of an encoding.
#[prebindgen]
pub fn encoding_new_clone(e: &Encoding) -> Encoding {
    e.clone()
}

/// Return the schema associated with an encoding, when present.
#[prebindgen]
pub fn encoding_get_schema(e: &Encoding) -> Option<String> {
    e.schema()
        .and_then(|s| std::str::from_utf8(s).ok().map(str::to_string))
}

/// Return the standard textual representation of an encoding.
#[prebindgen]
pub fn encoding_to_string(e: &Encoding) -> String {
    e.to_string()
}

/// Create an encoding from its textual representation.
///
/// Known names resolve to their standard identifiers. Other names are
/// preserved as custom encodings.
#[prebindgen]
pub fn encoding_new_from_string(s: String) -> Encoding {
    Encoding::from(s)
}

/// Create an encoding from its numeric identifier and optional schema.
#[prebindgen]
pub fn encoding_new_from_id(id: i32, schema: Option<String>) -> Encoding {
    Encoding::new(id as u16, schema.map(|s| s.into_bytes().into()))
}

/// Create an encoding with the supplied schema.
///
/// Schema interpretation is application-defined; for example, `utf-8` may
/// accompany `text/plain`.
#[prebindgen]
pub fn encoding_new_with_schema(e: &Encoding, schema: String) -> Encoding {
    e.clone().with_schema(schema)
}

// ── Predefined-constant accessors ─────────────────────────────────────────
// Each predefined encoding is exposed exactly once: `encoding_const_<name>()`
// returns a `&'static Encoding` backed by a function-local `static` — a
// permanent, shared pointer that must NOT be freed (the loaning idiom C
// bindings want). A binding needing the preset's decomposed values composes
// the general accessors itself (`encoding_get_id`, `encoding_to_string` over
// the factory) — e.g. `lang::JniGen`'s expression constants — so no
// per-preset `_id`/`_str` accessors exist here.

/// Predefined `ZENOH_BYTES` encoding.
#[prebindgen]
pub fn encoding_const_zenoh_bytes() -> &'static Encoding {
    static E: Encoding = Encoding::ZENOH_BYTES;
    &E
}

/// Predefined `ZENOH_STRING` encoding.
#[prebindgen]
pub fn encoding_const_zenoh_string() -> &'static Encoding {
    static E: Encoding = Encoding::ZENOH_STRING;
    &E
}

/// Predefined `ZENOH_SERIALIZED` encoding.
#[prebindgen]
pub fn encoding_const_zenoh_serialized() -> &'static Encoding {
    static E: Encoding = Encoding::ZENOH_SERIALIZED;
    &E
}

/// Predefined `APPLICATION_OCTET_STREAM` encoding.
#[prebindgen]
pub fn encoding_const_application_octet_stream() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_OCTET_STREAM;
    &E
}

/// Predefined `TEXT_PLAIN` encoding.
#[prebindgen]
pub fn encoding_const_text_plain() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_PLAIN;
    &E
}

/// Predefined `APPLICATION_JSON` encoding.
#[prebindgen]
pub fn encoding_const_application_json() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_JSON;
    &E
}

/// Predefined `TEXT_JSON` encoding.
#[prebindgen]
pub fn encoding_const_text_json() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_JSON;
    &E
}

/// Predefined `APPLICATION_CDR` encoding.
#[prebindgen]
pub fn encoding_const_application_cdr() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_CDR;
    &E
}

/// Predefined `APPLICATION_CBOR` encoding.
#[prebindgen]
pub fn encoding_const_application_cbor() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_CBOR;
    &E
}

/// Predefined `APPLICATION_YAML` encoding.
#[prebindgen]
pub fn encoding_const_application_yaml() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_YAML;
    &E
}

/// Predefined `TEXT_YAML` encoding.
#[prebindgen]
pub fn encoding_const_text_yaml() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_YAML;
    &E
}

/// Predefined `TEXT_JSON5` encoding.
#[prebindgen]
pub fn encoding_const_text_json5() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_JSON5;
    &E
}

/// Predefined `APPLICATION_PYTHON_SERIALIZED_OBJECT` encoding.
#[prebindgen]
pub fn encoding_const_application_python_serialized_object() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT;
    &E
}

/// Predefined `APPLICATION_PROTOBUF` encoding.
#[prebindgen]
pub fn encoding_const_application_protobuf() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_PROTOBUF;
    &E
}

/// Predefined `APPLICATION_JAVA_SERIALIZED_OBJECT` encoding.
#[prebindgen]
pub fn encoding_const_application_java_serialized_object() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT;
    &E
}

/// Predefined `APPLICATION_OPENMETRICS_TEXT` encoding.
#[prebindgen]
pub fn encoding_const_application_openmetrics_text() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_OPENMETRICS_TEXT;
    &E
}

/// Predefined `IMAGE_PNG` encoding.
#[prebindgen]
pub fn encoding_const_image_png() -> &'static Encoding {
    static E: Encoding = Encoding::IMAGE_PNG;
    &E
}

/// Predefined `IMAGE_JPEG` encoding.
#[prebindgen]
pub fn encoding_const_image_jpeg() -> &'static Encoding {
    static E: Encoding = Encoding::IMAGE_JPEG;
    &E
}

/// Predefined `IMAGE_GIF` encoding.
#[prebindgen]
pub fn encoding_const_image_gif() -> &'static Encoding {
    static E: Encoding = Encoding::IMAGE_GIF;
    &E
}

/// Predefined `IMAGE_BMP` encoding.
#[prebindgen]
pub fn encoding_const_image_bmp() -> &'static Encoding {
    static E: Encoding = Encoding::IMAGE_BMP;
    &E
}

/// Predefined `IMAGE_WEBP` encoding.
#[prebindgen]
pub fn encoding_const_image_webp() -> &'static Encoding {
    static E: Encoding = Encoding::IMAGE_WEBP;
    &E
}

/// Predefined `APPLICATION_XML` encoding.
#[prebindgen]
pub fn encoding_const_application_xml() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_XML;
    &E
}

/// Predefined `APPLICATION_X_WWW_FORM_URLENCODED` encoding.
#[prebindgen]
pub fn encoding_const_application_x_www_form_urlencoded() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_X_WWW_FORM_URLENCODED;
    &E
}

/// Predefined `TEXT_HTML` encoding.
#[prebindgen]
pub fn encoding_const_text_html() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_HTML;
    &E
}

/// Predefined `TEXT_XML` encoding.
#[prebindgen]
pub fn encoding_const_text_xml() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_XML;
    &E
}

/// Predefined `TEXT_CSS` encoding.
#[prebindgen]
pub fn encoding_const_text_css() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_CSS;
    &E
}

/// Predefined `TEXT_JAVASCRIPT` encoding.
#[prebindgen]
pub fn encoding_const_text_javascript() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_JAVASCRIPT;
    &E
}

/// Predefined `TEXT_MARKDOWN` encoding.
#[prebindgen]
pub fn encoding_const_text_markdown() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_MARKDOWN;
    &E
}

/// Predefined `TEXT_CSV` encoding.
#[prebindgen]
pub fn encoding_const_text_csv() -> &'static Encoding {
    static E: Encoding = Encoding::TEXT_CSV;
    &E
}

/// Predefined `APPLICATION_SQL` encoding.
#[prebindgen]
pub fn encoding_const_application_sql() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_SQL;
    &E
}

/// Predefined `APPLICATION_COAP_PAYLOAD` encoding.
#[prebindgen]
pub fn encoding_const_application_coap_payload() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_COAP_PAYLOAD;
    &E
}

/// Predefined `APPLICATION_JSON_PATCH_JSON` encoding.
#[prebindgen]
pub fn encoding_const_application_json_patch_json() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_JSON_PATCH_JSON;
    &E
}

/// Predefined `APPLICATION_JSON_SEQ` encoding.
#[prebindgen]
pub fn encoding_const_application_json_seq() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_JSON_SEQ;
    &E
}

/// Predefined `APPLICATION_JSONPATH` encoding.
#[prebindgen]
pub fn encoding_const_application_jsonpath() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_JSONPATH;
    &E
}

/// Predefined `APPLICATION_JWT` encoding.
#[prebindgen]
pub fn encoding_const_application_jwt() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_JWT;
    &E
}

/// Predefined `APPLICATION_MP4` encoding.
#[prebindgen]
pub fn encoding_const_application_mp4() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_MP4;
    &E
}

/// Predefined `APPLICATION_SOAP_XML` encoding.
#[prebindgen]
pub fn encoding_const_application_soap_xml() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_SOAP_XML;
    &E
}

/// Predefined `APPLICATION_YANG` encoding.
#[prebindgen]
pub fn encoding_const_application_yang() -> &'static Encoding {
    static E: Encoding = Encoding::APPLICATION_YANG;
    &E
}

/// Predefined `AUDIO_AAC` encoding.
#[prebindgen]
pub fn encoding_const_audio_aac() -> &'static Encoding {
    static E: Encoding = Encoding::AUDIO_AAC;
    &E
}

/// Predefined `AUDIO_FLAC` encoding.
#[prebindgen]
pub fn encoding_const_audio_flac() -> &'static Encoding {
    static E: Encoding = Encoding::AUDIO_FLAC;
    &E
}

/// Predefined `AUDIO_MP4` encoding.
#[prebindgen]
pub fn encoding_const_audio_mp4() -> &'static Encoding {
    static E: Encoding = Encoding::AUDIO_MP4;
    &E
}

/// Predefined `AUDIO_OGG` encoding.
#[prebindgen]
pub fn encoding_const_audio_ogg() -> &'static Encoding {
    static E: Encoding = Encoding::AUDIO_OGG;
    &E
}

/// Predefined `AUDIO_VORBIS` encoding.
#[prebindgen]
pub fn encoding_const_audio_vorbis() -> &'static Encoding {
    static E: Encoding = Encoding::AUDIO_VORBIS;
    &E
}

/// Predefined `VIDEO_H261` encoding.
#[prebindgen]
pub fn encoding_const_video_h261() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_H261;
    &E
}

/// Predefined `VIDEO_H263` encoding.
#[prebindgen]
pub fn encoding_const_video_h263() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_H263;
    &E
}

/// Predefined `VIDEO_H264` encoding.
#[prebindgen]
pub fn encoding_const_video_h264() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_H264;
    &E
}

/// Predefined `VIDEO_H265` encoding.
#[prebindgen]
pub fn encoding_const_video_h265() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_H265;
    &E
}

/// Predefined `VIDEO_H266` encoding.
#[prebindgen]
pub fn encoding_const_video_h266() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_H266;
    &E
}

/// Predefined `VIDEO_MP4` encoding.
#[prebindgen]
pub fn encoding_const_video_mp4() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_MP4;
    &E
}

/// Predefined `VIDEO_OGG` encoding.
#[prebindgen]
pub fn encoding_const_video_ogg() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_OGG;
    &E
}

/// Predefined `VIDEO_RAW` encoding.
#[prebindgen]
pub fn encoding_const_video_raw() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_RAW;
    &E
}

/// Predefined `VIDEO_VP8` encoding.
#[prebindgen]
pub fn encoding_const_video_vp8() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_VP8;
    &E
}

/// Predefined `VIDEO_VP9` encoding.
#[prebindgen]
pub fn encoding_const_video_vp9() -> &'static Encoding {
    static E: Encoding = Encoding::VIDEO_VP9;
    &E
}
