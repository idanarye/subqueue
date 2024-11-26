use std::io::{stdout, Write};

use clap::Parser;
use subqueue::scrape::PagedFetcher;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[derive(clap::Parser)]
enum Cli {
    Json { blog_url: String },
    Html { blog_url: String },
}

#[tokio::main(flavor = "current_thread")]
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
        Cli::Json { blog_url } => {
            tracing::info!("Dumping {blog_url}");
            let fetcher = subqueue::scrape::substack_posts::BlogPostFetcher::new(&blog_url)?;
            serde_json::to_writer(stdout().lock(), &fetcher.fetch_all().await?)?;
        }
        Cli::Html { blog_url } => {
            tracing::info!("Creating HTML for {blog_url}");
            let fetcher = subqueue::scrape::substack_posts::BlogPostFetcher::new(&blog_url)?;
            let mut out = stdout().lock();
            writeln!(out, "<html>")?;
            writeln!(
                out,
                "{}",
                r#"
                <head>
                <style>
                table, th, td {
                    border: 1px solid black;
                } 
                </style>
                </head>
                "#
            )?;
            writeln!(out, "<body>")?;
            writeln!(out, "<h1><a href={:?}>{}</a></h1>", blog_url, blog_url)?;
            writeln!(out, "<table>")?;
            writeln!(out, "<tr>")?;
            writeln!(out, "<th>Post</th>")?;
            writeln!(out, "<th>Date</th>")?;
            writeln!(out, "</tr>")?;
            for blog_entry in fetcher.fetch_all().await? {
                writeln!(out, "<tr>")?;
                writeln!(
                    out,
                    "<td><a href={:?}>{}</a></td>",
                    blog_entry.canonical_url, blog_entry.title
                )?;
                writeln!(out, "<td>{}</td>", blog_entry.post_date)?;
                writeln!(out, "</tr>")?;
            }
            writeln!(out, "</table>")?;
            writeln!(out, "</body>")?;
            writeln!(out, "</html>")?;
        }
    }
    Ok(())
}
