extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};

struct Sutra {
    name: String,
    desc: Option<String>,
    n: Vec<Sutra>
}

impl Sutra {
    fn new<T> (name: T, desc: Option<T>, n: Vec<Sutra>) -> Sutra 
    where T: Into<String>
    {
        Sutra { name: name.into(), desc: desc.map(|s| s.into()), n }
    }
}

pub fn load_sutra_from_yaml() {
    println!("loading sutra")
}

fn parse_yaml_string(yaml_string: &str) -> Result<Vec<Sutra>, &'static str> {
    let yaml = YamlLoader::load_from_str(yaml_string).unwrap();

    Ok(vec!(parse_node(&yaml[0]["n"][0])))
}

fn read_str<'a>(yaml: Option<Yaml>) -> Option<String> {
    yaml.map(|y| match y {
        Yaml::String(str) => Some(str),
        _ => None
    }).flatten()
}

fn parse_node<'a> (hash: &'a Yaml) -> Sutra {
    print!("{:?}", hash);
    let name = hash["name"].as_str().unwrap();

    let desc = hash["desc"].as_str();
    let nodes = match hash["n"].as_vec() {
        Some(vec) => 
            vec
            .iter()
            .map(|n| parse_node(n))
            .collect(),
        None => 
            Vec::new()
    };

   Sutra::new(name, desc, nodes)
}

// Parse main element 
// find n:
// For each element add name: desc: (optional), add children optional


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_yaml_when_well_formatted() {
        let yaml = "n:
    - name: How to Use Mind Sutras
      desc: 'This is about how you can use the mind sutras project
      to create cool mind maps.'
      n:
        - name: Format
          desc: You write your document '.msutras' in yaml format.
        - name: Save
          desc: Saving will update the rendering of your document.
";
        let sut = parse_yaml_string(&yaml).unwrap();

        assert_eq!(sut.len(), 1);
        assert_eq!(sut[0].n.len(), 2);
    }

}
