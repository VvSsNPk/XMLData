use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Problems {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Problem")]
    pub problem: Vec<Problem>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Problem {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@type")]
    pub problem_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Author")]
    pub author: Option<String>,
    #[serde(rename = "Classification")]
    pub classification: Option<Vec<String>>,
    #[serde(rename = "AfterYear")]
    pub after_year: Option<String>,
    #[serde(rename = "BeforeYear")]
    pub before_year: Option<String>,
    #[serde(rename = "Keyword")]
    pub keyword: Option<String>,
}

