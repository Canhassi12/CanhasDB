use std::fmt::Error;
use std::fmt::{self, Display};
use std::fs::OpenOptions;
use std::io::Write;

type Declaration = (String, String);
type Dictionary = Vec<Declaration>;

fn get(dict: &Dictionary, key: &String) -> Option<String> {
    for (dict_key, dict_value) in dict.iter() {
        if *dict_key == *key {
            return Some(dict_value.to_owned());
        }
    }
    None
}

fn set(dict: &mut Dictionary, key: &String, value: String) {
    let _ = remove(dict, &key);

    dict.push((key.clone(), value));
}

fn remove(dict: &mut Dictionary, key: &String) -> Result<(), ErrorF> {
    for (i, (dict_key, _)) in dict.clone().iter().enumerate() {

        if dict_key == key {
            dict.remove(i);
            return Ok(());
        }
    }
    Err(ErrorF)
}

#[derive(Debug)]
struct ErrorF;

impl fmt::Display for ErrorF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "deu ruim meu bom!!")
    }
}

fn main() {
    // let oi = std::fs::read_to_string("fodase.json").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider() -> Vec<(String, String)> {
        let dict = vec![
            (String::from("fds"), String::from("value")),
            (String::from("aaaaa"), String::from("oiee")),
            (String::from("canhas"), String::from("222")),
        ];

        dict
    }

    #[test]
    fn it_can_get_a_value() {
        let dict = provider();
        let result = get(&dict, &String::from("fds"));

        assert_eq!(Some(String::from("value")), result);
    }

    #[test] 
    fn it_can_set_a_new_entry() {
        let mut dict = provider();

        set(&mut dict, &String::from("joazinho"), String::from("10"));

        assert_eq!(dict.get(3), Some(&(String::from("joazinho"), String::from("10"))));
    } 

    #[test] // testar se o metodo set cria uma nova entrie
    fn it_can_delete_a_entry() {
        
        let mut dict = provider();
        
        let _ = remove(&mut dict, &String::from("canhas"));

        assert_eq!(dict.get(2), None);
    }
    
    #[test]
    fn it_key_should_persist_duplicate_entries() {
        let mut dict = provider();

        set(&mut dict, &String::from("joazinho"), String::from("10"));

        assert_ne!(dict.get(3), Some(&((String::from("canhas"), String::from("222")))));
    }
}



// let oi = std::fs::read_to_string("fodase.json").unwrap();


// let _ = remove(dict, key);

// let mut file = OpenOptions::new()
//     .read(true)
//     .append(true)
//     .open("fodase.json")
//     .expect("fodeu rapaziada");

// file.write(format!("[\"{key}\", \"{value}\"]").as_bytes())
//     .expect("fodeeeeuuuuuuuuuuuuuuuuuuuuuuuu");

// struct Database {
//     fields: HashMap<String, Vec<u8>>,
// }

// impl Database {
//     fn create(&mut self, key: String, value: Vec<u8>) -> Option<Vec<u8>> {
//         self.fields.insert(key, value)
//     }

//     fn read(&self, key: String) -> Option<&Vec<u8>> {
//         self.fields.get(&key)
//     }

//     fn update(&mut self, key: String, value: Vec<u8>) -> Option<Vec<u8>> {
//         self.fields.insert(key, value)
//     }

//     fn delete(&mut self, key: String) -> Option<Vec<u8>> {
//         self.fields.remove(&key)
//     }
// }

// let mut test = Database {
//     fields: HashMap::new(),
// };

// create

// test.create(String::from("fodase"), vec![12]);

// read

// let a = test.read(String::from("fodase"));

// println!("read: {:?}", a);

// update

// test.update(
//     String::from("fodase"),
//     String::from("alallalalalallalalla").into_bytes(),
// );

// let b = String::from_utf8(test.read(String::from("fodase")).unwrap().to_vec());

// println!("read after update: {:?}", b);

// delete

// test.delete(String::from("fodase"));

// test.read(String::from("fodase"));

// println!("delete: {:?}", test.fields);




// fn action(&mut self, action: Action) {
//     match action {
//         Action::Create(key, value) => self.create(key, value),
//         Action::Read(key) => self.read(key),
//         Action::Update(key, value) => self.update(key, value),
//         Action::Delete(key) => self.delete(key),
//     };
// }

// #[derive(Debug)]
// enum ActionResponse {
//     Create(Option<Vec<u8>>),
//     Read(Vec<u8>),
//     Update(Option<Vec<u8>>),
//     Delete(Option<Vec<u8>>),
// }

// enum Action {
//     Create(String, Vec<u8>),
//     Read(String),
//     Update(String, Vec<u8>),
//     Delete(String),
// }
