// use std::path::PathBuf;

use std::{env, fs, path::PathBuf};

use anyhow::{Error, Ok};
use xenosphere_parser::{
    // parsers::parse_makeup_token,
    tokens::makeup_lang::XeslFileContent,
};

use glob::glob;

use crate::CommonArgs;

pub fn parse(input: &CommonArgs) -> Result<(), Error> {
    let mut results: Vec<XeslFileContent> = Vec::new();

    let mut checked_files = vec![];
    for input_file in &input.files {
        if input_file.exists() && input_file.is_file() {
            // Process each file here
            checked_files.push(input_file.to_owned());
        } else if input_file.exists() && input_file.is_dir() {
            // Process each directory here
            glob(&format!("{}/**/*.xesl", input_file.to_str().unwrap()))?
                .filter_map(Result::ok)
                .for_each(|path| checked_files.push(path));
        }
    }

    let mut cache_path = input.cache_path.clone();
    if !cache_path.exists() {
        // Process cache directory here
        if cache_path.is_relative() {
            cache_path = env::current_dir()?.join(cache_path);
        }
        // println!("cache path : {:?}", cache_path);

        fs::create_dir_all(&cache_path)?
    }
    // println!("cache path : {:?}", cache_path);

    for file in checked_files {
        // file.is_absolute()

        let mut tokens_content = XeslFileContent::from_file(&file.canonicalize()?)?;
        // println!("parsed : {:?}", tokens_content);
        tokens_content.cache_path =
            PathBuf::from(cache_path.to_owned()).join(file.file_name().unwrap());
        tokens_content.cache_path.set_extension("bincode");

        // println!("{:?}", tokens_content.cache_path);
        results.push(tokens_content);
    }

    // let mut export_format: Vec<u8>;

    // results.into_iter().for_each(|res| {
    for res in results {
        // println!("{:?}", res);
        let mut export_path = res.file_path.with_extension("json");
        let mut should_direct_print = false;
        if input.output.as_ref().is_some() {
            if let Some(output_path) = input.output.as_ref() {
                if output_path.is_dir() {
                    export_path = output_path
                        .join(export_path.file_name().unwrap())
                        .canonicalize()
                        .unwrap();
                } else {
                    export_path = output_path.clone();
                }
            }

            // Process output file here
            // if input.output.is_dir() {
            if input.is_debug {
                export_path = res.file_path.with_extension("json");
            }

            // } else {
            // }
        } else if input.output.as_ref().is_none_or(|x| x.exists() == false) {
            // Handle non-existent output file
            // println!("Output file does not exist");
            should_direct_print = input.is_debug;
        }

        if should_direct_print {
            res.print();
        } else {
            // Serialize and write to file
            if input.is_debug {
                res.write_debug_cache(&export_path)?;
            } else {
                res.write_cache(&export_path)?;
            };
            // std::fs::write(&export_path, serialized).unwrap();
        }
        res.write_cache(&res.cache_path)?;
    }

    Ok(())
}
