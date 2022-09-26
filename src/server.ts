let express = require('express');
let { graphqlHTTP } = require('express-graphql');
let { buildSchema } = require('graphql');
var graphql = require('graphql');
import e from "express";
import { GraphQLList, GraphQLObjectType, GraphQLString, GraphQLInt } from "graphql";
import { Character, Rarities, Classes, Types, Transformation } from "./character";

const characterData: any[] = require("../data/DokkancharacterData.json")

var transformationType = new graphql.GraphQLObjectType({
  name: 'Transformation',
  fields: {
    transformedName: { type: GraphQLString },
    transformedID: { type: GraphQLString },
    transformedClass: { type: GraphQLString },
    transformedType: { type: GraphQLString },
    transformedSuperAttack: { type: GraphQLString },
    transformedUltraSuperAttack: { type: GraphQLString },
    transformedPassive: { type: GraphQLString },

  }
})

var characterType = new graphql.GraphQLObjectType({
  name: 'Character',
  fields: {
    id: {
      type: graphql.GraphQLNonNull(graphql.GraphQLString),
      description: 'The id of the character'
    },
    name: { type: GraphQLString },
    title: { type: GraphQLString },
    maxLevel: { type: GraphQLInt },
    maxSALevel: { type: GraphQLInt },
    rarity: { type: GraphQLString },
    class: { type: GraphQLString },
    type: { type: GraphQLString },
    cost: { type: GraphQLInt },
    imageURL: { type: GraphQLString },
    leaderSkill: { type: GraphQLString },
    superAttack: { type: GraphQLString },
    ultraSuperAttack: { type: GraphQLString },
    passive: { type: GraphQLString },
    activeSkill: { type: GraphQLString },
    activeSkillCondition: { type: GraphQLString },
    links: { type: GraphQLList(GraphQLString) },
    categories: { type: GraphQLList(GraphQLString) },
    kiMeter: { type: GraphQLList(GraphQLString) },
    baseHP: { type: GraphQLInt },
    maxLevelHP: { type: GraphQLInt },
    freeDupeHP: { type: GraphQLInt },
    rainbowHP: { type: GraphQLInt },
    baseAttack: { type: GraphQLInt },
    maxLevelAttack: { type: GraphQLInt },
    freeDupeAttack: { type: GraphQLInt },
    rainbowAttack: { type: GraphQLInt },
    baseDefence: { type: GraphQLInt },
    maxDefence: { type: GraphQLInt },
    freeDupeDefence: { type: GraphQLInt },
    rainbowDefence: { type: GraphQLInt },
    kiMultiplier: { type: GraphQLString },
    transformations: { type: GraphQLList(transformationType) }
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
        ids: { type: graphql.GraphQLList(graphql.GraphQLString) },
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

        if (args.ids) {
          result = result.filter(character => args.ids.includes(character.id) )
          delete args.ids
        }

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