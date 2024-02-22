I keep trying to figure out how to build my basic blog but, as I've kept falling into potholes, I've decided to chronicle my experience in this document the process of building my first ever "dynamic" website, in hopes it may one day be a useful resource for other beginners, or, a reference for the kinds of things beginners may be confused of.

I already have a long history of hobbyist *static* web-development, but have just never managed to make the jump towards building anything that interacts with a backend of some sort, alhtough not for a lack of trying. I've wanted to have a blog of some sorts for a while now, but I've never much of a fan of Wordpress, or PHP, or even just databases quite frankly.

I'm kind of hesitant about using a database in general because I just, don't, understand them; I feel like they're over-complicated and introduce a sudden external dependency for your application I suddenly have to also worry about. As such, I originally wanted to try doing a "flat-file CRM" that stored/read all the blog posts as just .md files in a folder I could edit. But, as time goes on, I've kind of been drawn to just, do a database-based solution, if for no better reason than to just try to better understand how databases work, and how programming for databases work.

I'd long been thinking about finding a project to do Rust, both because I wanted to learn Rust, and I thought It'd be cool to build a web-app that's really low-level and performant. I was also hoping the compiler would hopefully whip me into keeping to best-practices while exploring this quite dangerous field of programming.

So, okay, brief project descriptor: Building a basic Blog backend, in Rust, using a Database.

Having ruminated over this basic premise for quite some time, I've come up with a rough roadmap for how I'd likely want to approach developing something like this:
- Firstly: Start off by building it all as a simple, offline CLI-application for creating, viewing, editing and deleting blog-posts. This lets me figure out how I'll be doing all the database interaction with as little technical overhead as I can hope for.
- Once that's done; hook this up with a web-framework like Actix or Axum. Figure out some basic templating, and bam! A basic blog! Put it behind a reverse-proxy and it should be good to go!
- Lastly, as a stretch-goal; Try turning the app into an API, and build the web-frontend as a seperate web-application with something like Vue, perhaps Nuxt?

Thus, if this is the outline I'm to follow, the first step will be to figure out database interactions.

In my extremely limited experience building any sort of interactive web app, I've only really ever worked with Django (see [dyablog](https://github.com/0xBA5E64/dyablog)), which is a web-framework for Python that includes a lot of niceities by default. In the case of "storing data", it provides a really robust "ORM" - a thingy that abstracts away the need to interact with the database directly and, instead lets you interact with the database as if they were just objects in code

These objects are defined as models in code, and so, for instance, if you had a BlogPost model, you could get all your blog-posts by doing:
```py
from app.models.blogPosts import BlogPosts

blog_posts = BlogPosts.objects.all()
```
What's especially cool about it is that you can even filter things super easily and ergonomically like:
```py
# In this example, imagine certain blog-posts are "introductions", and so have "Introduc"+(ing|tion) in the title
# We can filter objects by their title-field by just doing title="something", but, even better, by using two underscores-
# -(__) we can access certain filtering "functions", such as "__contains", which checks if the string provided is somewhere in the given field.
# This can additionally be made case-insensitive via __icontains
intro_blog_posts = BlogPosts.objects.filter(title__icontains="introduc")
```
Integrating directly with SQL is probably more flexible but, I've never really understood SQL, and it's an additional language cluttering up my codebase.Therefore, I thought, to keep things from being needlessly complicated, I should spring to use an ORM in this app as well, ...right? Well, maybe.

Thing is, the one ORM I found that was recommended for Rust is called diesel.rs. And, it felt a bit... lacking.

