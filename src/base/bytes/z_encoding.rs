use crate::ZEncoding;
use prebindgen_proc_macro::prebindgen;

/// Numeric id of the encoding (u16 widened to i32 for JVM).
#[prebindgen]
pub fn z_encoding_id(e: &ZEncoding) -> i32 {
    e.id() as i32
}

/// Clone an encoding into an owned handle (cheap; materializes an owned
/// encoding from a borrowed/predefined one).
#[prebindgen]
pub fn z_encoding_clone(e: &ZEncoding) -> ZEncoding {
    e.clone()
}

/// Optional textual schema attached to the encoding.
#[prebindgen]
pub fn z_encoding_schema(e: &ZEncoding) -> Option<String> {
    e.schema()
        .and_then(|s| std::str::from_utf8(s).ok().map(str::to_string))
}

/// Canonical display string for a [`ZEncoding`] (upstream `Display` impl).
#[prebindgen]
pub fn z_encoding_to_string(e: &ZEncoding) -> String {
    e.to_string()
}

/// Parse a textual encoding into a [`ZEncoding`] (upstream `From<String>`:
/// known names resolve to their canonical id; everything else is preserved
/// under the custom-encoding id).
#[prebindgen]
pub fn z_encoding_from_string(s: String) -> ZEncoding {
    ZEncoding::from(s)
}

/// Build an encoding from its numeric id and optional schema.
#[prebindgen]
pub fn z_encoding_from_id(id: i32, schema: Option<String>) -> ZEncoding {
    ZEncoding::new(id as u16, schema.map(|s| s.into_bytes().into()))
}

/// Return a copy of `encoding` with `schema` attached.
#[prebindgen]
pub fn z_encoding_with_schema(encoding: &ZEncoding, schema: String) -> ZEncoding {
    encoding.clone().with_schema(schema)
}

// ── Predefined-constant accessors ─────────────────────────────────────────
// Each predefined encoding is stored as a `static` backed by the upstream
// `const` value so that callers receive a `&'static ZEncoding` — a permanent,
// shared pointer that must NOT be freed.

pub static Z_ENCODING_ZENOH_BYTES: ZEncoding = ZEncoding::ZENOH_BYTES;
#[prebindgen]
pub fn z_encoding_zenoh_bytes() -> &'static ZEncoding {
    &Z_ENCODING_ZENOH_BYTES
}

pub static Z_ENCODING_ZENOH_STRING: ZEncoding = ZEncoding::ZENOH_STRING;
#[prebindgen]
pub fn z_encoding_zenoh_string() -> &'static ZEncoding {
    &Z_ENCODING_ZENOH_STRING
}

pub static Z_ENCODING_ZENOH_SERIALIZED: ZEncoding = ZEncoding::ZENOH_SERIALIZED;
#[prebindgen]
pub fn z_encoding_zenoh_serialized() -> &'static ZEncoding {
    &Z_ENCODING_ZENOH_SERIALIZED
}

pub static Z_ENCODING_APPLICATION_OCTET_STREAM: ZEncoding = ZEncoding::APPLICATION_OCTET_STREAM;
#[prebindgen]
pub fn z_encoding_application_octet_stream() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_OCTET_STREAM
}

pub static Z_ENCODING_TEXT_PLAIN: ZEncoding = ZEncoding::TEXT_PLAIN;
#[prebindgen]
pub fn z_encoding_text_plain() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_PLAIN
}

pub static Z_ENCODING_APPLICATION_JSON: ZEncoding = ZEncoding::APPLICATION_JSON;
#[prebindgen]
pub fn z_encoding_application_json() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_JSON
}

pub static Z_ENCODING_TEXT_JSON: ZEncoding = ZEncoding::TEXT_JSON;
#[prebindgen]
pub fn z_encoding_text_json() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_JSON
}

pub static Z_ENCODING_APPLICATION_CDR: ZEncoding = ZEncoding::APPLICATION_CDR;
#[prebindgen]
pub fn z_encoding_application_cdr() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_CDR
}

pub static Z_ENCODING_APPLICATION_CBOR: ZEncoding = ZEncoding::APPLICATION_CBOR;
#[prebindgen]
pub fn z_encoding_application_cbor() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_CBOR
}

pub static Z_ENCODING_APPLICATION_YAML: ZEncoding = ZEncoding::APPLICATION_YAML;
#[prebindgen]
pub fn z_encoding_application_yaml() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_YAML
}

pub static Z_ENCODING_TEXT_YAML: ZEncoding = ZEncoding::TEXT_YAML;
#[prebindgen]
pub fn z_encoding_text_yaml() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_YAML
}

pub static Z_ENCODING_TEXT_JSON5: ZEncoding = ZEncoding::TEXT_JSON5;
#[prebindgen]
pub fn z_encoding_text_json5() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_JSON5
}

pub static Z_ENCODING_APPLICATION_PYTHON_SERIALIZED_OBJECT: ZEncoding =
    ZEncoding::APPLICATION_PYTHON_SERIALIZED_OBJECT;
