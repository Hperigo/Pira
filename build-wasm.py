import subprocess
import os
from http.server import HTTPServer, CGIHTTPRequestHandler

subprocess.call(["cargo", "build", "--release", "--example", "instances", "--target", "wasm32-unknown-unknown"])
subprocess.call(["wasm-bindgen", "./target/wasm32-unknown-unknown/release/examples/instances.wasm", "--out-dir", "web", "--target", "web"])

# create the server
os.chdir('./web')
# server_object = HTTPServer(server_address=('', 8000), RequestHandlerClass=CGIHTTPRequestHandler)
# server_object.serve_forever()


import http.server
import socketserver

PORT = 8000

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map.update({
      ".js": "application/javascript",
});

httpd = socketserver.TCPServer(("", PORT), Handler)
httpd.serve_forever()