For one, in the "Getting Started" guide, explaining how to set up a new model for you in your program, you basically have to define it thrice;
- Once in SQL, manually, as up/down migrations (wait, isn't this exactly what I was trying to avoid to begin with?)
- Next as a a "schema", which defines your SQL table fields in some sort of rust-sql-diesel limbo
- Lastly; as a "model", which is the actual Rust struct (object) it's gonna be mapped to.
Oh, and as if that wasn't tedious enough, you have to manually assign what type each field is supposed to map to, from the database, to the object. 

Okay so, this feels, weird, right? Like, that's a lot of manual labour that needs to be done, which feels like it could/should be abstracted away, right?
And, to be frank, some it apparently can be;
- The SQL migrations can apparently be auto-generated from the schema by doing diesel migration generate --diff-schema, but this is listed as an alternative method, not a primary one.
- Similarly. the models can alledgedly be auto-generated as-well by a third-party extension called "dsync", but again, that raises the question of; then why is it not built in to begin with?

These questions may come of as rhetorical, but they're not; I genuinely wanted answers. And so, I went looking. After reading a bunch of Reddit-posts, Stack-Overflow answers, and HackerNews comments, I seem to've found some answers, albeit indirectly:

- The general consensus by experienced devs appear to be to stay away from ORM's, as the alluring ease-of-use they alledge to offer usually just ends up being a headache as you begin to want to do more advanced queries and database operations. Ergo; ORM's are not worth the hassle they cause.
- Diesel therefore tries to somewhat circumvent this, by stripping away much of the abstractions ORM's usually offer, and forcing you as a developer to handle much of it manually, but still offering some of the niceities of an ORM once fully set-up. Ergo; Diesel.rs is not necessarily an ORM in the traditional manner, but rather a compromise of sorts, born from trying to resolve some of their shortcomings
- "If you want to build something with database-integration in Rust, use SQLx, it's an awesome universal library for integrating with most databases, and async!"

...so, in the end, unless I wanna spend the time to set up diesel.rs, I guess using SQLx would be the "better" option. There are two problems however:
- I still don't know SQL.
- I have tried and failed, dozens of times before, to grasp the what and how of async programming.

Still, apparently, if I am trying to build a web-service, it's best practice to be async anyhow, so I guess I have to learn that too now.

What I know is, for async code to work in Rust, it needs a "runtime". The most popular of which is tokio.rs
As a small light at the end of the thunnel though, tokio.rs also offers it's own web-framework to work with; Axom, so I guess that part of the stack falls into place nicely too now.

Bad news is, reading up briefly on Tokio, it appears to make Rust variable lifetimes even more of a nightmare to manage. The "good news" is, if I can learn to make it in Rust, it means I'll be learning to do it right, since Rust will force me to write it in a safe manner, whereas this sort of thing is usually extremely error-prone and risky.


# Brief TL;DR outline
- I want to have a blog.
- I don't like pre-existing solutions; too complex & confusing. I could do with a good code-project anyhow.
  - Part of said confusion is Databases: They're confusing!
    - An additional depencency to worry about; the app is no longer self-contained.
    - Introduce an additional language to worry about (SQL).
    - Additional security risks! SQL-Injection seems like a nightmare to watertighten.
- I explored flat-file CMS's.
  - Am not too pleased with that eco-system either, although they do feel better than their Database-driven counterparts.
  - Static page generators are also a thing, but, it would be cool to be able to edit things on-the-fly...
- I want to try to build a blog.
  - Flat-file sounds nice, static-site gen?
  - Despite displeasure; learning how to interact with databases is honestly probably a good idea.
  - As would learning Rust!
- Use an ORM?
  - Internet: Don't use an ORM.
- Database in Rust? rusqlite? SQLx?
  - rusqulite: it works, is basic (:+1:),
  - SQLx: community favorite!
    - SQLx? Async. Async? Tokio.
- So... Rust, SQLx, Tokio + Axom.

# diesel.rs vs SQLx

I've now tried using both diesel.rs & SQLx to interface with databases from Rust, and I have thus formed som opinions:

diesel.rs put me off as a beginner; being an ORM; I was expecting it to help offload much of the inital setup needed to get up and running with a database. Instead, it just felt like it complicated things further. My database/model had to be defined thrice, manually (see previous section), and much of the user experience in general just felt very manual. Because of this, I decided to give SQLx a shot; appearing to be a community darling of sorts, it alledgedly offered simpler access to raw SQL queries, that could still be typed-checked to validate validity. Despite this, I felt much of what I ended up doing for SQLx as being redundant from how diesel.rs too operated, yet somehow even more confusing and poorly documented. Moreover, SQLx also wants to manage database migrations for me, but does this solely through SQL apparenly, using an additional cli-tool called sqlx-sli.

...All in all, I'm not sure what I think is better, though I'm currently learning towards returning to diesel.rs, if for no other reason than that it's syncronous by default so, simple, to get started with. Still, SQLx, wasn't too bad either, so I don't know. Using both did certainly feel like it gave me a greater understanding of how database-interactions *look* and *feel* in Rust though, and, perhaps wider programming as well. Again, this is all new to me.


# Lacking documentation

Maybe it's just me being a rookie to this, but I found documentation for a lot of what I'm trying to do to be severely lacking. SQLx feels like it barely has any documentation to speak of, whereas diesel.rs, which to it's credit *does* have a getting-started guide, also feels severely lacking, all things considered.

Take for instance, my current perdicament; I want to build a web-app/api that fetches data from an API upon requests and outputs this to the user. The way I imagine this would be done is by having some sort of web-framework set up to do "routing" or whatever, and register a function to a certain path, such as, let's say; "/articles". Upon a visit, said web-framework runs the function, grabs the output, and sends it away to the user, neat, right?

...right? Because, that's actually not a rhetorical, I don't know if that's the recommended way of going about something like this, or if I should be fundementally structuring this some other way; am I shooting myself in the foot by doing things like this, or should I 

Whatever; this is a research/learning project, after all! It's good to run into potential issues, and learn from potential mistakes. So let's just start off building it however I first imagined doing it.

But then, we run into some immediate roadblocks; first of all, like I've mentioned previously, a lot of the recommended tools people mention for doing something like this are "async", which I don't understand, and have so far had no luck with teaching myself.

Secondly; SQLx - While perusing the paper-thin introduction to this framework featured in it's README, something almost immediately stood out to me; `___PoolOptions`... oh, yeah, it's probably a good idea to have one of those, right, cause starting a new database connection with each request is probably gonna cause slowdown or something of the sort. I could like, initiate one alongside the web-server, and pass individual connections to the different "request handlers" (I think they're called? The functions that give back an output for the server to serve)

So, here's an example where, the documentation feels like it's pushing towards doing things in a certain way. In this case; setting up a re-usable thread-pool for your app instead of creating & dropping connections nilly-willy per call.  which is probably smart but, I'm then also not sure if I'm missing out on *other* potential things I should be doing differently too.