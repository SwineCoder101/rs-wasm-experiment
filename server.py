#!/usr/bin/env python3
"""
Simple HTTP server to run the HTML demo locally.
WASM modules require CORS, so we need a web server instead of opening the file directly.
"""

import http.server
import socketserver
import webbrowser
import os
import sys

PORT = 8000

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Access-Control-Allow-Origin', '*')
        super().end_headers()

def main():
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    Handler = MyHTTPRequestHandler
    port = PORT
    
    try:
        with socketserver.TCPServer(("", port), Handler) as httpd:
            url = f"http://localhost:{port}/index.html"
            print(f"Server starting at {url}")
            print(f"Press Ctrl+C to stop the server")
            webbrowser.open(url)
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nServer stopped.")
        sys.exit(0)
    except OSError as e:
        if e.errno == 48:  # Address already in use
            port = PORT + 1
            print(f"Port {PORT} is already in use. Trying port {port}...")
            with socketserver.TCPServer(("", port), Handler) as httpd:
                url = f"http://localhost:{port}/index.html"
                print(f"Server starting at {url}")
                print(f"Press Ctrl+C to stop the server")
                webbrowser.open(url)
                httpd.serve_forever()
        else:
            raise

if __name__ == "__main__":
    main()

