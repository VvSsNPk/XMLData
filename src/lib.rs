use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use oxigraph::model::Term;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use quick_xml::{de, Writer};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::se::to_writer;
use crate::problem::Problems;
use crate::solution::{Author, Solution, Solutions};

pub mod solution;
pub mod problem;

pub mod constants;
pub mod parser;
pub mod xml_map;

pub fn make_queries(store: Store,problem_file:PathBuf){
    assert!(problem_file.exists(),"problem file doesn't exist");
    assert_eq!(problem_file.extension().unwrap().to_str().unwrap(),"xml","wrong format need xml file");
    let file_name = problem_file.file_name().unwrap().to_str().unwrap();
    let solutions_file = file_name.replace("problems","solutions");
    let file = File::open(problem_file).unwrap();
    let reader = BufReader::new(file);
    let problems : Problems = de::from_reader(reader).unwrap();
    let mut solutions = Solutions::new();
    let keyword_query = r"
        SELECT ?s
        WHERE {
            <{object}> <https://zbmath.org/isKeyword> ?s.
        }
    ";
    let intersection_query = r"
        SELECT ?s
        WHERE {
        {triples}
        }
    ";

    let final_query = r"
        SELECT ?p (COUNT(?k) as ?published)
            WHERE {
            ?p <https://zbmath.org/isKeyword> <{keyword}>.
            ?p <https://zbmath.org/isPublishedby> ?k.
            ?k <https://zbmath.org/containsKeyword> <{keyword}>.
            ?k <https://zbmath.org/inYear> ?x.
            FILTER (?x < {year1} && ?x > {year2}).
                }
            GROUP BY ?p
            ORDER BY DESC(?published) ?p
            LIMIT 10";
    let intersection_helper = r"?s <https://zbmath.org/isIntersection> <{object}>.";
    let mut file_to_create = PathBuf::new();
    file_to_create.push(solutions_file);
    file_to_create.set_extension("xml");
    let file_creation = File::create(file_to_create).unwrap();
    let mut bufwriter = BufWriter::new(file_creation);
    let mut writer = Writer::new(bufwriter);
    let start_event = Event::Start(BytesStart::new("Solutions"));
    writer.write_event(start_event).unwrap();
    for i in problems.problem{
        let mut bytes = BytesStart::new("Solution");
        bytes = bytes.with_attributes(vec![("id",i.id.to_string().as_str())]);
        writer.write_event(Event::Start(bytes)).unwrap();
        if i.problem_type == "keywords"{
            if let Some(author) = i.author{
                let author_query = keyword_query.replace("{object}",&author);
                writer.write_event(Event::Start(BytesStart::new("Query"))).unwrap();
                writer.write_event(Event::Text(BytesText::new(&author_query))).unwrap();
                writer.write_event(Event::End(BytesEnd::new("Query"))).unwrap();
                let mut solution = Solution::new(i.id,author_query.clone());
                if let Ok(QueryResults::Solutions(solutions_store)) = store.query(&author_query){
                    //let mut store = Vec::new();
                    for i in solutions_store{
                        if let Ok(query_sol) = i{
                            let qu = query_sol.get("s").unwrap().to_string().trim_end_matches(">").trim_start_matches("<").to_string();
                            writer.write_event(Event::Start(BytesStart::new("Keyword"))).unwrap();
                            writer.write_event(Event::Text(BytesText::new(&qu))).unwrap();
                            writer.write_event(Event::End(BytesEnd::new("Keyword"))).unwrap();
                        }
                    }
                    //solution.keyword = Some(store);
                    //solutions.solution.push(solution);
                }
            }
        }else if i.problem_type == "msc-intersection"{
            if let Some(classification) = i.classification{
                let mut intersection_triples = String::new();
                for i in classification{
                    let class_query_helper = intersection_helper.replace("{object}",&i);
                    intersection_triples.push_str(&class_query_helper);
                    intersection_triples.push_str("\n");
                }
                let intersection_query_final = intersection_query.replace("{triples}",intersection_triples.trim());
                writer.write_event(Event::Start(BytesStart::new("Query"))).unwrap();
                writer.write_event(Event::Text(BytesText::new(&intersection_query_final))).unwrap();
                writer.write_event(Event::End(BytesEnd::new("Query"))).unwrap();
                //let mut solution_store = Solution::new(i.id,intersection_query_final.clone());
                //println!("{}",intersection_query_final);
                if let QueryResults::Solutions(sol) = store.query(&intersection_query_final).unwrap(){
                    //let mut store = Vec::new();
                    for i in sol{
                        if let Ok(s) = i{
                            let qr_s = s.get("s").unwrap().to_string().trim_end_matches(">").trim_start_matches("<").to_string();
                            writer.write_event(Event::Start(BytesStart::new("Paper"))).unwrap();
                            writer.write_event(Event::Text(BytesText::new(&qr_s))).unwrap();
                            writer.write_event(Event::End(BytesEnd::new("Paper"))).unwrap();
                        }
                    }
                    //solution_store.paper = Some(store);
                    //solutions.solution.push(solution_store);
                }
            }

        }else{
            let mut query_final = final_query.replace("{keyword}", &i.keyword.unwrap());
            query_final = query_final.replace("{year1}",&i.before_year.unwrap());
            query_final = query_final.replace("{year2}",&i.after_year.unwrap());
            writer.write_event(Event::Start(BytesStart::new("Query"))).unwrap();
            writer.write_event(Event::Text(BytesText::new(&query_final))).unwrap();
            writer.write_event(Event::End(BytesEnd::new("Query"))).unwrap();
            let mut solution_store = Solution::new(i.id,query_final.clone());
            if let QueryResults::Solutions(sol) = store.query(&query_final).unwrap(){
                //let mut store = Vec::new();
                for i in sol{
                    if let Ok(s) = i{
                        if let &Term::Literal(x) = &s.get("published").unwrap(){
                            let stt = x.clone().destruct();
                            let mut start = BytesStart::new("Author");
                            start = start.with_attributes(vec![("count",stt.0.as_str())]);
                            writer.write_event(Event::Start(start)).unwrap();
                        }
                        let second = s.get("p").unwrap().to_string().trim_end_matches(">").trim_start_matches("<").to_string();
                        writer.write_event(Event::Text(BytesText::new(&second))).unwrap();
                        writer.write_event(Event::End(BytesEnd::new("Author"))).unwrap();


                    }
                }
                //solution_store.author = Some(store);
                //solutions.solution.push(solution_store);
            }
        }
        writer.write_event(Event::End(BytesEnd::new("Solution"))).unwrap();
    }
    writer.write_event(Event::End(BytesEnd::new("Solutions"))).unwrap();
}