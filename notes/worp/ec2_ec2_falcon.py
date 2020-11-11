import falcon
from werkzeug.serving import run_simple

app = falcon.API()

class HelloViewAPI:
    """
         This is an endpoint for Hello View
    """
    def on_get(self, req, resp):
        resp.status = falcon.HTTP_200
        resp.content_type = "text/plain"
        resp.body = "Hello World!"
    def on_post(self, req, resp):
        resp.status = falcon.HTTP_201
        resp.content_type = "text/plain"
        resp.body = "Resource added successfully"
    def on_put(self, req, resp):
        pass
    def on_delete(self, req, resp):
        """
            your codes goes here for delete method
        """
        pass


hobj = HelloViewAPI()

app.add_route('/hello', hobj)


if __name__ == '__main__':
    run_simple('0.0.0.0', 80, app)



#python3:


import urllib.request
import time

start = time.time()

contents = urllib.request.urlopen("http://0.0.0.0:80/hello").read()
end = time.time()
print(contents)
print(end - start)

#1.6 ms on the same machine

