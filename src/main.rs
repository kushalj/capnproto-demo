pub mod client;
pub mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "point_demo" => {
                return {
                    let _ = server::point_demo::write_to_stream();
                    Ok(())
                }
            }
            "client" => return client::main().await,
            "server" => return server::main().await,
            _ => (),
        }
    }

    println!("usage: {} [client | server] ADDRESS", args[0]);
    Ok(())
}
