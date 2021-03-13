# urlet

This project holds the front- and backend of the 'urlet' url shortening service.

The backend is written in rust using actix web framework. Backend stores data into postgres database. The sqlx library is used to query the database.

The frontend is built with svelte using the snowpack build tool. Frontend is served as static files from nginx. The api calls are proxied to the api server.