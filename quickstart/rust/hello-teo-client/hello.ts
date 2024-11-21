import { teo } from './src/index.js'
 
async function main() {
  const results = await teo.user.create({
    create: {
      email: "ada@teodev.io",
      name: "Ada",
      posts: {
        create: [
          {
            title: "Framework TEO",
            content: "This post introduces Teo."
          },
          {
            title: "The next generation",
            content: "Use the next generation technology."
          }
        ]
      }
    },
    include: {
      posts: true
    }
  })
  console.log(results.data)
}
 
main()