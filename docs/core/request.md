Represents the incoming HTTP request

It contains information about the request, such as the request `method`, `path`, `query string`, `parameters`, `headers`, and `body`.

[`Request`] cannot be accessed directly or modified anywhere in the code. It can only be accessed through the handler or middleware function with an immutable reference via methods.

