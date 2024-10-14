use std::collections::HashSet;
use std::io::Write;
use oxigraph::io::write::TripleWriter;
use oxigraph::model::{LiteralRef, NamedNodeRef, Triple, TripleRef};
use oxigraph::model::Term::Literal;
use oxigraph::model::vocab::xsd::G_YEAR_MONTH;
use serde::{Deserialize, Serialize};
use url::Url;
use url::form_urlencoded::Serializer;

pub const BASE_URL: &str = "https://zbmath.org/";
pub const PREDICATE_KEY_WORD: &str = "https://zbmath.org/isKeyword";
pub const PREDICATE_INTERSECTION: &str = "https://zbmath.org/isIntersection";
pub const PREDICATE_PUBLISHED_YEAR: &str = "https://zbmath.org/inYear";
pub const PREDICATE_PUBLISHED_BY:&str = "https://zbmath.org/isPublishedby";
pub const CONTAINS_KEY_WORD:&str = "https://zbmath.org/containsKeyword";
#[derive(Serialize, Deserialize,Debug)]
pub struct Record {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "document_id")]
    pub zbmath_document_id: String,
    #[serde(rename = "publication_year")]
    pub zbmmath_publication_year:u32,
    #[serde(rename = "author_ids")]
    pub zbmath_author_ids: Option<ZbmathAuthorIds>,
    #[serde(rename = "classifications")]
    pub zbmath_classifications: Option<ZbmathClassifications>,
    #[serde(rename = "keywords")]
    pub zbmath_keywords: Option<ZbmathKeywords>,
}

impl Record {
    pub fn write_keywords<T: Write>(&self,writer : &mut TripleWriter<T>){
        if let Some(keywords) = &self.zbmath_keywords{
            if let Some(authors) = &self.zbmath_author_ids {
                let base = Url::parse(BASE_URL).expect("cannot parse the url");
                let predicate = Url::parse(PREDICATE_KEY_WORD).expect("cannot parse the predicate keyword url");
                for i in &authors.zbmath_author_id{
                    for j in &keywords.zbmath_keyword{
                        let author_str = String::from("ai:") + i;
                        let encoded_author = Serializer::new(String::new())
                            .append_pair("q",&author_str)
                            .finish();
                        let mut node1_str = base.clone();
                        node1_str.set_path("authors/");
                        node1_str.set_query(Some(&encoded_author));
                        let subject = NamedNodeRef::new(node1_str.as_str()).expect("could not get the subject node author");
                        let key_word = String::from("ut:") + j;
                        let encoded_keyword = Serializer::new(String::new())
                            .append_pair("q",&key_word)
                            .finish();
                        let mut node2_str = base.clone();
                        node2_str.set_query(Some(&encoded_keyword));
                        let object = NamedNodeRef::new(node2_str.as_str()).expect("could not get the object node keyword");
                        let predicate = NamedNodeRef::new(predicate.as_str()).expect("could not get the predicate node ");
                        writer.write(TripleRef::new(
                            subject,
                            predicate,
                            object
                        )).expect("error writing triple");
                    }
                }
            }
        }
    }

    pub fn write_intersections<T:Write>(&self, writer : &mut TripleWriter<T>){
        if let Some(classification) = &self.zbmath_classifications{
            let base = Url::parse(BASE_URL).expect("error parsing base url");
            let document_str = String::from("an:") + &self.zbmath_document_id.to_string();
            //println!("{}",document_str);
            let encoded = Serializer::new(String::new())
                .append_pair("q",&document_str)
                .finish();
            let mut document_url = base.clone();
            document_url.set_query(Some(&encoded));
            //println!("{}",document_url.as_str());
            let predicate_url = Url::parse(PREDICATE_INTERSECTION).expect("predicate parsing failed");
            let mut hashset_store = HashSet::new();
            let mut hashset_two = HashSet::new();
            for i in &classification.zbmath_classification{
                let mut temp = String::new();
                let mut temp2 = String::new();
                for j in i.chars(){
                    if j.is_numeric(){
                        temp.push(j);
                        temp2.push(j);
                    }else{
                        temp.push(j);
                        break;
                    }
                }
                hashset_two.insert(temp2);
                hashset_store.insert(temp);
            }
            iter_into_query(classification.zbmath_classification.clone(),document_url.clone(),predicate_url.clone(),writer);
            iter_into_query(hashset_store,document_url.clone(),predicate_url.clone(),writer);
            iter_into_query(hashset_two,document_url,predicate_url,writer);

        }
    }

