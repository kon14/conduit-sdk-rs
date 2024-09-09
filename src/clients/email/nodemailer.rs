use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::collections::HashMap;

// #[derive(Debug)]
// struct AttachmentLike {
//     /// String, Buffer or a Stream contents for the attachment
//     content: Option<Content>,
//     /// Path to a file or a URL (data uris are allowed as well) if you want to stream the file instead of including it (better for larger attachments).
//     path: Option<String>,
// }

#[derive(Debug)]
pub enum Content {
    StringContent(String),
    Buffer(Vec<u8>),
    // Readable(Box<dyn Read>),
}

#[derive(Debug)]
pub struct Attachment {
    /// String, Buffer or a Stream contents for the attachment.
    pub content: Option<Content>,
    /// Path to a file or a URL (data uris are allowed as well) if you want to stream the file instead of including it (better for larger attachments).
    pub path: Option<String>,
    /// Filename to be reported as the name of the attached file.
    pub filename: Option<Filename>,
    /// Content id for using inline images in HTML message source.
    /// Using cid sets the default contentDisposition to 'inline' and moves the attachment into a multipart/related mime node, so use it only if you actually want to use this attachment as an embedded image.
    pub cid: Option<String>,
    /// If set and content is string, encodes the content to a Buffer using the specified encoding.
    /// Example values: base64, hex, binary etc. Useful if you want to use binary attachments in a JSON formatted e-mail object .
    pub encoding: Option<String>,
    /// Content type for the attachment.
    /// If not set will be derived from the filename property.
    pub content_type: Option<String>,
    /// Transfer encoding for the attachment.
    /// If not set it will be derived from the contentType property. Example values: quoted-printable, base64. If it is unset then base64 encoding is used for the attachment. If it is set to false then previous default applies (base64 for most, 7bit for text).
    pub content_transfer_encoding: Option<ContentTransferEncoding>,
    /// Content disposition type for the attachment.
    /// Defaults to 'attachment'.
    pub content_disposition: Option<ContentDisposition>,
    /// An object of additional headers.
    pub headers: Option<HashMap<String, String>>,
    /// Overrides entire node content in the mime message. If used then all other options set for this node are ignored.
    pub raw: Option<Raw>,
}

#[derive(Debug)]
pub enum Filename {
    StringFilename(String),
    NoFilename,
}

#[derive(Debug)]
pub enum ContentTransferEncoding {
    SevenBit,
    Base64,
    QuotedPrintable,
    FromContentType,
}

#[derive(Debug)]
pub enum ContentDisposition {
    Attachment,
    Inline,
}

#[derive(Debug)]
pub enum Raw {
    StringRaw(String),
    Buffer(Vec<u8>),
    // Readable(Box<dyn Read>),
    // AttachmentLike(Box<AttachmentLike>),
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Content::StringContent(s) => serializer.serialize_str(s),
            Content::Buffer(b) => serializer.serialize_str(&STANDARD.encode(b)),
            // Content::Readable(_) => serializer.serialize_none(),
        }
    }
}

impl Serialize for Filename {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Filename::StringFilename(s) => serializer.serialize_str(s),
            Filename::NoFilename => serializer.serialize_bool(false),
        }
    }
}

impl Serialize for ContentTransferEncoding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ContentTransferEncoding::SevenBit => serializer.serialize_str("7bit"),
            ContentTransferEncoding::Base64 => serializer.serialize_str("base64"),
            ContentTransferEncoding::QuotedPrintable => {
                serializer.serialize_str("quoted-printable")
            }
            ContentTransferEncoding::FromContentType => serializer.serialize_bool(false),
        }
    }
}

impl Serialize for ContentDisposition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ContentDisposition::Attachment => serializer.serialize_str("attachment"),
            ContentDisposition::Inline => serializer.serialize_str("inline"),
        }
    }
}

impl Serialize for Raw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Raw::StringRaw(s) => serializer.serialize_str(s),
            Raw::Buffer(b) => serializer.serialize_str(&STANDARD.encode(b)),
            // Raw::Readable(_) => serializer.serialize_none(),
            // Raw::AttachmentLike(_) => serializer.serialize_none(),
        }
    }
}

impl Serialize for Attachment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        if let Some(content) = &self.content {
            state.serialize_entry("content", content)?;
        }
        if let Some(path) = &self.path {
            state.serialize_entry("path", path)?;
        }
        if let Some(filename) = &self.filename {
            state.serialize_entry("filename", filename)?;
        }
        if let Some(cid) = &self.cid {
            state.serialize_entry("cid", cid)?;
        }
        if let Some(encoding) = &self.encoding {
            state.serialize_entry("encoding", encoding)?;
        }
        if let Some(content_type) = &self.content_type {
            state.serialize_entry("contentType", content_type)?;
        }
        if let Some(content_transfer_encoding) = &self.content_transfer_encoding {
            state.serialize_entry("contentTransferEncoding", content_transfer_encoding)?;
        }
        if let Some(content_disposition) = &self.content_disposition {
            state.serialize_entry("contentDisposition", content_disposition)?;
        }
        if let Some(headers) = &self.headers {
            state.serialize_entry("headers", headers)?;
        }
        if let Some(raw) = &self.raw {
            state.serialize_entry("raw", raw)?;
        }
        state.end()
    }
}
