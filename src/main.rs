use tokio_postgres::{NoTls, Error, Client};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=123", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    match select_address(client).await {
        Err(msg) => eprintln!("{}", msg),
        _ => {},
    }

    Ok(())
}

async fn select_address(client: Client) -> Result<(), Error> {
    let rows = client
        .query("SELECT street from address", &[])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    println!("Street is: {}", value);

    Ok(())
}