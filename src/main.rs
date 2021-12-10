use std::{io::Write, sync::Arc};

use anyhow::anyhow;
use reqwest::{
    cookie::{CookieStore, Jar},
    Url,
};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    Auth {},
}

const HOST: &str = "https://adventofcode.com";

fn cache_filepath() -> Result<String, anyhow::Error> {
    let home = std::env::var("HOME")?;
    Ok(format!("{}/.cache/aoc_cookie.json", home))
}

fn load_cookie() -> Result<reqwest::cookie::Jar, anyhow::Error> {
    let path = cache_filepath()?;
    let cookie = std::fs::read_to_string(&path)?;
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &Url::parse(HOST)?);
    Ok(jar)
}

fn save_cookie(jar: &Jar) -> Result<(), anyhow::Error> {
    let path = cache_filepath()?;
    let cookie = jar
        .cookies(&Url::parse(HOST)?)
        .ok_or_else(|| anyhow!("no cookie"))?;
    std::fs::write(path, cookie.as_bytes())?;
    Ok(())
}

async fn client() -> anyhow::Result<reqwest::Client> {
    let jar = auth().await?;
    let client = reqwest::Client::builder().cookie_provider(jar).build()?;
    Ok(client)
}

async fn auth() -> Result<Arc<Jar>, anyhow::Error> {
    if let Ok(jar) = load_cookie() {
        return Ok(Arc::new(jar));
    }

    let mut form = std::collections::HashMap::<_, _>::from_iter([]);

    let jar = Arc::new(reqwest::cookie::Jar::default());
    let client = reqwest::Client::builder()
        .cookie_provider(jar.clone())
        .build()?;

    let resp = client.get(&format!("{}/auth/github", HOST)).send().await?;

    for (k, v) in resp.url().query_pairs() {
        match k.as_ref() {
            "client_id" => form.insert("client_id", v.to_string()),
            "return_to" => form.insert("return_to", v.to_string()),
            _ => None,
        };
    }

    let text = resp.text().await?;
    let html = scraper::Html::parse_document(&text);
    for x in ["authenticity_token", "timestamp", "timestamp_secret"] {
        let sel = format!(r#"input[name="{}"]"#, x);
        let selector = scraper::Selector::parse(&sel).unwrap();
        let value = html
            .select(&selector)
            .next()
            .ok_or_else(|| anyhow!("{} not found", x))?
            .value()
            .attr("value")
            .unwrap();
        form.insert(x, value.to_string());
    }

    eprint!("login: ");
    std::io::stdout().flush()?;
    let mut login = String::new();
    std::io::stdin().read_line(&mut login)?;
    let login = login.trim().to_owned();
    let password = rpassword::prompt_password_stderr("password: ")?;

    form.insert("login", login);
    form.insert("password", password);

    let resp = client
        .post("https://github.com/session")
        .form(&form)
        .send()
        .await?;

    let html = scraper::Html::parse_document(&resp.text().await?);
    let selector = scraper::Selector::parse(r#"meta[http-equiv="refresh"]"#).unwrap();
    let url = html
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("data-url")
        .unwrap();

    client.get(url).send().await?;

    save_cookie(&jar)?;

    Ok(jar)
}

#[actix_rt::main]
async fn main() -> Result<(), anyhow::Error> {
    let resp = client()
        .await?
        .get("https://adventofcode.com/2021/day/1/input")
        .send()
        .await?;
    print!("{}", resp.text().await?);

    Ok(())
}
