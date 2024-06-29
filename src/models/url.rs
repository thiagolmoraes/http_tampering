use anyhow::{Result, anyhow};
use regex::Regex;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader, AsyncWriteExt};

pub struct Url {
    pub url: String,
}

impl Url {
    pub fn new<K>(url: K) -> Self
    where
        K: Into<String>,
    {
        Url {
            url: url.into(),
        }
    }

    pub async fn tampering(self) -> Result<String> {
        let client = Client::new();
        let url = self.url;

        match client.request(reqwest::Method::OPTIONS, &url).send().await {
            Ok(response) => {
                if let Some(allow_header) = response.headers().get("allow") {
                    let msg = format!("{} - Allow: {:?} - {}", response.status(), allow_header, url);
                    println!("{}", msg);
                    Ok(msg)
                } else {
                    let msg = format!("Allow header not found for {}", url);
                    println!("{}", msg);
                    Ok(msg)
                }
            },
            Err(e) => {          
                Err(anyhow!("Error sending request to {}: {}", url, e))
            }
        }
    
}

    pub async fn write_output(path: &str, output: Vec<String>) -> Result<()> {
        let mut file = File::create(path).await?;
    
        let content = output.join("\n");
    
        match file.write_all(content.as_bytes()).await {
            Ok(_) => println!("File written successfully!"),
            Err(e) => return Err(anyhow!("Error writing output: {}", e)),
        };
    
        Ok(())
    }

    pub async fn extract_url(file_path: &str) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        let file = File::open(file_path).await?;
        let reader = BufReader::new(file);

        let url_regex = Regex::new(r"https?://[^\s/$.?#].[^\s]*")?;

        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            for url in url_regex.find_iter(&line) {
                urls.push(url.as_str().to_string());
            }
        }

        Ok(urls)
    }
}