    pub fn write_final_problem<T:Write>(&self,writer: &mut TripleWriter<T>){
        if let Some(authors) =&self.zbmath_author_ids{
            let base = Url::parse(BASE_URL).expect("couldn't parse the uri");
            let predicate_year = Url::parse(PREDICATE_PUBLISHED_YEAR).expect("couldn't parse the year predicate");
            let year_literal = oxigraph::model::Literal::from(self.zbmmath_publication_year);
            let literal_ref = LiteralRef::from(&year_literal);
            let document_str = String::from("an:") + &self.zbmath_document_id;
            let encoded = Serializer::new(String::new())
                .append_pair("q",&document_str)
                .finish();
            let mut document = base.clone();
            document.set_query(Some(&encoded));
            let sub_temp = NamedNodeRef::new(document.as_str()).unwrap();
            let pred_temp = NamedNodeRef::new(predicate_year.as_str()).unwrap();
            writer.write(TripleRef::new(
                sub_temp,
                pred_temp,
                literal_ref,
            )).expect("error writing triple");
            for i in &authors.zbmath_author_id{
                let published_url = Url::parse(PREDICATE_PUBLISHED_BY).expect("couldn't parse url");
                let published_by = NamedNodeRef::new(published_url.as_str()).unwrap();
                let author_str = String::from("ai:") + i;
                let mut author_url = base.clone();
                let encoded = Serializer::new(String::new())
                    .append_pair("q",&author_str)
                    .finish();
                author_url.set_path("authors/");
                author_url.set_query(Some(&encoded));
                let subject = NamedNodeRef::new(author_url.as_str()).unwrap();
                let object = NamedNodeRef::new(document.as_str()).unwrap();
                writer.write(
                    TripleRef::new(
                        subject,
                        published_by,
                        object,
                    )
                ).unwrap();
            }
            if let Some(keywords) = &self.zbmath_keywords{
                let predicate_url = Url::parse(CONTAINS_KEY_WORD).unwrap();
                for i in &keywords.zbmath_keyword{
                    let keyword_str = String::from("ut:") + i;
                    let encoded = Serializer::new(String::new())
                        .append_pair("q",&keyword_str)
                        .finish();
                    let mut key_word_url = base.clone();
                    key_word_url.set_query(Some(&encoded));
                    let keyword_node = NamedNodeRef::new(key_word_url.as_str()).unwrap();
                    let subject = NamedNodeRef::new(document.as_str()).unwrap();
                    let predicate_of = NamedNodeRef::new(predicate_url.as_str()).unwrap();
                    writer.write(
                        TripleRef::new(
                            subject,
                            predicate_of,
                            keyword_node,
                        )
                    ).unwrap();
                }
            }

        }
    }

    pub fn combine<T:Write>(&self,writer: &mut TripleWriter<T>){
        self.write_keywords(writer);
        self.write_intersections(writer);
        self.write_final_problem(writer);
    }
}

pub fn iter_into_query<T,W:Write>(iterable: T, subject: Url, predicate: Url, writer: &mut TripleWriter<W>) where T:IntoIterator<Item=String>{
    for i in iterable{
        let object_str = String::from("cc:") + &i;
        let encoded = Serializer::new(String::new())
            .append_pair("q",&object_str)
            .finish();
        let mut object_url = Url::parse(BASE_URL).expect("failed to create base url");
        object_url.set_path("classification/");
        object_url.set_query(Some(&encoded));
        let subject_node = NamedNodeRef::new(subject.as_str()).unwrap();
        let predicate_node = NamedNodeRef::new(predicate.as_str()).unwrap();
        let object = NamedNodeRef::new(object_url.as_str()).unwrap();
        writer.write(TripleRef::new(
            subject_node,
            predicate_node,
            object,
        )).expect("writing triple failed");

    }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ZbmathAuthorIds {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "author_id")]
    pub zbmath_author_id: Vec<String>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ZbmathClassifications {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "classification")]
    pub zbmath_classification: Vec<String>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ZbmathKeywords {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "keyword")]
    pub zbmath_keyword: Vec<String>,
}