#[prebindgen]
pub fn z_encoding_application_python_serialized_object() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_PYTHON_SERIALIZED_OBJECT
}

pub static Z_ENCODING_APPLICATION_PROTOBUF: ZEncoding = ZEncoding::APPLICATION_PROTOBUF;
#[prebindgen]
pub fn z_encoding_application_protobuf() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_PROTOBUF
}

pub static Z_ENCODING_APPLICATION_JAVA_SERIALIZED_OBJECT: ZEncoding =
    ZEncoding::APPLICATION_JAVA_SERIALIZED_OBJECT;
#[prebindgen]
pub fn z_encoding_application_java_serialized_object() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_JAVA_SERIALIZED_OBJECT
}

pub static Z_ENCODING_APPLICATION_OPENMETRICS_TEXT: ZEncoding =
    ZEncoding::APPLICATION_OPENMETRICS_TEXT;
#[prebindgen]
pub fn z_encoding_application_openmetrics_text() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_OPENMETRICS_TEXT
}

pub static Z_ENCODING_IMAGE_PNG: ZEncoding = ZEncoding::IMAGE_PNG;
#[prebindgen]
pub fn z_encoding_image_png() -> &'static ZEncoding {
    &Z_ENCODING_IMAGE_PNG
}

pub static Z_ENCODING_IMAGE_JPEG: ZEncoding = ZEncoding::IMAGE_JPEG;
#[prebindgen]
pub fn z_encoding_image_jpeg() -> &'static ZEncoding {
    &Z_ENCODING_IMAGE_JPEG
}

pub static Z_ENCODING_IMAGE_GIF: ZEncoding = ZEncoding::IMAGE_GIF;
#[prebindgen]
pub fn z_encoding_image_gif() -> &'static ZEncoding {
    &Z_ENCODING_IMAGE_GIF
}

pub static Z_ENCODING_IMAGE_BMP: ZEncoding = ZEncoding::IMAGE_BMP;
#[prebindgen]
pub fn z_encoding_image_bmp() -> &'static ZEncoding {
    &Z_ENCODING_IMAGE_BMP
}

pub static Z_ENCODING_IMAGE_WEBP: ZEncoding = ZEncoding::IMAGE_WEBP;
#[prebindgen]
pub fn z_encoding_image_webp() -> &'static ZEncoding {
    &Z_ENCODING_IMAGE_WEBP
}

pub static Z_ENCODING_APPLICATION_XML: ZEncoding = ZEncoding::APPLICATION_XML;
#[prebindgen]
pub fn z_encoding_application_xml() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_XML
}

pub static Z_ENCODING_APPLICATION_X_WWW_FORM_URLENCODED: ZEncoding =
    ZEncoding::APPLICATION_X_WWW_FORM_URLENCODED;
#[prebindgen]
pub fn z_encoding_application_x_www_form_urlencoded() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_X_WWW_FORM_URLENCODED
}

pub static Z_ENCODING_TEXT_HTML: ZEncoding = ZEncoding::TEXT_HTML;
#[prebindgen]
pub fn z_encoding_text_html() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_HTML
}

pub static Z_ENCODING_TEXT_XML: ZEncoding = ZEncoding::TEXT_XML;
#[prebindgen]
pub fn z_encoding_text_xml() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_XML
}

pub static Z_ENCODING_TEXT_CSS: ZEncoding = ZEncoding::TEXT_CSS;
#[prebindgen]
pub fn z_encoding_text_css() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_CSS
}

pub static Z_ENCODING_TEXT_JAVASCRIPT: ZEncoding = ZEncoding::TEXT_JAVASCRIPT;
#[prebindgen]
pub fn z_encoding_text_javascript() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_JAVASCRIPT
}

pub static Z_ENCODING_TEXT_MARKDOWN: ZEncoding = ZEncoding::TEXT_MARKDOWN;
#[prebindgen]
pub fn z_encoding_text_markdown() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_MARKDOWN
}

pub static Z_ENCODING_TEXT_CSV: ZEncoding = ZEncoding::TEXT_CSV;
#[prebindgen]
pub fn z_encoding_text_csv() -> &'static ZEncoding {
    &Z_ENCODING_TEXT_CSV
}

pub static Z_ENCODING_APPLICATION_SQL: ZEncoding = ZEncoding::APPLICATION_SQL;
#[prebindgen]
pub fn z_encoding_application_sql() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_SQL
}

pub static Z_ENCODING_APPLICATION_COAP_PAYLOAD: ZEncoding = ZEncoding::APPLICATION_COAP_PAYLOAD;
#[prebindgen]
pub fn z_encoding_application_coap_payload() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_COAP_PAYLOAD
}

pub static Z_ENCODING_APPLICATION_JSON_PATCH_JSON: ZEncoding =
    ZEncoding::APPLICATION_JSON_PATCH_JSON;
