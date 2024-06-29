mod models;
use models::url::Url;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "url_processor",
    about = "A tool to process URLs from a file and apply tampering.",
    long_about = "This tool reads a file containing URLs, applies tampering operations, and writes the results to an output file."
)]
struct Args {
    #[arg(short, long, help = "Specify the input file containing URLs to process")]
    input: String,

    #[arg(short, long, help = "Specify the output file to write the results (optional)")]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let urls = Url::extract_url(&args.input).await?;

    let mut vec_url: Vec<String> = Vec::new();

    for url in urls {
        let new_url = Url::new(url);
        let content = new_url.tampering().await?;
        vec_url.push(content);
    }
    
    if let Some(output) = args.output {
        Url::write_output(&output, vec_url).await?;
    } else {
        println!("{:?}", vec_url);
    }

    Ok(())
}
