# laughing-journey

An app to create nickel.rs web project.

Not for serious use ( for the moment :) )

```sh
git clone https://github.com/FerarDuanSednan/laughing-journey.git
cd laughing-journey
cargo build
```

```
USAGE:
	latr [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message
    new     Create a new project
```

```
$ ./target/debug/latr new project
$ tree project
project
├── assets
│   ├── images
│   ├── javascripts
│   └── stylesheets
├── Cargo.toml
├── src
│   ├── config
│   │   ├── mod.rs
│   │   └── routes.rs
│   ├── controllers
│   │   ├── mod.rs
│   │   └── web_controller.rs
│   ├── helpers
│   ├── lib.rs
│   ├── main.rs
│   └── models
└── views
```
