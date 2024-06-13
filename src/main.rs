use byml::{Byml, Endian};

fn main() {
    // Get Input
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("Usage: byml-rs <path>");
    }
    // Convert
    convert(&args[1]);
}

fn convert(full_path: &str) {
    // Split path
    let path = full_path.rsplit_once('.').unwrap();
    if path.1 == "byml" {
        // byml to yml
        let file = std::fs::read(full_path).unwrap();
        let binary = Byml::from_binary(&file).unwrap();
        std::fs::write(format!("{}.yml", path.0), binary.to_text().unwrap()).unwrap();
    } else if path.1 == "yml" {
        // yml to byml (v4)
        let file = std::fs::read_to_string(full_path).unwrap();
        let text = Byml::from_text(&file).unwrap();
        std::fs::write(
            format!("{}.byml", path.0),
            text.to_binary(Endian::Little, 4).unwrap(),
        )
        .unwrap();
    }
}
