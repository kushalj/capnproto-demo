#![allow(dead_code)]

use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};

use futures::{AsyncReadExt, FutureExt};
use std::net::ToSocketAddrs;

#[path = "./schema/point_capnp.rs"]
pub mod point_capnp;
use point_capnp::point_tracker;

pub mod point_demo {
    use crate::server::point_capnp::point;
    use capnp::serialize;
    use std::fs::File;

    pub fn write_to_stream() -> std::io::Result<()> {
        let mut message = ::capnp::message::Builder::new_default();

        let mut demo_point = message.init_root::<point::Builder>();

        demo_point.set_x(5_f32);
        demo_point.set_y(10_f32);

        // This Result should be consumed properly in an actual app
        let _ = serialize::write_message(&mut ::std::io::stdout(), &message);

        // Save the point
        {
            let file = File::create("point.txt")?;
            let _ = serialize::write_message(file, &message);
        }

        // Read the point from file
        {
            let point_file = File::open("point.txt")?;

            // We want this to panic in our demo incase there is an issue
            let point_reader =
                serialize::read_message(point_file, ::capnp::message::ReaderOptions::new())
                    .unwrap();

            let demo_point: point::Reader = point_reader.get_root().unwrap();
            println!("\n(x = {}, y = {})", demo_point.get_x(), demo_point.get_y());
        }

        Ok(())
    }
}

pub struct Point {
    x: f32,
    y: f32,
}

struct PointTrackerImpl {
    points: Vec<Point>,
}

impl point_tracker::Server for PointTrackerImpl {
    fn add_point(
        &mut self,
        params: point_tracker::AddPointParams,
        mut results: point_tracker::AddPointResults,
    ) -> Promise<(), ::capnp::Error> {
        let point_client = pry!(params.get()).get_p();

        if let Ok(received_point) = point_client {
            self.points.push(Point {
                x: received_point.get_x(),
                y: received_point.get_y(),
            });
        }
        results.get().set_total_points(self.points.len() as u64);

        Promise::ok(())
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;

            // Cap'n Proto point_tracker client initialised here
            let point_tracker_client: point_tracker::Client =
                capnp_rpc::new_client(PointTrackerImpl { points: Vec::new() });

            println!("Server running");
            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system =
                    RpcSystem::new(Box::new(network), Some(point_tracker_client.clone().client));

                tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
            }
        })
        .await
}
