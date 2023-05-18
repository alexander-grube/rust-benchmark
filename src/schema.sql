CREATE TABLE IF NOT EXISTS person (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    job VARCHAR NOT NULL,
    is_adult BOOLEAN NOT NULL,
    favorite_number SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
)

-- insert some test data with 5 different people that have different names, jobs other than software engineer, and random favorite numbers
insert into person (name, job, is_adult, favorite_number) values ('John', 'Software Engineer', true, 1);
insert into person (name, job, is_adult, favorite_number) values ('Jane', 'Software Engineer', true, 2);
insert into person (name, job, is_adult, favorite_number) values ('Bob', 'Software Engineer', true, 3);
insert into person (name, job, is_adult, favorite_number) values ('Alice', 'Software Engineer', true, 4);
insert into person (name, job, is_adult, favorite_number) values ('Eve', 'Software Engineer', true, 5);
insert into person (name, job, is_adult, favorite_number) values ('Mallory', 'Software Engineer', true, 6);
insert into person (name, job, is_adult, favorite_number) values ('Trent', 'Software Engineer', true, 7);
insert into person (name, job, is_adult, favorite_number) values ('Carol', 'Software Engineer', true, 8);
insert into person (name, job, is_adult, favorite_number) values ('Dave', 'Software Engineer', true, 9);
insert into person (name, job, is_adult, favorite_number) values ('Frank', 'Software Engineer', true, 10);
insert into person (name, job, is_adult, favorite_number) values ('Grace', 'Software Engineer', true, 11);
insert into person (name, job, is_adult, favorite_number) values ('Heidi', 'Software Engineer', true, 12);
insert into person (name, job, is_adult, favorite_number) values ('Ivan', 'Software Engineer', true, 13);
insert into person (name, job, is_adult, favorite_number) values ('Judy', 'Software Engineer', true, 14);
insert into person (name, job, is_adult, favorite_number) values ('Kevin', 'Software Engineer', true, 15);
insert into person (name, job, is_adult, favorite_number) values ('Larry', 'Software Engineer', true, 16);
insert into person (name, job, is_adult, favorite_number) values ('Mallory', 'Software Engineer', true, 17);
insert into person (name, job, is_adult, favorite_number) values ('Nancy', 'Software Engineer', true, 18);
insert into person (name, job, is_adult, favorite_number) values ('Oscar', 'Software Engineer', true, 19);
insert into person (name, job, is_adult, favorite_number) values ('Peggy', 'Software Engineer', true, 20);
insert into person (name, job, is_adult, favorite_number) values ('Quentin', 'Software Engineer', true, 21);
insert into person (name, job, is_adult, favorite_number) values ('Randy', 'Software Engineer', true, 22);
insert into person (name, job, is_adult, favorite_number) values ('Steve', 'Software Engineer', true, 23);
insert into person (name, job, is_adult, favorite_number) values ('Trent', 'Software Engineer', true, 24);
insert into person (name, job, is_adult, favorite_number) values ('Ursula', 'Software Engineer', true, 25);
insert into person (name, job, is_adult, favorite_number) values ('Victor', 'Software Engineer', true, 26);
insert into person (name, job, is_adult, favorite_number) values ('Walter', 'Software Engineer', true, 27);
insert into person (name, job, is_adult, favorite_number) values ('Xavier', 'Software Engineer', true, 28);
insert into person (name, job, is_adult, favorite_number) values ('Yvonne', 'Software Engineer', true, 29);
insert into person (name, job, is_adult, favorite_number) values ('Zelda', 'Software Engineer', true, 30);