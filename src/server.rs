#![allow(dead_code)]

#[path = "./schema/point_capnp.rs"]
mod point_capnp;

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
