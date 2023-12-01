use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use reqwest::{cookie::Jar, Client, ClientBuilder, Url};
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct AocInputs {
    cache_dir: PathBuf,
    client: Client,
}

impl AocInputs {
    pub fn new(cache_dir: PathBuf, session: String) -> Result<Self> {
        let cookie = format!("session={}", session);
        let url = ("https://adventofcode.com").parse::<Url>()?;
        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &url);

        let client = ClientBuilder::new()
            .cookie_provider(Arc::new(jar))
            .build()?;

        Ok(Self { cache_dir, client })
    }

    fn cache_file(&self, day: u8) -> PathBuf {
        self.cache_dir.join(format!("day_{}.txt", day))
    }

    async fn write_cache(&self, day: u8, content: String) -> Result<()> {
        create_dir_all(&self.cache_dir).await?;
        let mut file = File::create(self.cache_file(day)).await?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }

    async fn read_cache(&self, day: u8) -> Result<String> {
        let mut file = File::open(self.cache_file(day)).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        Ok(content)
    }

    async fn fetch_input(&self, day: u8) -> Result<String> {
        let url = format!("https://adventofcode.com/2023/day/{}/input", day);
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            let input = response.text().await?;
            self.write_cache(day, input.clone()).await?;
            Ok(input)
        } else {
            println!("{}", response.text().await?);
            Err(anyhow!("Failed to fetch input from advent of code"))
        }
    }

    pub async fn get_input(&self, day: u8) -> Result<String> {
        match self.read_cache(day).await {
            Err(_) => {
                println!("Cache file not found, fetching input from advent of code");
                let input = self.fetch_input(day).await?;
                if self.write_cache(day, input.clone()).await.is_err() {
                    println!("Failed to create cache file for day {day}");
                }
                Ok(input)
            }
            ok => ok,
        }
    }
}
