CREATE TABLE IF NOT EXISTS users (
       id varchar(100) PRIMARY KEY,
       username varchar(32) NOT NULL,
       email_address varchar(100) NOT NULL,
       password_hash varchar(255) NOT NULL,
       created_date timestamp with time zone DEFAULT (now() at time zone 'utc')
);

CREATE TABLE IF NOT EXISTS problems (
       id SERIAL PRIMARY KEY,
       problem_name varchar(255) NOT NULL,
       description varchar(255) NOT NULL,
       markdown text NOT NULL,
       eval_data text NOT NULL,
       db_migration text NOT NULL
);

CREATE TABLE IF NOT EXISTS completions (
       id varchar(100) PRIMARY KEY,
       user_id varchar(100) NOT NULL,
       problem_id integer NOT NULL,
       FOREIGN KEY (user_id) REFERENCES users(id),
       FOREIGN KEY (problem_id) REFERENCES problems(id)
);

-- In the future, email verification will be required

--CREATE TABLE IF NOT EXISTS emails_sent (
--       id varchar(100) PRIMARY KEY,
--       receiver_address varchar(100) NOT NULL,
--       sender_address varchar(100) NOT NULL,
--       content text NOT NULL
--);
--
--CREATE TABLE IF NOT EXISTS email_verifications (
--       id varchar(100) PRIMARY KEY,
--       user_id varchar(100) NOT NULL,
--       email_address varchar(100) NOT NULL,
--       email_sent_id varchar(100) NOT NULL,
--       verified boolean DEFAULT FALSE,
--       date_updated timestamp with time zone DEFAULT (now() at time zone 'utc'),
--       FOREIGN KEY (user_id) REFERENCES users(id),
--       FOREIGN KEY(email_sent_id) REFERENCES emails_sent(id)
--);
