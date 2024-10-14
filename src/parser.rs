use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use bzip2::Compression;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use oxigraph::io::{GraphFormat, GraphSerializer};
use oxigraph::io::write::TripleWriter;
use quick_xml::{Reader, Writer};
use quick_xml::de::from_str;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::name::{LocalName, QName};
use crate::constants::SET_CONTAINS;
use crate::xml_map::Record;

#[derive(Debug)]
pub struct XMLParse{
    input_file : PathBuf,
    output_file : PathBuf,
}

impl XMLParse{
    pub fn new(input_file: PathBuf,output_file:PathBuf) -> Self{
        Self{
            input_file,
            output_file,
        }
    }

    pub fn parse_xml_file(&self) -> Result<(),io::Error>{
        let file_to_read = File::open(&self.input_file)?;
        let decoder_reader = BzDecoder::new(file_to_read);
        let mut reader = BufReader::new(decoder_reader);
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;
        let mut parser = Reader::from_reader(reader);
        parser.config_mut().trim_text(true);
        let mut store = Vec::new();
        let mut in_set = false;
        let mut counter = 0;
        let mut event_store = Vec::new();
        let mut writer = Writer::new(&mut event_store);
        writer.write_event(Event::Start(BytesStart::new("record"))).unwrap();
        if let Ok(Some(mut triple_writer)) = self.writer_of_xml() {
            loop {
                match parser.read_event_into(&mut store) {
                    Ok(Event::Eof) => { break; },
                    Ok(Event::Start(x)) => {
                        if SET_CONTAINS.contains(&x.local_name()) {
                            writer.write_event(Event::Start(x.clone())).unwrap();
                            in_set = true;
                        }
                    },
                    Ok(Event::Text(e)) => {
                        if in_set {
                            writer.write_event(Event::Text(e.clone())).unwrap();
                        }
                    },
                    Ok(Event::End(x)) => {
                        if SET_CONTAINS.contains(&x.local_name()) {
                            writer.write_event(Event::End(x.clone())).unwrap();

                            in_set = false;
                        }
                        if x.local_name() == LocalName::from(QName(b"record")) {
                            counter += 1;
                            writer.write_event(Event::End(BytesEnd::new("record"))).unwrap();
                            let str = writer.into_inner().to_vec();
                            let new_str = std::str::from_utf8(&str).unwrap();
                            let root_element: Record = from_str(new_str).expect("error deserializing the record type");
                            root_element.combine(&mut triple_writer);
                            event_store.clear();
                            writer = Writer::new(&mut event_store);
                            writer.write_event(Event::Start(BytesStart::new("record"))).unwrap();
                            if counter % 10000 == 0 {
                                println!("{counter}");
                            }
                        }
                    },
                    Ok(_) => {},
                    Err(_) => {}
                }
                store.clear();
            }
            triple_writer.finish()?;
        }else{
            println!("File already exists ?");
        }

        Ok(())
    }

    pub fn writer_of_xml(&self) -> Result<Option<TripleWriter<BzEncoder<BufWriter<File>>>>,io::Error>{
        if !&self.output_file.exists(){
            let file_to_write = File::create(&self.output_file)?;
            let writer_triple_data = BufWriter::new(file_to_write);
            let mut encoder_to_bz2 = BzEncoder::new(writer_triple_data,Compression::default());
            let mut triple_writer = GraphSerializer::from_format(GraphFormat::NTriples).triple_writer(encoder_to_bz2)?;
            return Ok(Some(triple_writer));
        }
        Ok(None)
    }
}