# rusthttpgunjs

## license: MIT

## Created By: Lightnet

## Notes:
 * This is just work space area folder.
 * gunrs (gunjs) work in progress.
 * Need to get each app to get http serve and websocket working.
 * Learning rust language

## Status:
 * Simple http and websocket (simple test)
 * Building Gunjs Database rust to gunrs library.

## Information:
  Just a prototype to develop gun database rust server for gunjs database setup. Testing different type of server package and ease of use and understand the rust language code.

## Porting from:
 * https://github.com/gundb/port
 
## Packages:
 * actix = "0.8.2"
 * actix-web = "1.0.0" ( main test?)

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

Notes:
 * Work in progress workspace.
 * File size vary if http and websocket different a bit.
 * Testing which build app is compact or friendly.
 * Files and Websocket has not build for gunjs yet.
 * Those are just simple test file size.

## Tests:
 * actix-web / gun (wip)
 * warp / gun (not tested)
 * hyper / gun (not tested)
 * warp / gun (not tested)
 * ws / gun (not tested)
 * tokio / gun (not tested)
 * tungstenite / gun (not tested)
 * rouille / gun (not tested)
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
 Make sure you have firewall, antvirus and others else it conflict with install and setup.

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
