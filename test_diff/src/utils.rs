use crate::bitmap::Bitmap;
use crate::error::Error;
use crate::Arguments;
use roxmltree::Document;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn test_diff(arguments: Arguments) -> Result<(), Error> {
    let old = parse(arguments.old)?;
    let new = parse(arguments.new)?;

    let diff = old.xor(&new);
    let indices = diff.where_bit_is(true);

    let (pass, fail) = changes(&old, &indices);

    println!("Fail -> Pass:");

    for index in pass {
        println!("  #{}", index + 1)
    }

    println!();
    println!("Pass -> Fail:");

    for index in fail {
        println!("  #{}", index + 1)
    }

    Ok(())
}

fn parse<P: AsRef<Path>>(path: P) -> Result<Bitmap, Error> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut content = String::new();
    let _ = reader.read_to_string(&mut content)?;

    let document = Document::parse(&content)?;
    Ok(results(document)?)
}

fn results(document: Document) -> Result<Bitmap, Error> {
    let mut bitmap = Bitmap::new();

    document.descendants()
        .filter(|node| node.has_tag_name("test"))
        .map(|node| node.attribute("status") == Some("passed"))
        .for_each(|passed| bitmap.push(passed));

    Ok(bitmap)
}

fn changes(old: &Bitmap, indices: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut new_pass = Vec::new();
    let mut new_fail = Vec::new();

    for index in indices {
        if old.get(*index).unwrap_or(false) {
            new_fail.push(*index);
        } else {
            new_pass.push(*index);
        }
    }

    (new_pass, new_fail)
}
