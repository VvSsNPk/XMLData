# Solution summary for Query Math Data in ZbMath.org
## Problem Definition
This assignment is different from previous assignments. In this assignment, we are given a large dataset.
We need to parse the dataset and build a RDF triples with the data and load it into a triple store and make queries.
The main Challenges we face in this assignment is the large amount of data set and parsing it efficiently and loading in
efficiently in the triple store. Also optimizing the queries to get the results that we wanted.

The RDF graphs are based on concept called semantic web, which essentially established an isa and insta relations between 
concept and subject. Here also the concept of an RDF triple is also the same. With RDF-triples, a graph is constructed and loaded
into a triple store which can be used later to query and get some results.

## Efficiently Parsing the data using the Quick_XML crate (SAX) parsing:

In this assignment as i mentioned previously, the challenge is to parse the big dataset. For this i used a lightning fast rust crate
that supports SAX parsing called quick_xml.So SAX parsing is Event-based parsing ulike DOM parsing. In DOM parsing the program contains
a big object that represents an XML file, but the problem is that the object is loaded in memory and the data set is too large to load
it in memory.

SAX parsing is event based parsing where we read each an individual Event and parse it which is not stored as a whole in memory but uses a 
buffer to parse individual event's.  Now since i dont need all the parts of the dataset i parse the whole dataset and take the neccessary events 
and make new xml out of it and parse as a Record type neccessary for this assignment. This record type takes a triple writer and serializes it into
a rdf triples and writes the neccessary triples with the use of SERDE.

## Linkin the DATA as RDF triples
After parsing we need to also get triples that represent following:
>> Subject: URI Predicate: URI object: URI

here the subject is a URI which is a link to a webpage of just a URI which convey's some data and i made my own custom predicates and the object is 
also an URI. I used the specified URI's as per described in the PDF for this project.

For the 1st query my triple store contains the following relation as a triple.
I used turtle syntax for RDF triples by the way.

>>Subject: Document URI pedicat: isKeyword URI object: keyword URI 

This is done as per specified in the PDF
The rest relations can be checked in the soutions XML files since the queries are also a triples it will explain a lot on how the data set is store, i.e
using which kind of relations

## Querying the DATA
Now after the data is processed and made into a data base consists of a triples it will be loaded into oxigraph which supports bulk loading of the data
and Also writes the data base into a local folder. Once the data base is store in data_base_store folder it doesnot need to be build again the program 
automatically uses it and queries the data base to get the answers.

## Summary
At the end i tried to optimize the database parsing and my program can parse and create a database in around 20 minutes for the big dataset. I also hard coded
all the queries for each problem type and the program will open the neccessary  triple store and will run the query on it .

