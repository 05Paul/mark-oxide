use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use serde::Deserialize;
use crate::error::Error;
use crate::{Input, Output};

const CURRENT_URL: &str = "https://spec.commonmark.org/current";
const BASE_URL: &str = "https://spec.commonmark.org";
const DOMAIN_NAME: &str = "spec.commonmark.org";

#[derive(Deserialize)]
pub struct TestCase {
    markdown: String,
    html: String,
    example: usize,
    start_line: usize,
    end_line: usize,
    section: String,
}

impl TestCase {
    fn into_test_function(self) -> String {
        format!("
                #[test]
                /// {}
                fn test_example_{:03}() {{
                    let parser = Parser::from_reader({:?}.as_bytes());
                    assert_eq!({:?}, parser.parse_to_string().unwrap_or(\"\".into()));
                }}
                ", self.section, self.example, self.markdown, self.html)
    }
}

pub fn test_cases(input: Input) -> Result<Vec<TestCase>, Error> {
    match (input.input_file, input.test_version) {
        (Some(path), _) => from_file(path),
        (_, Some(mut version)) => {
            if version == "latest" {
                version = current_version()?;
            }
            from_web(version)
        }
        _ => Err(Error::Other("Input must have either 'input_file' or 'test_version' set".to_string()))
    }
}

fn from_file<P: AsRef<Path>>(path: P) -> Result<Vec<TestCase>, Error> {
    let input = BufReader::new(File::open(path)?);
    Ok(serde_json::from_reader(input)?)
}

fn from_web(version: String) -> Result<Vec<TestCase>, Error> {
    let response = reqwest::blocking::get(format!("{BASE_URL}/{version}/spec.json"))?;
    Ok(serde_json::from_reader(response.text()?.as_bytes())?)
}

fn current_version() -> Result<String, Error> {
    let response = reqwest::blocking::get(CURRENT_URL)?;
    let text = response.text()?;

    let index = text.find(DOMAIN_NAME).ok_or(
        Error::Other(format!("Response '{text}' does not contain domain name: '{DOMAIN_NAME}'"))
    )?;

    let (_, text) = text.split_at_checked(index).ok_or(
        Error::Other(format!("Index {index} is out of bounds for '{text}'"))
    )?;

    let (_, text) = text.split_at_checked(DOMAIN_NAME.len() + 1).ok_or(
        Error::Other(format!("Index {index} is out of bounds for '{text}'"))
    )?;

    let index = text.find("/").ok_or(
        Error::Other(format!("Response '{text}' does not contain '/'"))
    )?;

    let (version, _) = text.split_at_checked(index).ok_or(
        Error::Other(format!("Index {index} is out of bounds for '{text}'"))
    )?;

    Ok(String::from(version))
}

pub fn output_tests(output: Output, cases: Vec<TestCase>) -> Result<(), Error> {
    match (output.stdout, output.output_file) {
        (true, _) => create_tests(cases, io::stdout()),
        (_, Some(path)) => {
            let file = File::create(path)?;
            create_tests(cases, file)
        },
        _ => Err(Error::Other("Output must have either 'output_file' or 'stdout' set".to_string()))
    }
}

fn create_tests<T: Write>(cases: Vec<TestCase>, out: T) -> Result<(), Error> {
    let mut out = BufWriter::new(out);
    for case in cases {
        out.write(case.into_test_function().as_bytes())?;
    }

    Ok(())
}
