<h1><picture>
  <source media="(prefers-color-scheme: dark)" srcset="/docs/img/logo_wh.png?raw=true">
  <source media="(prefers-color-scheme: light)" srcset="/docs/img/logo_bl.png?raw=true">
  <img align="left" alt="Test" src="/docs/img/logo_bl.png?raw=true" width="68px">
</picture> Sumi</h1>

A multi-user issue tracking system built with Yew frontend and actix-web backend.

This is a project for learning.  The app is in a working state, but it is relatively unstyled, does not yet display meaningful errors, and has little input verification.

## Features
- [x] Create, edit, close, re-assign tickets
- [x] Ticket list with sorting, filtering, and pagination
- [x] Enter notes for a ticket
- [x] Ticket event tracking (re-assignments, status changes, etc)
- [X] Nested-document style wiki
- [x] Markdown support for ticket descriptions, notes, and wiki
- [x] Editing conflict handling with revision history
- [x] Multi-user support
- [x] Local authentication with Argon2 hashing and salting
- [x] REST-style API
- [x] Dark/light theme
- [x] Localization support

## Future Features

### Down the pipe
These are some features I'm looking to add:
- [ ] Custom dashboard
- [ ] Projects
- [ ] Tasklists for tickets
- [ ] Show number of notes and tasks for a ticket in ticket list
- [ ] Show description preview for a ticket on ticket list
- [ ] Add On Hold status for tickets
- [ ] Time tracking
- [ ] Custom ticket tags
- [ ] Recycle bin for deleted items
- [ ] Contacts
- [ ] Access levels and disabled users
- [ ] Test coverage
- [ ] More logging and error handling

### Pipe-dream
These are some features I'd like to see but will not get to at this stage:
- [ ] E-mail integration (updates, submit via-e-mail)
- [ ] Scheduled tickets
- [ ] Issue submission portal
- [ ] Asset tracking
- [ ] Reporting
- [ ] Optional OIDC authentication
- [ ] Custom Fields

## Quickstart with Docker

1. Generate and place certificates in ./certificates/ folder (cert.pem & key.pem).  Sample command to generate self-signed cert:
```
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj /CN=localhost
```

2. Set your environment variables in .env (refer to .env.sample).

3. Build the Docker image
```
docker build -f Dockerfile -t vgwidt/sumi .
```

4. Modify docker-compose.yml as needed and run:
```
docker-compose up -d
```

## Setup Development Environment

### Install Dependencies

Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install diesel_cli dependencies:
* Linux
```
sudo apt-get install libpq-dev
```
* Windows
Install PostgreSQL (don't need to include DB server if not using that for the instance). Add the following environment variables:
```
PQ_LIB_DIR = C:\Program Files\PostgreSQL\14\lib
PATH = C:\Program Files\PostgreSQL\14\bin
```

If you tried to build or run using cargo before adding the environment variables, run cargo clean.  If in Windows and you get a Non-UTF-8 output error, install English language pack for VC build tools.

Windows: libintl-9.dll which ships with EDB is broken, get libintl-8.dll and put in debug folder (https://github.com/diesel-rs/diesel/discussions/2947)

Install diesel_cli
```
cargo install diesel_cli --no-default-features --features postgres
```

Install Docker (optional for quick PostgreSQL setup)
```
sudo apt -y install apt-transport-https ca-certificates curl gnupg2 software-properties-common
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io
sudo systemctl status docker
```

Install target for wasm
```
rustup target add wasm32-unknown-unknown
```

Install trunk (version 0.16.0 required)
```
cargo install --version 0.16.0 trunk
```

Other dependencies
```
cargo install --locked wasm-bindgen-cli
```

If running from Windows, you need to manually create the symlink.  Run the following from the backend folder:
```
mklink /D dist ..\frontend\dist
```

Finally, create .env file in project root.  Refer to .env.sample for required variables.

Note, in a clean Linux environment using Docker for Redis without reverse proxies where everything runs on the same server, localhost does not work.  Use 127.0.0.1 instead.

### Build

Run `./run.sh` (Linux, set to executable with `chmod +x ./run.sh`) or `.\run.bat` (Windows).  This will execute the database migration, use trunk to build the frontend, then run the backend with cargo.

## Getting Started

Default login is admin/password