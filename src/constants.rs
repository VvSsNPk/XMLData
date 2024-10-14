use std::collections::HashSet;
use quick_xml::name::{LocalName, QName};
use once_cell::sync::Lazy;

pub static SET_CONTAINS:Lazy<HashSet<LocalName>> = Lazy::new(||{
    let test = HashSet::from(
        [
            LocalName::from(QName(b"author_id")),
            LocalName::from(QName(b"author_ids")),
            LocalName::from(QName(b"document_id")),
            LocalName::from(QName(b"classification")),
            LocalName::from(QName(b"classifications")),
            LocalName::from(QName(b"keywords")),
            LocalName::from(QName(b"keyword")),
            LocalName::from(QName(b"publication_year")),
        ]
    );
    test
});