let express = require('express');
let { graphqlHTTP } = require('express-graphql');
let { buildSchema } = require('graphql');
var graphql = require('graphql');
import { Character, Rarities, Classes, Types, Transformation } from "./character";

const characterdata: any[] = require("../data/DokkanCharacterData.json")



// Define the User type
var characterType = new graphql.GraphQLObjectType({
  name: 'Character',
  fields: {
    id: { type: graphql.GraphQLString },
    name: { type: graphql.GraphQLString },
    title: { type: graphql.GraphQLString },
  }
});

// Define the Query type
var queryType = new graphql.GraphQLObjectType({
  name: 'Query',
  fields: {
    character: {
      type: characterType,
      args: {
        id: { type: graphql.GraphQLString },
        title: { type: graphql.GraphQLString }
      },
      resolve: (_notUsed: any, args: any) => {
        return characterdata.find((character: { id: string }) => character.id === args.id);
      }
    }
  }
});

var schema = new graphql.GraphQLSchema({ query: queryType });

// Construct a schema, using GraphQL schema language
// let schema = buildSchema(`
//   type Character {
//     id: String
//     name: String
//     title: String
//   }

//   type Query {
//     character(id: String): Character
//   }
// `);

// The root provides a resolver function for each API endpoint
// let root = {
//   character: ({ id }: { id: string }) => {
//     return characterdata.find((character: { id: string }) => character.id.toLowerCase().includes(id.toLowerCase()));
//   },
// };

let app = express();
app.use('/graphql', graphqlHTTP({
  schema: schema,
  graphiql: true,
}));

app.listen(8080);
console.log('Running a GraphQL API server at http://localhost:8080/graphql');