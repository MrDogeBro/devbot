use anyhow::Result;
use tokio::{fs::File, io::AsyncReadExt};
use tokio_postgres::{connect, Client, NoTls};

pub struct Database {
    pub client: Client,
}

impl Database {
    pub async fn load(config: &crate::config::Config) -> Result<Self> {
        let (client, conn) = connect(
            format!(
                "host={} port={} dbname={} user={} password={}",
                config.env.db_host,
                config.env.db_port,
                config.env.db_name,
                config.env.db_user,
                config.env.db_pass
            )
            .as_str(),
            NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Error connecting to db: {}", e);
            }
        });

        let mut build_sql: String = String::new();
        File::open(format!("{}/build.sql", &config.data_path.staticd))
            .await?
            .read_to_string(&mut build_sql)
            .await?;

        client.batch_execute(build_sql.as_str()).await?;

        Ok(Self { client })
    }
}
