#![allow(dead_code)]

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};

use futures::{AsyncReadExt, FutureExt};
use std::net::ToSocketAddrs;

use crate::server::point_capnp::{point, point_tracker};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} client HOST:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&addr).await?;

            println!("Connected to TCP Stream");

            stream.set_nodelay(true)?;
            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

            // RPC code
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));
            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let point_tracker: point_tracker::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

            let mut request = point_tracker.add_point_request();

            // let's make a Point:
            let mut message = ::capnp::message::Builder::new_default();
            let mut new_point = message.init_root::<point::Builder>();
            new_point.set_x(5_f32);
            new_point.set_y(10_f32);

            request.get().set_p(new_point.into_reader())?;

            let reply = request.send().promise.await.unwrap();

            println!(
                "Total points in Point Tracker: {}",
                reply.get().unwrap().get_total_points()
            );

            Ok(())
        })
        .await
}
