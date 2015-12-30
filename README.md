# laughing-journey

An app to create nickel.rs web project.

Not for serious use ( for the moment :) )

# TODO
- [x] Generate base project structure
- [ ] Improve default landing page
- [ ] Add support for zurb / bootstrap
- [ ] Allow the use of other template engines than mustache
- [ ] Add generator for controllers/views/...
- [ ] Clean the code ( yes, no pattern or whatever for now ... )
- [ ] err... find a better name ?

# Usage

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
    		--skeleton=[path] Set path to skeleton folder
```

```
$ $LATR_PATH/latr new --skeleton=$SKELETON_FOLDER project
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
