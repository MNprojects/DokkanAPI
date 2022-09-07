let express = require('express');
let { graphqlHTTP } = require('express-graphql');
let { buildSchema } = require('graphql');
var graphql = require('graphql');
import { Character, Rarities, Classes, Types, Transformation } from "./character";

const characterdata: any[] = require("../data/DokkanCharacterData.json")

var characterType = new graphql.GraphQLObjectType({
  name: 'Character',
  fields: {
    id: { type: graphql.GraphQLString },
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
      resolve: (_notUsed: any, args: any) => {
        if(args.)
        return characterdata.find(character => character.id === args.id);
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