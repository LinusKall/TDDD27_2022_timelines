# Group and individual presentations

## Group

[2022 TDDD27 LiU Timelines](https://www.youtube.com/watch?v=6h2XQy4MpsE)

## Individual

[2022 TDDD27 LiU Timelines edvkj374](https://www.youtube.com/watch?v=HIPMSMIPJdw)

[2022 TDDD27 LiU Timelines linka231](https://www.youtube.com/watch?v=EPOofzJCSEI)

[2022 TDDD27 LiU Timelines sebfr298](https://www.youtube.com/watch?v=kydb9X3GjEU)

# Timelines

A simple calendar application

## Description

This project aims to create a task manager/calendar webpage similar to google calendar and google tasks. The main functions of the tool should be that every user can create timelines for different categories of tasks, color code these timelines as well as schedule their tasks. Ultimately we would also like to implement a database of tasks that will work as suggestions on tasks to add to the user's timeline.

## Functional Specification

Here we describe the high-level functionality we would like to implement for the different parts in the stack.

### Backend + Database

* Seperate the web and database servers into two binaries to keep the project more modular, and thus more scalable.
* Use GraphQL (and/or REST) for communication between server and client.
* Authentication between client, web server, and database server.

### Frontend

* Key commands for navigation.
* Multiple views/components for displaying timelines
    * List
    * Calendar
* Create new timelines
* Add task/event to timeline
* Find overlapping tasks/events
* Highlight overlapping tasks/events when creating new task/event
* Depending on your permissions for a timeline, you can add other users to a timeline and give them permissions.
* Each timeline has an associated color decided at random or by a user with the correct permissions.
* (Optional) Interoperate with other online calendars (e.g. Google calendar).

## Technical Specification

These are the libraries and/or frameworks which are intrested in using.

### Frontend

For the frontend, there are two core frameworks which we have taken into consideration.

#### React

React is a framework developed by Meta, built on top of the JSX standard. This allows developers to easily create components and web applications with functionality and form.

React could be considered somewhat modular and/or barebones, meaning that a lot of the functionality is often provided by third-party frameworks.

React is probably the most popular solution for frontend development, thus there are a lot of resources available to us.

#### Yew

Yew is a frontend framework written in the Rust programming language. Like React, it utilizes the concept of components. Unlike React, the compiled code mostly consists of Webassembly.

Webassembly often has a smaller footprint compared to JavaScript code and is sometimes faster, making it more efficient.

Another advantage of writing frontend applications using Yew are the inherit advantages of writing code in Rust, which comes with a lot of memory safety guarantees. Microsoft once estimated that [70% of all security bugs were memory safety issues](https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/).

### Backend

For the backend, we are intrested in using the following frameworks.

#### Actix-Web

Actix-web is framework for writing webservers in Rust. It has a lot of features, including support for websockets, middleware, REST, static files, and more.

#### Diesel

Diesel is a framework used for writing database queries in Rust. This will allow us to send queries from a web server (written in Rust) to a database.

#### Tokio-Postgres

A framework used for sending SQL queries in Rust. It is made by the Tokio group, which is sponsored by AWS, Discord, Dropbox, Facebook, and others.

#### Juniper

Juniper is framework which makes it easier to implement GraphQL into a web server written in Rust.

### Database

At the moment, we are not completly sure which database we want to use. Diesel and other frameworks written in Rust easily interoperate with PostgreSQL, which we are not completly familiar with. Could be a good challenge! We are more familiar with MySQL, which does not seem to be as prevelant within the Rust community.

#### PostgreSQL

A scalable Object-Relational database. Known for its reliability, feature robustness, and performance.

#### SQLite

Probably one of the most utilized databases. Known for its simplicity and wide adoption.
