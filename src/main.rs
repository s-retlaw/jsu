use std::error::Error;
use serde_json::{Deserializer, Value};
use clap::{App, Arg};
use std::io;
use std::io::Read;

fn get_next_open_bracket_offset(input : &str, offset: usize) -> usize{
    let mut result = offset;
    let(_, s) = input.split_at(offset);
    for c in s.chars() {
        if c == '{' || c == '[' {break;}
        result += 1;
    }
    result
}

fn has_object(a : &Vec<Value>) -> bool{
    for v in a {
        if v.is_object(){
            return true;
        }
    }
    false
}

fn extract_json_objects(input :&str) -> Value{
    let mut results = Vec::new();
    let mut offset = 0;
    loop{
        offset = get_next_open_bracket_offset(input, offset);
        if offset >= input.len() {break;}
        let (_, s) = input.split_at(offset);
        let mut stream = Deserializer::from_str(s).into_iter::<Value>();
        let value = stream.next();
        match value {
            Some(Ok(v)) => {
                if let Some(obj) = v.as_object(){
                    if !obj.is_empty(){ results.push(v); } 
                    offset += stream.byte_offset();
                }else if let Some(a) = v.as_array(){
                    if has_object(a){
                        results.push(v);
                        offset += stream.byte_offset();
                    }else{
                        offset += 1;
                    }
                }
            },
            _ => {
                offset += 1;
            }
        };
        if offset >= input.len(){break;}
    }
    Value::Array(results)
}

fn expand_json_value(js : Value) -> Value{
    match js {
        Value::String(s) => {
            match serde_json::from_str(&s) {
                Ok(v) => v,
                Err(_err) => Value::String(s) 
            }
        },
        Value::Array(a) => {
            let new_a =  a.into_iter().map(expand_json_value).collect::<Vec<Value>>();
            Value::Array(new_a)
        },
        Value::Object(mut o) => {
            let keys = o.keys().map(|k| k.to_string()).collect::<Vec<String>>();
            for key in keys{
                o[&key] = expand_json_value(o[&key].clone());
            }
            Value::Object(o)
        }
        _ => js
    }
}

fn get_input(file_name : Option<&String>) -> Result<String, Box<dyn Error>> {
    Ok(match file_name {
        Some(name) => {
            std::fs::read_to_string(name).map_err(|e| format!("Error reading input file {name} : {e}"))?
        },
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).map_err(|e| format!("Error reading from stdin : {e}"))?;
            input
        }
    })
}

fn write_output(output_file : Option<&String>, data: &str) -> Result<(), Box<dyn Error>> {
    match output_file{
        Some(file_name) => {
            std::fs::write(file_name, data)?;
        },
        None => {
            println!("{data}");
        }
    };
    Ok(())
}

fn run_cmd_line() -> Result<(), Box<dyn Error>>  {
    let matches = App::new("jsu")
        .version("0.2.0")
        .author("Walter Szewelanczyk")
        .about("Json Utils")
        .arg(Arg::new("input_file")
            .short('f')
            .long("file")
            .help("The file to process.  If not present it will use stdin.")
            .takes_value(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("The file to write the output to.  Will write to stdout if not present.")
            .takes_value(true))
        .arg(Arg::new("compact")
            .short('c')
            .long("compact")
            .takes_value(false)
            .action(clap::ArgAction::SetTrue)
            .help("use a compact output format.  By default we use a pretty output."))
        .arg(Arg::new("expand")
            .short('e')
            .long("expand")
            .takes_value(false)
            .action(clap::ArgAction::SetTrue)
            .help("Will look at the JSON and look at string values to see if they are encoded JSON and if so it will expand and repalce the string with actual JSON data"))
        .arg(Arg::new("extract")
            .short('x')
            .long("extract")
            .takes_value(false)
            .action(clap::ArgAction::SetTrue)
            .help("Scan the text looking for any embeded JSON objects and return them.  Will return an array if more than one."))
        .get_matches();

    let input_file = matches.get_one::<String>("input_file");
    let output_file = matches.get_one::<String>("output");
    let compact = *matches.get_one::<bool>("compact").unwrap();
    let expand = *matches.get_one::<bool>("expand").unwrap();
    let extract = *matches.get_one::<bool>("extract").unwrap();

    let input = get_input(input_file)?;
    let mut json = if extract { 
        extract_json_objects(&input)
    }else{
        serde_json::from_str::<Value>(&input).map_err(|e| format!("Error parsing JSON : {e}"))?
    };
    if expand {
        json = expand_json_value(json);
    }
    let output = if compact{
        serde_json::to_string(&json).map_err(|e| format!("Error converting JSON to a compact string : {e}"))?
    }else{
        serde_json::to_string_pretty(&json).map_err(|e| format!("Error converting JSON to a string : {e}"))?
    };
    write_output(output_file, &output).map_err(|e| format!("Error writing to output : {e}"))?;
    Ok(())
}


fn main(){
    if let Err(e) = run_cmd_line() {
        eprintln!("{}", e)
    }
}
