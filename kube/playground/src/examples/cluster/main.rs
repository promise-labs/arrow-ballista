use ballista::prelude::{BallistaConfig, BallistaContext, Result};

/// This example demonstrates how to connect to a remote Ballista scheduler and execute a query.
/// The query starts by instantiating an external table, with the data stored in S3.
/// The query then selects all rows from the external table and displays them.
/// 
/// The example assumes that the code is being executed on a client from within the cluster, hence
/// the use of the "ballista-scheduler.default" local cluster DNS name to connect to the scheduler.
#[tokio::main]
async fn main() -> Result<()> {
    let config = BallistaConfig::builder()
        .set("ballista.shuffle.partitions", "1")
        .build()?;

    let ctx = BallistaContext::remote("localhost", 50050, &config).await.unwrap();

    let _ = ctx.sql("
            create external table un_pop_data 
            stored as csv 
            with header row
            location 's3://sdfdatasets/hello-world/world_population_full.csv';
    ").await?;

    let new_df = ctx.sql("select * from un_pop_data").await?;

    new_df.show().await.unwrap();

    Ok(())
}

