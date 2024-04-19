use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Sratring up..");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("Something went wrong!!");

    hello().await;

    Ok(())
}

#[tracing::instrument]
async fn hello() {
    println!("Hello there!");
}
