Diesel multi-join experiment
============================

Previous: https://github.com/pickfire/diesel-multi-join-play
Next: https://github.com/pickfire/diesel-join-not-exist

After getting footguns in sequelize and gorm, just want to try out diesel and
state of diesel.

Now starting to get hang of diesel after trying to contribute docs even though
I am confused myself, thanks to @weiznich comments I am able to learn quickly
and noticed that a lot of the stuff are documented even though I find them
hard to discover at first.

I noticed one of the main thing I was confused is on the generated schema.
I imported `users::dsl::*` and `posts::dsl::*` which made me confused and
I realized I never look into the docs for this part. After I took a look at
`hello::schema::users::dsl` part I noticed that some of the stuff there are
just useful re-exports then I realized that going `hello::schema::users` by
reading the docs is easy, like `users::table`. My bad for not reading docs. T_T

In the meantime, I was also stuck at some weird errors. I guess maybe I did
something wrong somewhere but the above helps, I was a bit confused about
modules and unit structs at first since I didn't go read docs but I wish there
are some suggestions on what traits could be used there, but the error seemed
weird to me.

```
error[E0223]: ambiguous associated type
  --> src/main.rs:19:45
   |
19 |     let query = diesel::update(posts.filter(posts::columns::id.eq_any(subquery)))
   |                                             ^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<hello::schema::posts::table as Trait>::columns`
```

The example I tested out here is to have 3 struct, same structure I used
for the previous experiment. I use sqlite for this for easy testing. I wanted
to update a field in another table based on another table, so the method I
used here is to use subquery.

    +---------+     +---------+     +---------+
    | User    |<-+  | Post    |     | Comment |
    +---------+  |  +---------+     +---------+
    | id      |  |  | id      |     | id      |
    | name    |  |  | title   |     | body    |
    |         |  |  | body    |<----+ post_id |
    |         |  |  | user_id +--+--+ user_id |
    +---------+  |  +---------+  |  +---------+
                 +---------------+

To update the posts title to "Z" the user name "John". The query,

```rust
let subquery = users::table
    .select(users::id)
    .filter(users::name.eq("John"));
let query = diesel::update(posts::table.filter(posts::user_id.eq_any(subquery)))
    .set(posts::title.eq("Z"));
```

Which results in the SQL query,

```sql
UPDATE `posts`
  SET `title` = ?
WHERE `posts`.`user_id`
   IN (
  SELECT `users`.`id`
   FROM `users`
  WHERE `users`.`name` = ?
) -- binds: ["Z", "John"]
```

## Get started

Rust, diesel_cli (with `sqlite` feature) is required.

```
$ diesel migration run
$ cargo run --bin init  # populate database
$ cargo run --bin hello  # query
```
