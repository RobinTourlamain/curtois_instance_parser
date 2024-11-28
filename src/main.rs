mod parser;

fn main() {

    let instance = parser::parse_curtois2014(".\\instances\\Instance1.txt");
    println!("{:#?}", instance);

}
