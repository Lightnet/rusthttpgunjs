# rusthttpgunjs

## License: MIT

## Created By: Lightnet

## Status:
 * Simple http and websocket (simple test)
 * Building Gunjs Database rust to gunrs library.
 * Learning rust language

## Information:
  Just a prototype to develop gun database rust server for gunjs database setup. Testing different type of server package and ease of use and understand the rust language code.

## Porting from:
 * https://github.com/gundb/port
 
## Packages:
 * actix-web = "1.0.0" ( main test area)
  
## Setup | Build:
  Note you need to install rust lanuguage.

 * actixwebgun (project / main)

```
cargo run -p actixhttpgun
```

 The project folder main test actixwebgun. Since it easy to develop test file.
### Features:
 * Server
   * Get
   * Put
   * Dup Checks
   * Hem Checks
 * Client
   * Send Test



## Folders workspace: 

| Package     | Crates  | workspace       | Server | Client | Lib | File Size | Status | gunrs  | json  |
| ---         |---      | ---             | ---    | ---    | --- | ---       | ---    | ---    | ---   |
| actix-web   | 247     | actixwebgun     | x      | o      | o   | 12,163 KB | wip    | x      | x     |
| rocket      |  83     | rocketgun       | x      | o      | o   | 8,886 KB  | ???    | o      | o     |
| warp        | 198     | warpgun         | x      | o      | o   | 8,179 KB  | ???    | o      | o     |
| hyper       | 132     | hypergun        | x      | o      | o   | 5,836 KB  | ???    | o      | o     |
| tungstenite | 79      | tungstenitegun  | x      | o      | o   | 4,015 KB  | ???    | o      | o     |
| tokio       | 118     | tokiogun        | x      | o      | o   | 3,948 KB  | ???    | o      | o     |
| rouille     | 114     | rouillegun      | x      | o      | o   | 2,641 KB  | ???    | o      | o     |
| websocket   | 100     | websocketgun    | x      | o      | o   | 2,191 KB  | ???    | o      | o     |
| ws          | 81      | wsgun           | x      | o      | o   | 1,580 KB  | ???    | o      | o     |
| TcpListener |  1      | httpgunjs       | x      | o      | o   | 188 KB    | ???    | o      | o     |
| gunrs       |  1      | gunrs           | o      | o      | x   | 427 KB    | wip    | o      | o     |

 * Those are just simple test file size.

## Tests:
 * actix-web / gun (tested / wip)
 * warp / gun (partly tested)
 * hyper / gun (partly tested)
 * warp / gun (partly tested)
 * ws / gun (partly tested)
 * tokio / gun (partly tested)
 * tungstenite / gun (partly tested)
 * rouille / gun (partly tested)
 * http / gun (raw with websocket/ws package???)
 * gun (wip)

## command line:
```
cargo run -p actixwebgun //server host web 
cargo run -p rouillegun //server host web 
cargo run -p httpgunjs //server host web simple page web
cargo run -p datagun //test gunrs
```

## Window Linux subsystem:
 Make sure you have firewall, antvirus and others disable else it conflict with install and setup.

```
 Windows 10
  * window app store
    * ubuntu linux os for bash
      * setup account for linux
```

```bash
 sudo apt-get update  //update packages
 sudo apt-get gcc     //install compiler
```

```cmd
cargo run // default run app

cargo clean //clean up build app

cargo build // build app

rustup default nightly

//rustup default stable
```
workspace
```
//run app
cargo run -p actixhttpgun //not yet added gunrs

//run lib test check for error
cargo test -p gunrs
```

## vscode:
 * https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/
 * https://code.visualstudio.com/docs/editor/variables-reference

## Notes:
 * This is work space area folder.
 * File size projects may vary if http, websocket and other packages to support web.
 * gunrs (gunjs) work in progress.
 * Testing which build app is compact or friendly.
 * Files and Websocket has not build for gunjs yet.