use ballista::prelude::{BallistaConfig, BallistaContext, Result};
use datafusion::prelude::CsvReadOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let config = BallistaConfig::builder()
        .set("ballista.shuffle.partitions", "1")
        .build()?;

    let ctx = BallistaContext::standalone(&config, 2).await?;

    // register csv file with the execution context
    ctx.register_csv(
        "test",
        &format!("data/aggregate_test_100.csv"),
        CsvReadOptions::new(),
    )
    .await?;

    let df = ctx.sql("select count(1) from test").await?;

    df.show().await?;
    Ok(())
}
