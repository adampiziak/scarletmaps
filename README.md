# ScarletBus
Welcome to the ScarletBus API. This is a GraphQL API built on top of the Transloc service used by Rutgers University. It aims to provide extensive information about Rutgers routes and stops in a flexible manner. Because this is a GraphQL endpoint, the responses can be tailored to the user's specific needs.

## Types
This API provides four root types

- route(id: `int!`)
  - id `int!`
  - name `String!`
  - areas `[String!]!`
  - segments `[String!]!`
  - stops `[Stop!]`

- stop(id: `int!`)
  - id `int!`
  - name `String!`
  - area `String!`
  - routes `[Route!]`

- routes(active: `bool`)
- stops(active: `bool`)
## Examples
For example, the user can model a request to return the names of the currently active routes and their served areas.

model
```js
{
  routes (active: true) {
    names
    areas
  }
}
```

request
```js
axios.post('https://api.scarletbus.com/graphql/', {
    query: {
      {
        routes (active: true) {
          names
          areas
        }
      }
    }
});
/*
{
  routes[
    {
      name: "A",
      areas: ["Busch", "College Ave"]
    },
    {
      name: "B",
      areas: ["Busch", Livingston]
    }
    ....
  ]
}

*/
```

## License
This software falls under the MIT permissive license.
