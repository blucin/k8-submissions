#!/usr/bin/env python3

import requests
import os
import sys


def main():
    # Fetch random Wikipedia article and create a todo.
    backend_url = os.environ.get('BACKEND_URL', 'http://todo-backend:3000')
    
    try:
        # Use a user agent to avoid being blocked
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        }
        
        response = requests.get(
            'https://en.wikipedia.org/wiki/Special:Random',
            headers=headers,
            allow_redirects=True,
            timeout=10
        )
        
        wiki_url = response.url
        
        if not wiki_url or 'Special:Random' in wiki_url:
            print(f"Error: Could not get a valid Wikipedia article URL. Got: {wiki_url}")
            return 1
        
        todo_text = f"Read {wiki_url}"
        
        response = requests.post(
            f"{backend_url}/todos",
            json={"text": todo_text},
            timeout=10,
            headers={"Content-Type": "application/json"}
        )
        
        if response.status_code in [200, 201]:
            print(f"Successfully created todo: {todo_text}")
            return 0
        else:
            print(f"Failed to create todo. Status: {response.status_code}, Response: {response.text}")
            return 1
            
    except Exception as e:
        print(f"Error: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main())
