use std::{env, collections::HashSet, fs::read_to_string};
use std::fs;
use std::path::Path;
use multimap::MultiMap;
use serde_json::{Map, Value, from_str, to_string, to_value};

fn main() {
    let array: [&'static str; 5] = ["mstBuff", "mstClass", "mstFunc", "mstSkill", "mstSvt"];
    let mut record : MultiMap<i64, String> = MultiMap::new();
    let mut svt_id : HashSet<i64> = HashSet::new();

    let __path = env::args_os().nth(1).unwrap();

    let path = __path.to_str().unwrap();
    for __file in fs::read_dir(Path::new(path)).unwrap() {
        let path_buf = __file.unwrap().path();
        let basename = path_buf.file_stem().unwrap().to_str().unwrap();
        let path = path_buf.to_str().unwrap();

        let file_index = array.iter().position(|allowed_paths| basename == allowed_paths.to_string());
        if path_buf.extension().unwrap().to_str().unwrap().to_ascii_lowercase() == "json" && (file_index != None) {
            // println!("reading file {}", path);
            let content = read_to_string(path).unwrap();
            let deserialized_value : Value = from_str(content.as_ref()).unwrap();
            let records = deserialized_value.as_array().unwrap();

            for __record in records {
                match file_index.unwrap() {
                    // mstBuff
                    0 => {
                        for key in ["vals", "tvals", "ckSelfIndv", "ckOpIndv"].iter() {
                            let vals = __record[key].as_array().unwrap().iter()
                                .map(|value| value.as_i64().unwrap());
                            for val in vals {
                                record.insert(val, array[file_index.unwrap()].to_string());
                            }
                        }
                    },
                    // mstClass
                    1 => record.insert(__record["attri"].as_i64().unwrap() + 99, array[file_index.unwrap()].to_string()),
                    // mstFunc
                    2 => {
                        for key in ["tvals", "questTvals"].iter() {
                            let vals = __record[key].as_array().unwrap().iter()
                                .map(|value| value.as_i64().unwrap());
                            for val in vals {
                                record.insert(val, array[file_index.unwrap()].to_string());
                            }
                        }
                    },
                    // mstSkill
                    3 => {
                        for key in ["actIndividuality"].iter() {
                            let vals = __record[key].as_array().unwrap().iter()
                                .map(|value| value.as_i64().unwrap());
                            for val in vals {
                                record.insert(val, array[file_index.unwrap()].to_string());
                            }
                        }
                    },
                    // mstSvt
                    4 => {
                        for key in ["individuality"].iter() {
                            let vals = __record[key].as_array().unwrap().iter()
                                .map(|value| value.as_i64().unwrap());
                            for val in vals {
                                record.insert(val, array[file_index.unwrap()].to_string());
                            }
                        }
                        // remove servant self traits
                        svt_id.insert(__record["baseSvtId"].as_i64().unwrap());
                    }
                    _ => {}
                }
            }
        }
    }

    // remove "negative" traits
    let mut filtered_records : Vec<(i64, Vec<String>)> = record.into_iter()
        .filter(|_trait| _trait.0.is_positive() && !svt_id.contains(&_trait.0))
        .collect();

    filtered_records.sort_unstable_by_key(|pair| pair.0);

    // aggregate traits
    let mut output = Map::new();
    for mut mapping in filtered_records {
        let (ref mut _trait, ref mut occurrences) = mapping;
        occurrences.sort_unstable();
        occurrences.dedup();
        output.insert((*_trait).to_string(), to_value(occurrences.clone()).unwrap());
    }

    println!("{}", to_string(&Value::Object(output)).unwrap());
}
