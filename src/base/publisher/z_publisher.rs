use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{ZEncoding, ZError, ZPublisher, ZZBytes};

#[prebindgen]
pub fn z_publisher_put(
    publisher: &ZPublisher,
    payload: ZZBytes,
    encoding: Option<&ZEncoding>,
    attachment: Option<ZZBytes>,
) -> Result<(), ZError> {
    let mut publication = publisher.put(payload);
    if let Some(enc) = encoding {
        publication = publication.encoding(enc.clone());
    }
    if let Some(att) = attachment {
        publication = publication.attachment(att);
    }
    publication.wait()
}

#[prebindgen]
pub fn z_publisher_delete(
    publisher: &ZPublisher,
    attachment: Option<ZZBytes>,
) -> Result<(), ZError> {
    let mut delete = publisher.delete();
    if let Some(att) = attachment {
        delete = delete.attachment(att);
    }
    delete.wait()
}
