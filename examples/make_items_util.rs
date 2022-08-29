use void_a_nomicon::error::GameError;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("need icon path, new out path too");
    }
    if let Err(e) = make_template(&args[1], &args[2], &args[3..]) {eprintln!("{}", e);};
}

fn make_template(icon_path: &str, out_path: &str, set_tags: &[String]) -> Result<(), GameError> {
    use std::fs;
    use std::io::prelude::*;
    let mut out_file = fs::OpenOptions::new().create(true).append(true).open(out_path)?;
    for file in fs::read_dir(icon_path)? {
        let dir = file?;
        if dir.metadata()?.is_dir() {continue;}
        let name = dir.file_name();
        let name = name.to_string_lossy().to_string();
        writeln!(&mut out_file, "{}", "{next}")?;
        writeln!(&mut out_file, "name: \"{}\"", &name[..name.len() - 4])?;
        writeln!(&mut out_file, "description: \"\"")?;
        writeln!(&mut out_file, "icon: \"icons/{}\"", dir.file_name().to_str().unwrap())?;
        write!(&mut out_file, "tags: [")?;
        for tag in set_tags.iter() {
            write!(&mut out_file, "\"{}\",", tag)?;
        }
        writeln!(&mut out_file, "]")?;
    }
    Ok(())
}