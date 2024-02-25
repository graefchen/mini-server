# mini-server

A small web server written in Rust (you probably should not use it).

## references

- [Coding a Web Server in 25 Lines - Computerphile](https://youtu.be/7GBlCinu9yg)
- [Rust Book: Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Wikipedia - HTTP](https://en.wikipedia.org/wiki/HTTP)

## design

Some design goals:

1. minimal (in code size and dependencies) + resiliant + multithreaded (maybe?)
2. always return index.html
3. When no index.html page -> costum file browser
4. Ignore /../ request -> default to /
5. When in this address: "address/blog/" blog is not a dictionary (with an
   index.html file in it), but a file -> return the file
6. Only allow http requests ... for now ...
7. Make it fancy like `serve` ... with Information
   - print fancy header localhost address
   - print fancy request
   - print fancy answer
8. Make the IP address default to the current ip address and make it available
   to the network
9. Add some options to maybe change the port, if not defined
10. If a folder / file is not existing look into base folder and return the
    defined errorcode.html
11. If such a file does not exist look if the file exist in an error folder.html

## start server

```shell
$ mini-server
```
