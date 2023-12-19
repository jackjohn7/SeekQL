# SeekQL

SeekQL is a free and open-source SQL practice suite.

A user gains access to a number of practice problems. They are given
a prompt and must write a SQL query to satisfy the prompt's
requirements. They are also given a visual representation of the 
schema.

# How it works

When a user submits an answer, a new, embedded or in-memory database
is created with the dummy-data detailed in the prompt.

# Technologies

- Rust
- Askama
- HTMX
- SQLx
