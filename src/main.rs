extern crate bzip2;
extern crate oxigraph;
extern  crate quick_xml;
extern crate url;
extern crate xml;

use crate::xml_map::Record;
use bzip2::Compression;
use oxigraph::io::{DatasetFormat, GraphFormat, GraphSerializer};
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use quick_xml::de::*;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::name::{LocalName, QName};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::time::Instant;
use oxigraph::model::Term;
use team172::make_queries;
use crate::constants::SET_CONTAINS;
use crate::parser::XMLParse;
use crate::problem::Problems;
use crate::solution::Solutions;

pub mod xml_map;
pub mod solution;
pub mod problem;
pub mod constants;
pub mod parser;
fn main(){
    let time = Instant::now();
    let mut input_file = PathBuf::new();
    input_file.push("mini-dataset (1).xml.bz2");
    let mut out_put_file = PathBuf::new();
    out_put_file.push("database.nq.bz2");
    let xml_parser = XMLParse::new(input_file,out_put_file.clone());
    xml_parser.parse_xml_file().expect("couldnot parse the file");


    println!("finished parsing the file");
    let time_taken = time.elapsed();
    println!("time taken to parse the file is : {}",time_taken.as_secs());
    let file = File::open(out_put_file).expect("cannot open file");
    let encoder = bzip2::read::BzDecoder::new(file);
    let mut bufreader = BufReader::new(encoder);
    let mut database_store = PathBuf::new();
    database_store.push("database_store_mini");
    let mut problem_file = PathBuf::new();
    problem_file.push("problems-mini.xml");
    if database_store.exists(){
        if !database_store.is_dir(){
            println!("expected a data base store as a directory but found maybe a file ?")
        }
        let mut store = Store::open(database_store).unwrap();
        make_queries(store,problem_file);
    }else {
        let mut store = Store::open(database_store).unwrap();
        store.bulk_loader().load_dataset(&mut bufreader, DatasetFormat::TriG, None).unwrap();
        store.flush().expect("flushed the store so that data goes to the database");
        make_queries(store,problem_file);
    }
}
