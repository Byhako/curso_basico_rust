use std::io::{self, Write};
use std::{collections::HashMap, fs};
use csv::{ReaderBuilder,StringRecord};

const PATH : &str = "history.csv";
const FRIST_TAG: &str = "INICIO";
fn get_input() ->io::Result<String>{
    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
#[derive(Debug, PartialEq)]
struct StoryLine {
    data_type : String,
    tag : String,
    text : String,
    life : i32,
    options : Vec<StoryLine>
}
impl StoryLine {
    fn new(row: StringRecord) -> StoryLine {
        StoryLine {
            data_type : row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life: row.get(3).unwrap().trim().to_string().parse().unwrap_or(0),
            options: Vec::new()
        }
    }
}

fn main() {
    game(get_lines_form_path(PATH));
}

fn get_lines_form_path(path: &str) -> HashMap<String, StoryLine>{
    // let mut no_used_lines : Vec<StoryLine> = Vec::new();
    let mut lines : HashMap<String, StoryLine> = HashMap::new();
    let content = fs::read_to_string(path).unwrap();
    let mut rbr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
    let mut last_line: String = String::default();
    
    for line in rbr.records(){
        let line = line.unwrap();
        let new_line = StoryLine::new(line);
        if new_line.data_type == "SITUACION" {
            let tag = new_line.tag.clone();
            lines.insert(tag.clone(), new_line);
            last_line = tag
        }else if new_line.data_type == "OPCION" {
            if let Some(situacion) = lines.get_mut(&last_line){
                situacion.options.push(new_line);
            }
            // else {no_used_lines.push(new_line);}
        }
    }
    lines
}
fn game(lines: HashMap<String, StoryLine>){
    let mut life : i32 = 100;
    let mut current_tag: String = FRIST_TAG.to_string();
    let mut is_just_selection: bool = true;
    loop {
        if life <=0 {
            println!("Fin del juego"); break;
        }
        if let Some(line) = lines.get(&current_tag){
            if is_just_selection{life += line.life} //si una entrada es invalida, se vuelve a ejecutar esto
            println!("Vida: {}",life);
            println!("{}", line.text);
            for (index,option) in line.options.iter().enumerate(){
                println!("{{{}}}{}", index, option.text)
            }
            let chosen : usize = get_input().unwrap().trim().parse().unwrap_or(usize::max_value());
            if let Some(chose_option) = line.options.get(chosen){
                is_just_selection = true;
                current_tag = chose_option.tag.clone();
            } else {
                println!("Entrada invalida");
                is_just_selection = false
            }
            println!("")
        } else {break;}
    }
}
