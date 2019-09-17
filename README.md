# rusthttpgunjs

## license: MIT

## Created By: Lightnet

## Notes:
 * This is just work space area folder.
 * Note yet build gunrs.
 * Need to get each app to get http serve and websocket working.

## Status:
 * Learning rust language
 * Simple http and websocket (simple test)
 * Build Gun rust script.

## Information:
  Just a prototype test on rust server for gunjs database setup. Testing different type of server package and ease of use and understand the rust language code.

## Packages:
 * actix = "0.8.2"
 * actix-web = "1.0.0"

## run commands:
```
cargo run -p actixwebhttpgun
```
## Folders workspace: 

| Package     | workspace       | Server | Client | Lib | File Size |
| ---         | ---             | ---    | ---    | --- | ---       |
| actix-web   | actixwebgun     | x      | o      | o   | 12,163 KB |
| hyper       | hypergun        | x      | o      | o   | 5,836 KB  |
| tokio       | tokiogun        | x      | o      | o   | 3,948 KB  |
| warp        | warpgun         | x      | o      | o   | 8,179 KB  |
| http        | httpgunjs       | x      | o      | o   | 188 KB    |
| gun         | gunrs           | o      | o      | x   | 427 KB    |

Notes:
 * Work in progress workspace.
 * Testing which build app is compact or friendly.
 * Files and Websocket has not build for gunjs yet.
 * Those are just simple test file size.

# Window Linux subsystem:
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
```
workspace
```
//run app
cargo run -p actixhttpgun

//run lib test check for error
cargo test -p gunrs
```

## vscode:
 * https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/
 * https://code.visualstudio.com/docs/editor/variables-reference
