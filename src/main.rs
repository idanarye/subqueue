use clap::Parser;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[derive(clap::Parser)]
enum Cli {
    Dump { blog_url: String },
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
            println!("{:#?}", fetcher.fetch(10, 0).await?);
        }
    }
    Ok(())
}
