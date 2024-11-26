use std::io::{stdout, Write};

use clap::Parser;
use subqueue::scrape::PagedFetcher;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[derive(clap::Parser)]
enum Cli {
    Dump { blog_url: String },
    Html { blog_url: String },
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::Level::WARN.into())
                .from_env()
                .unwrap(),
        )
        .init();

    match Cli::parse() {
        Cli::Dump { blog_url } => {
            tracing::info!("Dumping {blog_url}");
            let fetcher = subqueue::scrape::substack_posts::BlogPostFetcher::new(&blog_url)?;
            serde_json::to_writer(stdout().lock(), &fetcher.fetch_all().await?)?;
        }
        Cli::Html { blog_url } => {
            tracing::info!("Creating HTML for {blog_url}");
            let fetcher = subqueue::scrape::substack_posts::BlogPostFetcher::new(&blog_url)?;
            let mut out = stdout().lock();
            writeln!(out, "<html>")?;
            writeln!(out, "<body>")?;
            writeln!(out, "<ul>")?;
            for blog_entry in fetcher.fetch_all().await? {
                writeln!(out, "<li><a href={}>{}</a></li>", blog_entry.canonical_url, blog_entry.title)?;
            }
            writeln!(out, "</ul>")?;
            writeln!(out, "</body>")?;
            writeln!(out, "</html>")?;
        }
    }
    Ok(())
}
