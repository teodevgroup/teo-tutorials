from asyncio import run
from teo import App, Response, Request, HandlerGroup, TeoException
from entities import EchoPathArguments, StaticPathArguments, AlterCreatedAtInput, Teo, UploadInput
from pathlib import Path
from random import choice
from string import ascii_letters
from shutil import move

async def main():
    app = App()
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
    app.main_namespace().define_handler("hello", hello_handler)
    def empty_handler():
        return Response.empty()
    app.main_namespace().define_handler("empty", empty_handler)    
    def echo_handler(captures: EchoPathArguments):
        return Response.string(captures["data"], "text/plain")
    app.main_namespace().define_handler("echo", echo_handler)
    def static_handler(path_args: StaticPathArguments):
        return Response.send_file("static", path_args["path"])
    app.main_namespace().define_handler("static", static_handler)
    def record_group(group: HandlerGroup):
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
        group.define_handler("alterCreatedAt", alter_created_at_handler)
    app.main_namespace().define_model_handler_group("Record", record_group) 
    def upload_handler(input: UploadInput):
        extension = Path(input["file"].filepath).suffix
        random_file_name = ''.join(choice(ascii_letters) for i in range(6))
        destination = "static/images/" + random_file_name + extension
        result_path = "/" + destination
        move(input["file"].filepath, destination)
        return Response.data({ "path": result_path })
    app.main_namespace().define_handler("upload", upload_handler)
    await app.run()

run(main())