<img src="documentation/logo.svg" height="60" />

# ScarletMaps
<p>
  <a href="/LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  </p>
Welcome to the ScarletMaps API. This is a GraphQL API built on top of the Transloc service used by Rutgers University. It aims to provide extensive information about Rutgers routes and stops in a flexible manner. Because this is a GraphQL endpoint, the responses can be tailored to the user's specific needs.

## Clients
| Name | Platform | Repository | Description | 
| :--- | :------- | :--------- | :---------- | 
| ScarletMaps | Web | [scarletmaps-web](https://github.com/adam-piziak/scarletmaps-web) | Web client |
| ScarletMaps | Android | [scarletmaps-android](https://github.com/adam-piziak/scarletmaps-android) | Android client |
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
This project is licensed under the terms of the MIT license. See [LICENSE](https://github.com/adam-piziak/scarletbus/blob/master/LICENSE) for more details.
