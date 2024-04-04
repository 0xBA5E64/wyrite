# Introduction
I've never quite understood the motivation behind dedicated databases like MySQL or PostgreSQL; what value one might derive from storing data in a seperate application from the one you're actually running, though some sort of "standardized" mechanism, when each and every application still handles their data differenly.
Even with something like SQLite, that can be integrated into the application itself, why would one want to use something which requires a completely seperate "querying lanaguage" like SQL, instead of say, just re-using something like a list, vector or "HashMap" inside the body of the language itself, and serialize it onto the disk for persistance? Granted, some applications do just that, but many chose the route of **SQL**; why? - **It's usually a rhetorical for me, but here, I intend to actually find some answers for myself though this exploration and documentation adventure!**


# The SQL Syntax
SQL, as a "syntax", has always seemed like a confusingly verbose "language" that never quite satisfied my desire for systematic syntax, instead opting for some weird hybrid that looks part code, part English.
Take the following query as an example:
```SQL
CREATE USER foo2@test IDENTIFIED BY 'password';
```
Reading it it's pretty easy to understand what's going on; you're creating a user `foo2` that's "*at* `test`", with a password of "`password`". Ignoring whatever "@test" is for now, this is pretty simple to read, and barring the monospace typeface and all-caps syntax, I'm pretty sure most English speakers off the street could tell you what this is supposed to convey.

Things get trickier however as soon as I try to decipher the syntax; In a domain like computing where spaces are used to destinguish between distinct parts of a command or statement, why is `IDENTIFIED` and `BY` separate? To me this signals that there may be a time where you might use `IDENTIFIED` without `BY`, but when exactly would that be!? For that matter, why is the verb `IDENTIFIED` used to specify a password, a concept that's much more commonly associated with "Authentication"?

Knowing `@test` is specifying the `foo2`-user as only being authorized to connect from the `test` Host, were *I* to design the syntax for something like this, I'd probably want it to look a bit more like:
```SQL
USER CREATE 'foo2' AUTH HOST 'test' AUTH PASSWORD 'password';
```
We want to create a user, so we call on `CREATE` from `USER`, then we pass the username (`foo2`), and any additional parameters, such as authentication settings are optional statements to further configure the user.


# Inserting data into a database ...safely?
Similarly, I've long wondered how one is supposed to *input __data__* into an SQL database. Now, this might seem like something that'd be utterly obvious for even a beginner to simply look up, and sure enough, search and you will find answers:
```SQL
INSERT INTO customers (name, adress, balance) VALUES ("John Doe", "Local-street, 127.0.0.1", "39.99");
```
Simple enough, right? And yeah, honestly, I think it is quite simple.
But what if I instead wanted to say, insert a string with a quotation-mark in it in one of the fields? Sure, we might able to escape it first, so what about a line-break; perhaps we want the adress to be split across multiple lines, what then? Worse yet, what if I wanted to store *arbitrary __binary__ data* in any of these fields? Say, a profile *image*? How am I supposed to insert that? I guess I could maybe encode it into something like `base64`, but that feels *mighty* hacky, and from what I understand, databases *are* indeed supposed to be able to have "binary blob" columns, so how am I supposed to interact with those?

Well, as I began looking though SQLx's documentation, I found my answer: **"parameterized queries"**, which is to define your SQL *query* with *variables* in place of values, and passing the data in seperately though your database interface:
```Rust

type Person struct {
    FirstName string `db:"first_name"`
    LastName  string `db:"last_name"`
    Email     string
}

const newEntry = Person{
    first_name: "Bin",
    last_name: "Smuth",
    email: "bensmith@allblacks.nz",
}

db.NamedExec(`INSERT INTO person (first_name,last_name,email) VALUES (:first_name,:last_name,:email)`, Person)

// TODO: ^ Verify all of this code is an actually working example.
// (This was quickly scrap-booked together from the sqlx readme)

```
...Neat! No, really! That sounds perfectly sound to me; Write the SQL almost like a shipping label, and then just attach the data. This also apparently has the added benefit of preventing SQL injec- ...wait. I-is that *seriously* how SQL-injection vulnerabilities happen!? Do people seriously just stick user data straight into the SQL query!? That's... at all, acceptable!?