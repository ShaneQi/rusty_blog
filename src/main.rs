use pulldown_cmark::{html, Parser};
use yaml_rust2::YamlLoader;
use handlebars::{no_escape, Handlebars};
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use git2::{Repository, ResetType};

fn main() {
    // Get working paths.
    let mut input_path = env::args().nth(1).unwrap_or("./".to_string());
    let mut output_path = env::args().nth(2).unwrap_or("./".to_string());

    // Check if path has trailing slash.
    if input_path.chars().rev().nth(0).unwrap() != '/' {
        input_path += "/";
    }
    if output_path.chars().rev().nth(0).unwrap() != '/' {
        output_path += "/";
    }
    println!("Input path: {}", input_path);
    println!("Onput path: {}", output_path);

    // Config handlebars.
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(no_escape);
    handlebars
        .register_template_file("post", "./templates/post.hbs")
        .expect("Failed to register post template");
    handlebars
        .register_template_file("index", "./templates/index.hbs")
        .expect("Failed to register index template");
    handlebars
        .register_template_file("contact", "./templates/contact.hbs")
        .expect("Failed to register contact template");

    // If the input is a repo, hard reset it.
    fetch_reset_master_hard(&input_path);

    // Process md files.
    let mut posts: Vec<BTreeMap<String, String>> = vec![];
    for dir_item in Path::new(&format!("{}posts/", input_path))
        .read_dir()
        .expect(&format!(
            "Failed to find post directory: {}posts/",
            input_path
        ))
    {
        let path = dir_item.expect("").path();
        // Sidecar asset dirs are named like the post permalink; skip non-Markdown.
        if path.is_dir() || path.extension().and_then(|s| s.to_str()).unwrap_or("") != "md" {
            continue;
        }
        println!("Processing file: {:?}", path);
        let data = read_file(path);
        let post_html = handlebars.render("post", &data).expect("");
        let permalink = data["permalink"].as_str().to_string();
        let post_out_dir = format!("{}{}/", output_path, permalink);
        let _ = fs::create_dir_all(&post_out_dir);
        File::create(format!("{}index.html", post_out_dir))
            .and_then(|mut file| file.write_all(post_html.as_bytes()))
            .expect("");
        copy_post_sidecar(
            &format!("{}posts/{}", input_path, permalink),
            &post_out_dir,
        );
        posts.push(data);
    }
    posts.sort_by(|a, b| b["date"].as_str().cmp(a["date"].as_str()));

    // Generate homepage.
    let index_post_html = handlebars.render("index", &posts).expect("");
    File::create(&format!("{}index.html", output_path))
        .and_then(|mut file| file.write_all(&index_post_html.as_bytes()))
        .expect("");

    copy_assets(output_path);

}

fn copy_assets(output_path: String) {
    let _ = fs::create_dir_all(&format!("{}css/", output_path));
    let _ = fs::create_dir_all(&format!("{}script/", output_path));
    for dir_item in Path::new("./assets/css/").read_dir().expect("") {
        let path = dir_item.expect("").path();
        let file_name = path.file_name().and_then(|f| f.to_str()).expect("");
        println!("Copying asset css file: {:?}", path);
        fs::copy(path.clone(), &format!("{}css/{}", output_path, file_name)).expect("");
    }
    for dir_item in Path::new("./assets/script/").read_dir().expect("") {
        let path = dir_item.expect("").path();
        let file_name = path.file_name().and_then(|f| f.to_str()).expect("");
        println!("Copying asset script file: {:?}", path);
        fs::copy(path.clone(), &format!("{}script/{}", output_path, file_name)).expect("");
    }
}

/// Copy `content/posts/<permalink>/` into `output/<permalink>/` so Markdown can
/// reference images as `./hero.jpg` next to the generated `index.html`.
fn copy_post_sidecar(sidecar_path: &str, post_out_dir: &str) {
    let sidecar = Path::new(sidecar_path);
    if !sidecar.is_dir() {
        return;
    }
    copy_dir_contents(sidecar, Path::new(post_out_dir));
}

fn copy_dir_contents(src_dir: &Path, dest_dir: &Path) {
    for dir_item in src_dir.read_dir().expect("Failed to read post sidecar directory") {
        let path = dir_item.expect("").path();
        let file_name = path.file_name().and_then(|f| f.to_str()).expect("");
        // Never replace the generated post page with a sidecar file.
        if file_name == "index.html" {
            println!("Skipping sidecar index.html: {:?}", path);
            continue;
        }
        let dest = dest_dir.join(file_name);
        if path.is_dir() {
            fs::create_dir_all(&dest).expect("Failed to create sidecar output directory");
            copy_dir_contents(&path, &dest);
        } else {
            println!("Copying post asset: {:?} -> {:?}", path, dest);
            fs::copy(&path, &dest).expect("Failed to copy post sidecar asset");
        }
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> BTreeMap<String, String> {
    let file = File::open(path).expect("");
    let buf = BufReader::new(file);
    let mut yaml = String::new();
    let mut markdown = String::new();
    let mut yaml_began: Option<bool> = Option::None;
    for line in buf.lines() {
        let this_line = line.expect("");
        if this_line == "---" {
            match yaml_began {
                Some(true) => yaml_began = Some(false),
                _ => yaml_began = Some(true),
            }
        } else {
            match yaml_began {
                Some(true) => {
                    yaml = yaml + &this_line;
                    yaml = yaml + "\n";
                }
                Some(false) => {
                    markdown = markdown + &this_line;
                    markdown = markdown + "\n";
                }
                _ => (),
            }
        }
    }

    // Parse markdown to html.
    let mut content = String::new();
    let parser = Parser::new(&markdown);
    html::push_html(&mut content, parser);

    // Parse yaml to post info.
    let yamls = YamlLoader::load_from_str(&yaml).expect("");
    let yaml_map = &yamls[0];
    let mut data = BTreeMap::new();
    let title = yaml_map["title"].as_str().expect("").to_string();
    let date = yaml_map["date"].as_str().expect("");
    data.insert("date".to_string(), date.to_string());
    data.insert("title".to_string(), title.clone());
    data.insert(
        "permalink".to_string(),
        yaml_map["permalink"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or(
                title.to_lowercase().replace(" ", "-") + "-" +
                    &date.chars().skip(2).take(8).collect::<String>(),
            ),
    );
    data.insert("content".to_string(), content);

    return data;
}

fn fetch_reset_master_hard(repo_path: &str) {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        _ => return,
    };
    repo.find_remote("origin")
        .expect("The repo doesn't have origin remote.")
        .fetch(&["master"], None, None)
        .expect("Failed to fetch from origin.");
    let oid = repo.refname_to_id("refs/remotes/origin/master").expect(
        "The origin remote dones't have master branch.",
    );
    let object = repo.find_object(oid, None).expect(
        "Failed to find object from oid.",
    );
    repo.reset(&object, ResetType::Hard, None).expect(
        "Failed to hard reset current head to origin/master.",
    );
}