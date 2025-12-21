use std::error::Error;
use std::path::Path;

use ironworks::{
    Ironworks,
    excel::{Excel, Language},
    sqpack::{Install, SqPack},
};
use regex::Regex;

mod config;
mod exd_schema;
mod export;
mod formatter;

const LANGUAGES: [Language; 4] = [
    Language::English,
    Language::German,
    Language::French,
    Language::Japanese,
];

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::read().expect("Could not read config");
    let path = Path::new(&config.path);

    let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(path)));
    let mut excel = Excel::new(ironworks);

    // Skip sheets without schemas (quest/, custom/, etc.)
    let skip_sheet_regex = Regex::new(r"\/").unwrap();

    for language in LANGUAGES {
        excel.set_default_language(language);

        for sheet in excel.list().unwrap().iter() {
            if skip_sheet_regex.is_match(&sheet) {
                continue;
            }

            export::sheet(&excel, language, &sheet)?;
        }
    }

    // Quick debugging for schema updates

    // for language in LANGUAGES {
    //     excel.set_default_language(language);
    //     export::sheet(&excel, language, &String::from("Mount"))?;
    // }

    // let language = Language::English;
    // excel.set_default_language(language);
    // export::sheet(&excel, language, "Mount")?;

    Ok(())
}
