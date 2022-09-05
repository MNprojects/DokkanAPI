let express = require('express');
let { graphqlHTTP } = require('express-graphql');
let { buildSchema } = require('graphql');
const characterdata: any[] = require("../data/DokkanCharacterData.json")

// Construct a schema, using GraphQL schema language
let schema = buildSchema(`
  type Query {
    characterName(name: String): [String]
  }
`);
 
// The root provides a resolver function for each API endpoint
let root = {
  characterName: (args: any) => {
    return characterdata.filter((character: { Name: string; }) => character.Name.toLowerCase().includes(args.name.toLowerCase())).map(character => character.Name);
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