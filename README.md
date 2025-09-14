> [!CAUTION]
> This was speedran in a week, and the code is utter spaghetti which barely qualifies as an MVP. Proceed at your own risk.

# A PoC RAT (proof-of-concept remote access trojan)
This was done mainly to practice some async networking with Rust. Maybe I'll make it better, maybe not.

# Usage
The repository is divided into 4 sub-modules:
- The common library, shared across all projects
- The server, which handles the implant connections and acts as a relay between the dashboard and the implants
- The dashboard, which connects to the server to control the implants
- The implant itself, which is planted on the target machine, and maintains a connection to the C2

To run the server, simply use cargo run in the c2 directory.
To run the implant, specify the c2 address with the TYSE_ADDRESS environment variable, and cargo run.
To run the dashboard, do the same for the dashboard module.

For example, in dashboard:
```
TYSE_ADDRESS=127.0.0.1:9120 cargo run
```

Connects the dashboard to the Command and Control server running on localhost, port 9120 (the default port).
