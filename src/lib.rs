use std::io::{BufWriter, Write,BufReader, BufRead};
use std::fs::OpenOptions;
use serde_json::{Value};
use std::fs::File;
use std::process;
use std::fs;

pub fn file()->String{
    String::from(r#"D:\temp\contents.txt"#)
}

pub fn update_list(a:String){
    
    let v: Value = serde_json::from_str(&a).expect("Unable to convert to JSON");
    let f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file())
        .expect("Unable to open file");
    let mut f = BufWriter::new(f);
    let data: String=format!("{}~\n",v);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}

pub fn read_all()->String{

    let f = File::open(file()).unwrap_or_else(|err| {
    println!("Unable to open paramter file due the error: {}",err);
    process::exit(1);
    });
    let f = BufReader::new(f);
    let mut a:String="".to_string();
    for line in f.lines(){
        let  temp = format!("{} \n",line.expect("Unable to read the elements from prm file").trim().to_string());
        let mut splitter=temp.splitn(2, '~');
        let first = splitter.next().expect("Unable to split the key from file").trim().to_string();
        let val=format!("{},",first);
        a.push_str(&val);
    }
    a
}


pub fn return_update()->String{
    let mut vec = Vec::new();//Create a vector
    let f = File::open(file()).unwrap_or_else(|err| {
    println!("Unable to open paramter file due the error: {}",err);
    process::exit(1);
    });//open file

    let f = BufReader::new(f);

    for line in f.lines(){
        vec.push(line.expect("").to_string());
    }

    let val= vec.first().expect("Unable to fetch first record").to_string();
    let mut splitter=val.splitn(2, '~');
    let first = splitter.next().expect("Unable to split the key from file").trim().to_string();

    vec.remove(0);
    fs::remove_file(file()).expect("Unable to delete the file");
     let f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file())
        .expect("Unable to open file");
    let mut f = BufWriter::new(f);
    for x in vec.iter(){
        let data=format!("{} \n",x.to_string());
        f.write_all(data.as_bytes()).expect("Unable to write data");
    }
    first
}

