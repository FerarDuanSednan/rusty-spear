# laughing-journey

An app to create nickel.rs web project.

Not for serious use ( for the moment :) )

# TODO
- [x] Generate base project structure
- [x] Add a plugin support for assets ( kind of... )
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
export LATR=`pwd`/target/debug/latr
```

```
USAGE:
	latr [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message
    install    Install a plugin
    		latr install [OPTIONS] <PLUGINS_NAMES>
    		--plugins-path <PLUGINS_PATH>    Set the path to the plugins folder.
    new        Create a new project
    		latr new [OPTIONS] <NAME>
    		--skeleton <SKEL_PATH>    Set the path to the skeleton folder.
```

```
$ $LATR new --skeleton=$SKELETON_FOLDER project
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
