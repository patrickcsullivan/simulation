# Organization

The eventual goal is to have three or more crates:

- `sim_server`: server that runs the simulation and talks to clients and potentially other servers
- `player_client`: WASM client used for interacting with the simulation
- `..._client`: other clients that might, for example, log data or perform analytics
- `message`: shared library that defines message structure for communication between different executables
- `...`: other shared libraries

For now, this project just contains the single `sim_server` crate.