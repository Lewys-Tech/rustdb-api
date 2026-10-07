rustdb-api 🌐

An HTTP API built entirely from scratch in Rust — no axum, no actix, no hyper. Just raw TcpListener, manual HTTP parsing, and your own routing logic — sitting on top of a real persistent storage engine (rustdb).

This is the final project in a 4-part series exploring systems programming in Rust: a chat server, a shell, a database engine, and now an HTTP layer that ties the database to the web.


Why Build HTTP From Scratch?

Frameworks like axum and actix hide all of this — and for production work, that's the right call. But building it manually once means:

You understand exactly what a Content-Length mismatch bug actually is
You know why read_exact exists and what happens without it
Debugging a framework's behavior later makes sense, because you've seen the raw mechanism it's built on
