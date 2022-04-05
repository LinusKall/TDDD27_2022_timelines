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

