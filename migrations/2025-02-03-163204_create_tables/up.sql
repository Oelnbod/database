create table if not exists passwords (
  id integer primary key not null,
  website text not null,
  username text not null,
  password text not null
);
