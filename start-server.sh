#!/bin/bash
# Start a simple HTTP server to run the HTML demo

# Check if Python 3 is available
if command -v python3 &> /dev/null; then
    python3 server.py
elif command -v python &> /dev/null; then
    python server.py
else
    echo "Python not found. Please install Python 3 or use one of these alternatives:"
    echo ""
    echo "Option 1: Using PHP"
    echo "  php -S localhost:8000"
    echo ""
    echo "Option 2: Using Node.js (http-server)"
    echo "  npx http-server -p 8000"
    echo ""
    echo "Option 3: Using Ruby"
    echo "  ruby -run -e httpd . -p 8000"
    echo ""
    echo "Then open http://localhost:8000/index.html in your browser"
    exit 1
fi

