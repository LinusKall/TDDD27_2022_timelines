# Timelines

A simple calendar application

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

