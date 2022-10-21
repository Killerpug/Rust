# Building a Multithreaded web server

Here is the plan to build the web server:

* Learn a bit about TCP and HTTP.
* Listen for TCP connections on a socket.
* Parse a small number of HTTP requests.
* Create a proper HTTP response.
* Improve the throughput of our server with a thread pool.

Note: the method we’ll use won’t be the best way to build a web server with Rust.

## A quick overview on TCP and HTTP

The two main protocols involved in web servers are the Hypertext Transfer Protocol (HTTP) and the Transmission Control Protocol (TCP). Both protocols are request-response protocols, meaning a client initiates requests and a server listens to the requests and provides a response to the client. The contents of those requests and responses are defined by the protocols.

TCP is the lower-level protocol that describes the details of how information gets from one server to another but doesn’t specify what that information is. HTTP builds on top of TCP by defining the contents of the requests and responses. It’s technically possible to use HTTP with other protocols, but in the vast majority of cases, HTTP sends its data over TCP. We’ll work with the raw bytes of TCP and HTTP requests and responses.

