# SeekQL

SeekQL is a free and open-source SQL practice suite.

A user gains access to a number of practice problems. They are given
a prompt and must write a SQL query to satisfy the prompt's
requirements. They are also given a visual representation of the
schema.

# How it works

When a user submits an answer, a new, embedded or in-memory database
is created with the dummy-data detailed in the prompt.

There are two types of questions: state-modification and data-
fetching . Some queries will require that the user modify the state
of the database. These operations include creating and modifying
tables and inserting rows.

# Technologies

- [Rust](https://www.rust-lang.org/)
- [Askama](https://github.com/djc/askama)
- [HTMX](https://htmx.org/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [TailwindCSS](https://tailwindcss.com/)

# Setup

Clone the repository:

``` bash
git clone https://github.com/jackjohn7/SeekQL
cd SeekQL
```

## With Docker

Requirements:

- [Docker](https://www.docker.com/)

Build the docker container

``` bash
docker build -t seek_ql .
```

Run the docker container

``` bash
docker run --publish 5173:5173 seek_ql
```

OR Optionally use the detach flag to free your terminal

``` bash
docker run --publish 5173:5173 -d seek_ql
```

## Without Docker

Requirements:

- [Cargo](https://rustup.rs/)
- [NPM](https://www.npmjs.com/)

Install node dependencies (tailwind and prettier):

``` bash
npm install
```

Run the application:

``` bash
cargo run
```

# Test Notation

The pseudo data model for tests can be seen below:

```json
"problems": [
    {
        "name": "name of problem",
        "description": "description of problem",
        "migration_query": "migration or location of migration",
        "markdown": "markdown or markdown location detailing problem",
        "problem_type": "state_change | data_fetch",
        "expected_result": "describe the state of db or the rows returned"
    }
]
```
