import subprocess
import os
from http.server import HTTPServer, CGIHTTPRequestHandler

subprocess.call(["cargo", "build", "--release", "--example", "scene_graph", "--target", "wasm32-unknown-unknown"])
subprocess.call(["wasm-bindgen", "./target/wasm32-unknown-unknown/debug/examples/scene_graph.wasm", "--out-dir", "web", "--target", "web"])

# create the server
os.chdir('./web')
server_object = HTTPServer(server_address=('', 8000), RequestHandlerClass=CGIHTTPRequestHandler)
server_object.serve_forever()