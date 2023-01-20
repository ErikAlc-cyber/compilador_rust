extern crate pest;                                                                                                                                                                                   
#[macro_use]                                                                                                                                                                                         
extern crate pest_derive;                                                                                                                                                                            

// 1. Importamos modulos                                                                                                                                                                                            
use std::fs;
use std::io::Read;                                                                                                                                                                                         
use pest::Parser;                                                                                                                                                                                    
use pest::iterators::Pair;
use std::fs::File;                                                                                                                                                                           

// 2. Definimos nuestroarchivo de gramatica.                                                                                                                                                                                                 
#[derive(Parser)]                                                                                                                                                                                    
#[grammar = "grammar.pest"]                                                                                                                                                                            
struct IdentParser; 

// 3. Imprime los detalles de analisis
fn print_pair(pair: &Pair<Rule>, hard_divider: bool) {
    println!("Terminal: {:?}", pair.as_span());
    println!("Entrada: {:?}", pair.as_str());
    if hard_divider {
        println!("{:=>60}", "");
    } else {
        println!("{:->60}", "");
    }
}

fn main() {

    //4.Declarar el doc de entrada
    let doc_entrada = std::env::args().nth(2).expect("Especifique una archivo porfavor");
    let mut cont = String::new();
    let mut arch = File::open(doc_entrada).unwrap();
    let mut nmb: String = String::new();
    arch.read_to_string(&mut cont).ok();
    cont = cont.replace('\n', " ");

    println!("Doc de entrada:\n{:?}\n\n",cont.as_str());

    // 4. Parse el archivo de entrada
    let pair = IdentParser::parse(Rule::declaration, cont.as_str())
        .expect("No se puede completar el parse")
        .next().unwrap();

    print_pair(&pair, true);

    for inner_pair in pair.into_inner() {
 
        print_pair(&inner_pair, true);

        match inner_pair.as_rule() {
            // 6.Si vemos una identacion:
            Rule::idents =>  {
                // 7. Iteramos sobre la estructura interna
                for inner_inner_pair in inner_pair.into_inner() {
                    match inner_inner_pair.as_rule() {
                        // 8. el termino idencion es el mayor nivel
                        Rule::ident => {
                            print_pair(&inner_inner_pair, false);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}