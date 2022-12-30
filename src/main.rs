use std::{collections::HashMap, io::Read};
use std::fs::File;
use regex_automata::{MultiMatch, dfa::regex::Regex};

/*
El objetivo es hacer un compilador para C
Pasos de un compilador: 
1.-Analizador Lexico
2.-Analizador Sintactico
3.-Analizador Semantico
4.-Codigo intermedio
5.-Optimizador
6.-Codigo
7.-STONKS!!!

Bonus:
+ Manejador de errores
+ Admin de tabla de simbolos
*/


//Delcaracion de tablas para poder hacer el analisis lexico/sintactico
#[derive(Debug)]
struct Token{
    map: HashMap<u32, String>,
    table: HashMap<String, String>
}


impl Token {
    fn new(entries:String) -> Result<Token, std::io::Error>{
        
        let mut id:u32 = 0;
        
        let mut map: HashMap<u32, String> = HashMap::new();
        let mut table: HashMap<String, String> = HashMap::new();


        println!("{}", entries);

        for c in entries.split("'") {
            if c.is_empty(){
                break;
            }
            else{
                let mut values = c.split(':');

                let key = values.next().expect("No Key");
                let val = values.next().expect("No Value");
                
                table.insert(String::from(key), String::from(val));
                map.insert(id, c.to_owned());
                
                id = id + 1;
            }
        }
        Ok(Token {map, table})
    }
    
}

//Fases de compilacion
fn comp(doc:String, _exe:String){

    let mut cont = String::new();
    let mut arch = File::open(doc).unwrap();
    let mut nmb: String = String::new();
    let mut nmb_prov: String = String::new();
    let mut palabra:String = String::new();

    arch.read_to_string(&mut cont).ok();
    cont = cont.replace('\n', " ");
    cont = cont.replace('{', " {");
    cont = cont.replace('(', " (");

    //Fase #1: Analizador Lexico
    println!("Contenido del archivo: \n");
    println!("{}",cont);
    println!("\nInicia analizador: ");
    for character in cont.chars(){
        if  character.is_ascii_whitespace() || character == ';'{
            if !palabra.is_empty(){
                nmb_prov = analizer(palabra);
                nmb = format!("{}{}'",nmb,nmb_prov);
                palabra = String::new();
            }  
        }   
        else {
            palabra.push(character);
        }
    }

    if  !nmb_prov.is_empty(){
        let tokens = Token::new(nmb).expect("Fallo en el analisis lexico");
        
        println!("\n Resultado");
        println!("\nTokenisado: {:#?}", tokens); 
    }
    else{

    }

    //Fase #2: analisis Sintactico, aunque creo que ya lo hace el #1

}

//Esta funcion reconoce el token y lo clasifica
fn analizer(word:String) -> String{

    let mut id = String::new();

    //Empieza el automata para el reconocimiento de palabras y operandos
    let pch = word.chars().next().unwrap();

    println!("Palabra: {}, primera letra:{}", word, pch);
    

    match pch{
        '0' ..= '9' => id = isNumber(&word),
        'A'..='Z' | 'a'..='z' => id = isWord(&word),
        '<' | '>' | '=' => id = isOpRel(&word),
        _ => println!("Error, expresion no reconocida: {}", word)
    }

    return format!("{}:{}",word, id);
}

fn isNumber(word:&String) -> String{
    let re = Regex::new(r"{0-9}+.{0-9}?E{+-}?[{0-9}+]?");
    let check = 0;
    return  "Numero".to_string();
}

fn isWord(word:&String) -> String{
    let re = Regex::new(r"[{a-z}|{A-Z}_][[{a-z}|{A-Z}_]|{0-9}]*");
    return "Palabra".to_string();
}

fn isOpRel(word:&String)-> String {
    let re = Regex::new(r"<|>|<=|=>|==|<>");
    return "Operacion Relacional".to_string();    
}

//Este se va a tener que transformar en varias funcones para simplificar el proceso


fn main() {

    let doc_entrada = std::env::args().nth(2).expect("Especifique una archivo porfavor");
    let action: String = std::env::args().nth(1).expect("Especifique una accion porfavor");

    //Detecta el archivo de entrada y una operacion especifica para el archivo

    match action.as_ref() {
        "-c" => {
            comp(doc_entrada, "a".to_owned()); 
        },
        "-o" => {
            let doc_salida = std::env::args().nth(3).expect("Especifique una nombre porfavor");
            comp(doc_entrada, doc_salida);
        },
        _ => println!("Instruccion invalida") 
    }
}