use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Solutions {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Solution")]
    pub solution: Vec<Solution>,
}

impl Solutions {
    pub fn new() -> Self{
        Self{
            text: None,
            solution: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Solution {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Keyword")]
    pub keyword: Option<Vec<String>>,
    #[serde(rename = "Paper")]
    pub paper: Option<Vec<String>>,
    #[serde(rename = "Query")]
    pub query: String,
    #[serde(rename = "Author")]
    pub author: Option<Vec<Author>>,
}

impl Solution{
    pub fn new(id: u32,query: String) -> Self{
        Self{
            id,
            text: None,
            keyword: None,
            paper : None,
            query,
            author: None,
        }
    }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Author {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

