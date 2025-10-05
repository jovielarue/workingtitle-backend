# Working Title Backend

## Endpoints

_All endpoints are prefixed by http://localhost:5000 and recieve formdata_

### Auth Endpoints

- **POST** /authorize -- get JWT -- credentials are "test@email.com" and "password"

### User Endpoints

- **POST** /users -- create user -- takes in user struct fields
- **GET** /users/<id> -- get user
- **PATCH** /users/<id> -- update user -- takes in one or more of the user struct fields to update
- **DELETE** /users/<id> -- delete user
-

### Ticket Endpoints

- **POST** /tickets -- create ticket -- takes in ticket struct fields
- **GET** /tickets/<id> -- get ticket
- **PATCH** /tickets/<id> -- update ticket -- takes in one or more of the ticket struct fields to update
- **DELETE** /tickets/<id> -- delete ticket
-

### Event Endpoints

- **POST** /events -- create event -- takes in event struct fields
- **GET** /events/<id> -- get event
- **PATCH** /events/<id> -- update event -- takes in one or more of the event struct fields to update
- **DELETE** /events/<id> -- delete event
