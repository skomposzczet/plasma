# plasma
Plasma is Another Secure Messaging App
# About 
E2EE mechanism is designed according to the [X3DH Key Agreement Protocol (also known as signal protocol)](https://www.signal.org/docs/specifications/x3dh/).

Key exchange is implemented using `p256` library which implements NIST P-256 - asymmetric cryptography technique . It supports ECDH and ECDSA algorithms. It is used for generating keys, signing, verifying signatures and Diffie-Hellman key exchange.

ChaCha20-Poly1305 is chosen encrypting algorithm. It combines ChaCha20 stream cipher with the Poly1305 message authentication code.  It is selected for its [security and speed](https://security.googleblog.com/2014/04/speeding-up-and-strengthening-https.html).
# Implementation
## X3DH library
### Keys
Defines public and private keys structures:
- identity key,
- ephemeral key,
- signed prekey,
- one-time prekey.

Library also provides definitions of signature and shared secret.
### Key exchange
There are two functions implementing X3DH protocol:
- `x3dh_sig`,
- `xd3h`.

Prior is used by party initializing exchange (Alice) - it also verifies signature. Latter is used by the other party (Bob).
### Other
Library also defines helper structs representing common bundles of keys etc.:
- `RegisterBundle`,
- `PeerBundle`,
- `InitialMessage`.

Mentioned structs can be serialized and deserialized.
## Backend
### Database
Server app has `model` module that sets up connection with MongoDB. In addition it is divided in submodules that map BSON documents to Rust native structures and vice versa:
 - `chat` - for chat related documents,
 - `keys` - for X3DH protocol related documents,
 - `user` - for user related documents.

### Security
Authentication is achived with JWT.
### API
App serves REST API and allows for WebSocket connection. Both are implemented with `warp` framework. Every endpoint besides login and register require users authentication with JWT.
## Client app
### API
App provides `api` module which is responsible for communicating with server app via REST API. It is built on `reqwest` library.

Same module also implements connection with backend via WebSocket connection which is implemented with `tokio_tungstenite` and `http` libraries.
### User and their data
`Account` struct is abstraction of user which is either authenticated or not. In first case struct provides methods fetching data related to user e.g. keys, chat list, etc. Data is retrieved either from local storage or by querying backend.

It also has methods generating shared key. They make use of X3DH  lib interface.

Some data is stored locally. It is responsibility of `keyring` module which saves keys, shared secrets and token in `$HOME/.plasma`.
### Cipher
`Cipher` structure is initialized with shared secret. It then allows for encrypting and decrypting messages with `encrypt` and `decrypt` functions respectively. Functions additionally require message timestamp to use it as nonce. Struct implements cipher with `chacha20poly1305` library. 
### UI
UI is terminal based and is implemented with `Ratatui` framework in `tui:ui` module. `App` struct defined in `app` module aggregates data to display in UI, server communication handles and logic providing objects. It handles events, and performs encrypting (of new outgoing messages) and decrypting (of chat history and new incoming messages)  with use of `cipher` module.
### CLI
CLI allowing to register and login to app is implemented in main module with `Clap` library. If login credentials (or token authentication) are correct main UI drawing and event handling loop is started.
# User Interface
### Login/Register
App provides `help` command to display usage information, and `login` and `register` command used to login to existing account and create new account respectively. Command can be run with `--help` flag to display further information. In both cases user will be asked to securely provide password unless valid token is detected in login case.
### Interface
TUI consists of four panes: chat list, user search input, chat history and new message input. App has five modes allowing to navigate:
- normal - to switch between modes and quit app,
- browse - to browse existing chat,
- new chat - to find user,
- scroll - to navigate chat history pane,
- new message - to type new message.

Type `Esc` to select normal mode and `q` to quit app when in normal mode. Use `bnsm` to select remaining modes respectively. Use `hj` or `Enter` to navigate in browse and  `hjklq` or `Enter` to navigate in scroll mode.
### Demo
\<demo\>
# Deploying
Backend can be deployed with supplied `docker-compose.yaml`:
```bash
docker compose up
```
Server docker image is automatically built with use of `Dockerfile`. Server app will connect to created mongo container, which will persist data in `data/db`. To correctly connect `.env.docker` must be filled.

Client app can be built with cargo:
```bash
cargo run 
```
