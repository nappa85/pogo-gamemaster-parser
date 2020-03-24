# PoGo-GameMaster-Parser

## Project history

This project was born as a Rust program that parses the GameMaster, pushing the resulting combinations (aka every moveset and IV combination) into a database.<br />
To consume the database data, it's needed also a frontend, with a minimal backend to retrieve the data.<br />
Despite having the best possible performances, I had 3 different elements to maintain:
* Rust Parser
* Site Backend
* Site Frontend

This is a lot of work for a project that simple, not made for a big audience.<br />
So I throw away everything except the frontend (you can still find the Rust Parser in the `rust` branch), and made everything work from client's browser.<br />
Performances are bad, as in every modern web application, but the use case never implied good performances.<br />
Maybe, one day, I'll redo everyting with WASM for good performances.
