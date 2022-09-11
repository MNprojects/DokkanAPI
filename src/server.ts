let express = require('express');
let { graphqlHTTP } = require('express-graphql');
let { buildSchema } = require('graphql');
var graphql = require('graphql');
import e from "express";
import { GraphQLList, GraphQLString } from "graphql";
import { Character, Rarities, Classes, Types, Transformation } from "./character";

const characterData: any[] = require("../data/DokkancharacterData.json")

var characterType = new graphql.GraphQLObjectType({
  name: 'Character',
  fields: {
    id: {
      type: graphql.GraphQLNonNull(graphql.GraphQLString),
      description: 'The id of the character'
    },
    name: { type: graphql.GraphQLString },
    title: { type: graphql.GraphQLString },
    maxLevel: { type: graphql.GraphQLInt },
    maxSALevel: { type: graphql.GraphQLInt },
    rarity: { type: graphql.GraphQLString },
    class: { type: graphql.GraphQLString },
    type: { type: graphql.GraphQLString },
    cost: { type: graphql.GraphQLInt },
    imageURL: { type: graphql.GraphQLString },
    leaderSkill: { type: graphql.GraphQLString },
    superAttack: { type: graphql.GraphQLString },
    ultraSuperAttack: { type: graphql.GraphQLString },
    passive: { type: graphql.GraphQLString },
    activeSkill: { type: graphql.GraphQLString },
    activeSkillCondition: { type: graphql.GraphQLString },
    links: { type: graphql.GraphQLList(GraphQLString) },
    categories: { type: graphql.GraphQLList(GraphQLString) },
    kiMeter: { type: graphql.GraphQLList(GraphQLString) },
    baseHP: { type: graphql.GraphQLInt },
    maxLevelHP: { type: graphql.GraphQLInt },
    freeDupeHP: { type: graphql.GraphQLInt },
    rainbowHP: { type: graphql.GraphQLInt },
    baseAttack: { type: graphql.GraphQLInt },
    maxLevelAttack: { type: graphql.GraphQLInt },
    freeDupeAttack: { type: graphql.GraphQLInt },
    rainbowAttack: { type: graphql.GraphQLInt },
    baseDefence: { type: graphql.GraphQLInt },
    maxDefence: { type: graphql.GraphQLInt },
    freeDupeDefence: { type: graphql.GraphQLInt },
    rainbowDefence: { type: graphql.GraphQLInt },
    kiMultiplier: { type: graphql.GraphQLString },
    transformations: { type: graphql.GraphQLList(GraphQLString) }
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
          return characterData.find(character => character.id === args.id);
        }

        if (args.title) {
          return characterData.find(character => character.title.toLowerCase() === args.title.toLowerCase());
        }
      }
    },
    characters: {
      type: new GraphQLList(characterType),
      args: {
        name: { type: graphql.GraphQLString },
        title: { type: graphql.GraphQLString },
        rarity: { type: graphql.GraphQLString },
        class: { type: graphql.GraphQLString },
        type: { type: graphql.GraphQLString },
        imageURL: { type: graphql.GraphQLString },
        leaderSkill: { type: graphql.GraphQLString },
        superAttack: { type: graphql.GraphQLString },
        ultraSuperAttack: { type: graphql.GraphQLString },
        passive: { type: graphql.GraphQLString },
        activeSkill: { type: graphql.GraphQLString },
        activeSkillCondition: { type: graphql.GraphQLString },
        kiMultiplier: { type: graphql.GraphQLString },
        categories: { type: graphql.GraphQLList(graphql.GraphQLString) },
        links: { type: graphql.GraphQLList(graphql.GraphQLString) },

      },
      // @ts-ignore
      resolve: (_notUsed, args) => {
        let result: Character[] = characterData;
        // only works for string based queries because of toLowerCase
        Object.entries(args).forEach(arg => {
          if (typeof (arg[1]) === 'string') {
            result = result.filter(character =>
              // @ts-ignore
              character[arg[0]]?.toLowerCase().includes(arg[1].toLowerCase()))
          }
          else if (typeof (arg[1]) === 'object') {
            // @ts-ignore
            result = result.filter(character => compareCharacterLists(arg[1], character[arg[0]]))
          }
        });
        return result
      }
    }
  }
});

function compareCharacterLists(searchForList: [], characterList: string[]) {
  // character needs to be in every category given
  return searchForList.every(searchListItem => {
    // only one category needs to match the search category
    return characterList.some(characterListItem => {
      // may change this to be exact match rather than includes. Seems more flexible right now
      // @ts-ignore
      return characterListItem.toLowerCase().includes(searchListItem.toLowerCase())
    })
  });
}


var schema = new graphql.GraphQLSchema({ query: queryType });

let app = express();
app.use('/graphql', graphqlHTTP({
  schema: schema,
  graphiql: true,
}));

app.listen(8080);
console.log('Running a GraphQL API server at http://localhost:8080/graphql');