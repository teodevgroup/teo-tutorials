from asyncio import run, get_event_loop, new_event_loop
from teo import App, Response


async def main():
    app = App()
    def hello_handler(_request):
        return Response.html("""
            <html>
                <head>
                    <title>Hello, Teo handlers</title>
                </head>
                <body>
                    <h1>Hello, Teo handlers!</h1>
                </body>
            </html>
        """)
    app.main_namespace().define_handler("hello", hello_handler)
    await app.run()

