# gRPC Hello World Example

So this is where we are starting out journey to dive deeper into gRPC, protobuffers and http2.

## Steps you can follow

- First create a cargo project:
```cargo new <project_name>```

- Create a `.proto` file for defining service and message types
```
mkdir proto && cd proto
touch hello.proto
```

- Now define syntax version, package name, services and messages inside this file as you'd like (You can use the proto file in this repository for example)

- Add the dependencies to the project which will be required
```
cargo add tonic prost
cargo add tokio -F rt-multi-thread
cargo add --build tonic-build
```

- After adding the dependencies we can write the build.rs file which will compile the proto file and we can then access the generated rust code within our project

- Since we will be needing both a server and a client, we can define these as binaries in our Cargo.toml file using [[bin]] decorator in Cargo.toml file and defining the name of the binaries and their paths.

```
[[bin]]
name = "client"
path = "src/client.rs"

[[bin]]
name = "server"
path = "src/server.rs"
```

- Write the programs for the server and client, but the most important thing to keep in mind is to import the rust code that was generated after we `cargo b` the project
```
pub mod hello {
  tonic::include_proto!("hello");
}
```

- After writing the program for server and client, you can run the server in one terminal with
```
cargo r --bin server
```
- And you can now make request to this server with the client binary, which you can run from a separate terminal with
```
cargo r --bin client
```

