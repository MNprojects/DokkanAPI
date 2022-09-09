let express = require('express');
let { graphqlHTTP } = require('express-graphql');
let { buildSchema } = require('graphql');
var graphql = require('graphql');
import { Character, Rarities, Classes, Types, Transformation } from "./character";

const characterdata: any[] = require("../data/DokkanCharacterData.json")

var characterType = new graphql.GraphQLObjectType({
  name: 'Character',
  fields: {
    id: {
      type: graphql.GraphQLNonNull(graphql.GraphQLString),
      description: 'The id of the character'
    },
    name: { type: graphql.GraphQLString },
    title: { type: graphql.GraphQLString },
  }
});

var queryType = new graphql.GraphQLObjectType({
  name: 'Query',
  fields: {
    character: {
      type: characterType,
      args: {
        id: { type: graphql.GraphQLString },
        title: { type: graphql.GraphQLString }
      },
      // @ts-ignore
      resolve: (_notUsed, args) => {
        if (args.id && args.title) {
          throw new Error("Can only return character by either id or title, not both at the same time.");
        }

        if (args.id) {
          return characterdata.find(character => character.id === args.id);
        }

        if (args.title) {
          return characterdata.find(character => character.title.toLowerCase() === args.title.toLowerCase());
        }
      }
    }
  }
});


var schema = new graphql.GraphQLSchema({ query: queryType });

let app = express();
app.use('/graphql', graphqlHTTP({
  schema: schema,
  graphiql: true,
}));

app.listen(8080);
console.log('Running a GraphQL API server at http://localhost:8080/graphql');