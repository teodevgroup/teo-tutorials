import { App, Response, Request } from '@teodevgroup/teo'
import { AlterCreatedAtInput, EchoPathArguments, StaticPathArguments, Teo, UploadInput } from './entities'
import path from 'path'
import fs from 'fs'

const app = new App()
app.mainNamespace().defineHandler("hello", () => {
    return Response.html(`
        <html>
            <head>
                <title>Hello, Teo handlers</title>
            </head>
            <body>
                <h1>Hello, Teo handlers!</h1>
            </body>
        </html>
    `)
})
app.mainNamespace().defineHandler("empty", () => {
    return Response.empty()
})
app.mainNamespace().defineHandler("echo", (request: Request) => {
    const pathArguments: EchoPathArguments = request.captures()
    return Response.string(pathArguments.data, "text/plain")
})
app.mainNamespace().defineHandler("static", (request) => {
    const pathArguments: StaticPathArguments = request.captures()
    return Response.sendFile("static", pathArguments.path)
})
app.mainNamespace().defineModelHandlerGroup("Record", (group) => {
    group.defineHandler("alterCreatedAt", async (request) => {
        const teo: Teo = request.teo()
        const input: AlterCreatedAtInput = request.bodyObject()
        const record = await teo.record.findUniqueObject({
            "where": {
                "id": input.id
            }
        })
        if (record) {
            record.createdAt = input.createdAt
            await record.save()
            return Response.data(await record.toTeon())
        } else {
            throw Error("not found")
        }
    })
})
app.mainNamespace().defineHandler("upload", (request) => {
    const input: UploadInput = request.bodyObject()
    const extension = path.extname(input["file"].filepath)
    const randomFileName = (Math.random() + 1).toString(36).substring(7)
    const destination = "static/images/" + randomFileName + extension
    const resultPath = "/" + destination
    fs.copyFileSync(input["file"].filepath, destination)
    return Response.data({ path: resultPath })
})

app.run()