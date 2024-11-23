from asyncio import run
from teo import App, Response, HandlerGroup, TeoException
from entities import EchoPathArguments, StaticPathArguments, AlterCreatedAtInput, Teo, UploadInput
from pathlib import Path
from random import choice
from string import ascii_letters
from shutil import move

async def main():
    app = App()
    @app.main.handler("hello")
    def hello_handler():
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
    @app.main.handler("empty")    
    def empty_handler():
        return Response.empty()
    @app.main.handler("echo")
    def echo_handler(captures: EchoPathArguments):
        return Response.string(captures["data"], "text/plain")
    @app.main.handler("static")
    def static_handler(path_args: StaticPathArguments):
        return Response.send_file("static", path_args["path"])
    @app.main.model("Record").handler("alterCreatedAt")
    async def alter_created_at_handler(input: AlterCreatedAtInput, teo: Teo):
        record = await teo.record.find_unique_object({
            "where": {
                "id": input["id"]
            }
        })
        if record is None:
            raise TeoException.not_found()
        record.created_at = input["createdAt"]
        await record.save()
        return Response.data(await record.to_teon())
    @app.main.handler("upload")
    def upload_handler(input: UploadInput):
        extension = Path(input["file"].filepath).suffix
        random_file_name = ''.join(choice(ascii_letters) for i in range(6))
        destination = "static/images/" + random_file_name + extension
        result_path = "/" + destination
        move(input["file"].filepath, destination)
        return Response.data({ "path": result_path })
    await app.run()

run(main())