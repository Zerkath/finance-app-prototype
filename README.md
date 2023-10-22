# Finance App Prototype

## Technology

The project is powered by [Tauri](https://tauri.app/) + [Svelte](https://svelte.dev/).
For storing users information Sqlite is being used by the backend code in src-tauri.
To interface with sqlite, rusqlite is being used.

## Getting started

To start the project in development mode: `npm run tauri dev`

### Testing

#### Backend

To test the rust code, there are some premade npm commands:

```bash
# there is a simple command that will cd the shell into src-tauri
# and run a cargo command
npm run cargo <any args>

# more advanced command that will run tests for the rust project
# and create a report to ./src-tauri/target/coverage/tarpaulin-report.html
npm run test:backend

# to test frontend
npm run test:frontend

# to test both
npm run test
```

### Migrations

The database is being automatically initialized on startup.
Currently there is no mechanism for migrations, if needed could be added with [rust-db/refinery](https://github.com/rust-db/refinery)
The initial prototype version of the application exposes a endpoint to remove the current database tables and reintialize them.

## Building

Simply running `npm run tauri build` will create a binary for the current platform.

For now, there is no automated step for creating a binary for each major platform (linux, windows, macOS).
The build has been tested on the following platforms manually [linux(wsl2g), windows]
