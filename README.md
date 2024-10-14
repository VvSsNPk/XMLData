# Query MATH-DATA Assignment
In this assignment,
we have to parse a given XML dataset and make RDF triples and load it into a triplestore to query on the data.

For this assignment,
I used as my previous assignments Rust programming language and the Rust ecosystem to parse the data and make Queries.

I have used a triple store call [oxigraph](https://docs.rs/oxigraph/latest/oxigraph/) which has a rust api which works well for this assignment.

To run the program you need to have rust installed with cargo package manager.

Build the project using the following command
````bash 
cargo build --release
````
Run the project using the following command.
````bash 
cargo run --release
````
>>One more Important thing since the triple store uses the disk to store the data and do queries
>you need to either run the script on mini or big but not both at once because it will mix the dataset

Now i will discuss a little bit about the solutions [here](Solution-summary.md)

