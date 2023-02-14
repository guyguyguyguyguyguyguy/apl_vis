use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use serde_json::{Value};

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
fn handle_client(mut stream: &TcpStream, cap: usize) -> Vec<String> {
    let buf_reader = BufReader::with_capacity(cap, stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    http_request
}

// fn get_texture(ns: String) -> impl Fn(Vec<usize>) -> ndarray::Array3<u8> {
fn get_texture(shape: &Vec<usize>) -> impl Fn(String) -> ndarray::Array3<u8> + '_ {
    move |ar: String| {
        let [x, y, z] = shape[..] else {panic!("Not a 3D array")};
        let arr = ar.replace("[", "").replace("]", "");
        println!("{:?}", arr.split(","));
        let flat_vec: Vec<u8> = arr.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
        println!("{:?}", flat_vec);
        let arr = ndarray::Array::from_shape_vec((z, y, x), flat_vec).expect("cannot create ndarray");
        arr
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7342").unwrap();

    let (mut client, _) = listener.accept().expect("couldn't accept");
    // client.set_nonblocking(true);

    let mut s_buf = [0; 7];
    let _ = client.read(&mut s_buf);

    let shape: Vec<usize> = String::from_utf8_lossy(&s_buf)
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let arr_size = shape
        .clone()
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{}", arr_size);

    // let arr_size: usize = sizes.into_iter().reduce(|a, b| a * b).unwrap();
    // println!("{}", arr_size);

    let mut buf = vec![0; arr_size * 2 + 1];
    _ = client.read(&mut buf);

    let v: Vec<u8> = serde_json::from_slice(&buf)?;
    // let va = v
    println!("{:?}", v);

    // let init_tex = get_texture(&shape);
    // let tex = init_tex(String::from_utf8_lossy(&buf).to_string());
    // println!("{:?}", tex);
    Ok(())
}
