#![feature(test)]

extern crate futures;
extern crate histogram;
extern crate rand;
extern crate test;
extern crate tokio_core;
extern crate tokio_io;

#[cfg(test)]
mod tests {
    use futures::*;
    use tokio_core::reactor::Core;
    use tokio_core::net::TcpStream;
    use tokio_io::io::{read_until, write_all};
    use histogram::Histogram;
    use rand;
    use std::io;
    use std::time::Instant;
    use std::sync::{Arc, Mutex};
    use test::Bencher;

    const NUM: usize = 1000;
    const CONCURRENT: usize = 8;

    #[bench]
    fn one_thread_latency(b: &mut Bencher) {
        let addr = "127.0.0.1:12345".parse().unwrap();
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        b.iter(move || {
            let future = TcpStream::connect(&addr, &handle)
                .and_then(|sock| write_all(sock, "GET key \r\n".as_bytes()))
                .and_then(|(sock, _)| {
                    read_until(io::BufReader::new(sock), b'\n', vec![])
                })
                .map(|(_, response)| {
                    assert_eq!(String::from_utf8(response).unwrap(), "None\n")
                });

            core.run(future)
        });
    }

    #[test]
    fn one_thread_reads() {
        let addr = "127.0.0.1:12345".parse().unwrap();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let metrics = Arc::new(Mutex::new(Histogram::new()));
        let test_start = Instant::now();

        let connects = stream::iter((0..NUM).map(|_| {
            let metrics = metrics.clone();
            let start = Instant::now();
            Ok(
                TcpStream::connect(&addr, &handle)
                    .and_then(move |sock| write_all(sock, "GET key \r\n".as_bytes()))
                    .and_then(move |(sock, _)| {
                        read_until(io::BufReader::new(sock), b'\n', vec![])
                    })
                    .map(move |(_, response)| {
                        let end = Instant::now();
                        let duration = end.duration_since(start).subsec_nanos() as u64;
                        metrics.lock().unwrap().increment(duration).unwrap();
                        assert_eq!(String::from_utf8(response).unwrap(), "None\n")
                    }),
            )
        }));

        core.run(
            connects
                .buffer_unordered(CONCURRENT)
                .map_err(|e| panic!("client err: {:?}", e))
                .for_each(|_| Ok(())),
        ).unwrap();

        let test_end = Instant::now();
        let test_duration = test_end.duration_since(test_start).subsec_nanos();
        println!("TEST DURATION {}s", test_duration as f32 / 1_000_000_000f32);
        let metrics = metrics.lock().unwrap();
        println!(
            "PERCENTILES 95th: {:?}, 99th: {:?}",
            metrics.percentile(95.0).unwrap() as f32 / 1_000_000f32,
            metrics.percentile(99.0).unwrap() as f32 / 1_000_000f32
        );

        println!(
            "AVG REQ/s = {}",
            (NUM as f32) / (test_duration as f32 / 1_000_000_000f32)
        );
    }

    #[test]
    fn one_thread_writes() {
        let addr = "127.0.0.1:12345".parse().unwrap();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let metrics = Arc::new(Mutex::new(Histogram::new()));
        let test_start = Instant::now();

        let connects = stream::iter((0..NUM).map(|_| {
            let metrics = metrics.clone();
            let start = Instant::now();
            Ok(
                TcpStream::connect(&addr, &handle)
                    .and_then(move |sock| {
                        let key = rand::random::<f64>();
                        write_all(sock, format!("PUT {} set\r\n", key).into_bytes())
                    })
                    .and_then(move |(sock, _)| {
                        read_until(io::BufReader::new(sock), b'\n', vec![])
                    })
                    .map(move |(_, response)| {
                        let end = Instant::now();
                        let duration = end.duration_since(start).subsec_nanos() as u64;
                        metrics.lock().unwrap().increment(duration).unwrap();
                        assert_eq!(String::from_utf8(response).unwrap(), "None\n")
                    }),
            )
        }));

        core.run(
            connects
                .buffer_unordered(CONCURRENT)
                .map_err(|e| panic!("client err: {:?}", e))
                .for_each(|_| Ok(())),
        ).unwrap();

        let test_end = Instant::now();
        let test_duration = test_end.duration_since(test_start).subsec_nanos();
        println!("TEST DURATION {}s", test_duration as f32 / 1_000_000_000f32);
        let metrics = metrics.lock().unwrap();
        println!(
            "PERCENTILES 95th: {:?}, 99th: {:?}",
            metrics.percentile(95.0).unwrap() as f32 / 1_000_000f32,
            metrics.percentile(99.0).unwrap() as f32 / 1_000_000f32
        );

        println!(
            "AVG REQ/s = {}",
            (NUM as f32) / (test_duration as f32 / 1_000_000_000f32)
        );
    }

}
