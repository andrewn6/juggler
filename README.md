# Juggler

Experimental load balancer tool, distributes incoming traffic accross multiple backend servers.

It is written in Rust using (Hyper)[https://hyper.rs] library.

## Install
*Make sure rust is installed before using Juggler!*

Clone the repository.
```
git clone https://github.com/anddddrew/juggler
```

Build the codebase
```
cargo build
```

Start the load balancing server
```
./start.sh
```

*Note: you may want to modify the start script above to your usecase*

## Roadmap

Currently this tool is very basic and only supports the `round robin` algorithm, I also want to make a CLI out of this so you can download it and use it locally on your system to test your application like this: `juggler -p 3000 -s http://localhost:8000 -s http://localhost:8001`

I also want to upport more load balancing algorithms, more specifically:
    - weighted round robin
    - least connection