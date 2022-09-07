let express = require('express');
let { graphqlHTTP } = require('express-graphql');
let { buildSchema } = require('graphql');
const characterdata: any[] = require("../data/DokkanCharacterData.json")

// Construct a schema, using GraphQL schema language
let schema = buildSchema(`
  type Character {
    id: String
    name: String
    title: String
  }

  type Query {
    character(id: String): Character
  }
`);

// The root provides a resolver function for each API endpoint
let root = {
  character: ({ id }: { id: String }) => {
    return characterdata.find((character: { id: string }) => character.id.toLowerCase().includes(id.toLowerCase()));
  },
};

let app = express();
app.use('/graphql', graphqlHTTP({
  schema: schema,
  rootValue: root,
  graphiql: true,
}));

app.listen(8080);
console.log('Running a GraphQL API server at http://localhost:8080/graphql');