#[prebindgen]
pub fn z_encoding_application_json_patch_json() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_JSON_PATCH_JSON
}

pub static Z_ENCODING_APPLICATION_JSON_SEQ: ZEncoding = ZEncoding::APPLICATION_JSON_SEQ;
#[prebindgen]
pub fn z_encoding_application_json_seq() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_JSON_SEQ
}

pub static Z_ENCODING_APPLICATION_JSONPATH: ZEncoding = ZEncoding::APPLICATION_JSONPATH;
#[prebindgen]
pub fn z_encoding_application_jsonpath() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_JSONPATH
}

pub static Z_ENCODING_APPLICATION_JWT: ZEncoding = ZEncoding::APPLICATION_JWT;
#[prebindgen]
pub fn z_encoding_application_jwt() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_JWT
}

pub static Z_ENCODING_APPLICATION_MP4: ZEncoding = ZEncoding::APPLICATION_MP4;
#[prebindgen]
pub fn z_encoding_application_mp4() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_MP4
}

pub static Z_ENCODING_APPLICATION_SOAP_XML: ZEncoding = ZEncoding::APPLICATION_SOAP_XML;
#[prebindgen]
pub fn z_encoding_application_soap_xml() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_SOAP_XML
}

pub static Z_ENCODING_APPLICATION_YANG: ZEncoding = ZEncoding::APPLICATION_YANG;
#[prebindgen]
pub fn z_encoding_application_yang() -> &'static ZEncoding {
    &Z_ENCODING_APPLICATION_YANG
}

pub static Z_ENCODING_AUDIO_AAC: ZEncoding = ZEncoding::AUDIO_AAC;
#[prebindgen]
pub fn z_encoding_audio_aac() -> &'static ZEncoding {
    &Z_ENCODING_AUDIO_AAC
}

pub static Z_ENCODING_AUDIO_FLAC: ZEncoding = ZEncoding::AUDIO_FLAC;
#[prebindgen]
pub fn z_encoding_audio_flac() -> &'static ZEncoding {
    &Z_ENCODING_AUDIO_FLAC
}

pub static Z_ENCODING_AUDIO_MP4: ZEncoding = ZEncoding::AUDIO_MP4;
#[prebindgen]
pub fn z_encoding_audio_mp4() -> &'static ZEncoding {
    &Z_ENCODING_AUDIO_MP4
}

pub static Z_ENCODING_AUDIO_OGG: ZEncoding = ZEncoding::AUDIO_OGG;
#[prebindgen]
pub fn z_encoding_audio_ogg() -> &'static ZEncoding {
    &Z_ENCODING_AUDIO_OGG
}

pub static Z_ENCODING_AUDIO_VORBIS: ZEncoding = ZEncoding::AUDIO_VORBIS;
#[prebindgen]
pub fn z_encoding_audio_vorbis() -> &'static ZEncoding {
    &Z_ENCODING_AUDIO_VORBIS
}

pub static Z_ENCODING_VIDEO_H261: ZEncoding = ZEncoding::VIDEO_H261;
#[prebindgen]
pub fn z_encoding_video_h261() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_H261
}

pub static Z_ENCODING_VIDEO_H263: ZEncoding = ZEncoding::VIDEO_H263;
#[prebindgen]
pub fn z_encoding_video_h263() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_H263
}

pub static Z_ENCODING_VIDEO_H264: ZEncoding = ZEncoding::VIDEO_H264;
#[prebindgen]
pub fn z_encoding_video_h264() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_H264
}

pub static Z_ENCODING_VIDEO_H265: ZEncoding = ZEncoding::VIDEO_H265;
#[prebindgen]
pub fn z_encoding_video_h265() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_H265
}

pub static Z_ENCODING_VIDEO_H266: ZEncoding = ZEncoding::VIDEO_H266;
#[prebindgen]
pub fn z_encoding_video_h266() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_H266
}

pub static Z_ENCODING_VIDEO_MP4: ZEncoding = ZEncoding::VIDEO_MP4;
#[prebindgen]
pub fn z_encoding_video_mp4() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_MP4
}

pub static Z_ENCODING_VIDEO_OGG: ZEncoding = ZEncoding::VIDEO_OGG;
#[prebindgen]
pub fn z_encoding_video_ogg() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_OGG
}

pub static Z_ENCODING_VIDEO_RAW: ZEncoding = ZEncoding::VIDEO_RAW;
#[prebindgen]
pub fn z_encoding_video_raw() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_RAW
}

pub static Z_ENCODING_VIDEO_VP8: ZEncoding = ZEncoding::VIDEO_VP8;
#[prebindgen]
pub fn z_encoding_video_vp8() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_VP8
}

pub static Z_ENCODING_VIDEO_VP9: ZEncoding = ZEncoding::VIDEO_VP9;
#[prebindgen]
pub fn z_encoding_video_vp9() -> &'static ZEncoding {
    &Z_ENCODING_VIDEO_VP9
}
