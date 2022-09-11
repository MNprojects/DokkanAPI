# DokkanAPI
This API is designed to allow the user to query for character data for the mobile game Dragon Ball Z Dokkan Battle. Data is restricted to only LR and UR characters.

This API is in open beta, so if you have feature ideas now is the best time for feedback.

# Consuming the API
This is a GraphQL API with no authentication required.
Base URL: `https://dokkanapi.azurewebsites.net`

Go to the `/graphql` endpoint in your browser to see the GraphiQL interface which helps you understand the structure for queries.

For more information on how to consume a GraphQL API, visit the documentation on the specification: https://graphql.org/learn/  or https://graphql.org/graphql-js/graphql-clients/

There are only 2 main queries: character or characters. 



## Character Query
Character (singular) let's you get the data for a specific character using either their Id or Title as unique identifiers. These need to be exact matches.

### Example 1
Query for the id string "11900" will return the data:
```
{
  character(id: "11900") {
    name
    title
    id
  }
}
```

Result:
```
{
  "data": {
    "character": {
      "name": "Broly & Cheelai & Lemo",
      "title": "A New Life on Vampa",
      "id": "11900"
    }
  }
}
```
### Example 2
Query for the title instead (note you can use lowercase but it must be an exact match otherwise):
```
{
  character(title: "a new life on vampa") {
    name
    title
    id
  }
}
```

Result:
```
{
  "data": {
    "character": {
      "name": "Broly & Cheelai & Lemo",
      "title": "A New Life on Vampa",
      "id": "11900"
    }
  }
}
```

## Characters Query
Characters (plural) allows for searching of multiple characters based on most string-based data points. All search strings are looking to match on the substring and will ignore case to make it easier to use. 

### Example 1
Query for all characters where the name contains "raditz"
```
{
  characters(name: "raditz") {
    name
    title
    id
  }
}
```

Result:
```
{
  "data": {
    "characters": [
      {
        "name": "Raditz (Giant Ape)",
        "title": "Atrocious Crackdown",
        "id": "11078"
      },
      {
        "name": "Raditz",
        "title": "Cold-Hearted Warrior",
        "id": "11930"
      },
      {
        "name": "Vegeta (Kid) & Raditz (Kid)",
        "title": "Daring Planetary Invasion",
        "id": "11895"
      }
    ]
  }
}
```
### Example 2
Query all character that belong to *ALL* of the given categories:
```
{
  characters(categories: ["transformation boost", "terrifying conq"]) {
    name
    title
    id
  }
}
```

Result (concatenated): 
```
{
  "data": {
    "characters": [
      {
        "name": "Cooler",
        "title": "Almighty Cleave",
        "id": "1911"
      },
      {
        "name": "Frieza (Full Power)",
        "title": "Battle Prowess Fully Unleashed",
        "id": "12274"
      },

      ...

    ]
  }
}
```
### Example 3
I specifically designed searching for categories/links to require that a character matches all of the given criteria rather than introducing only meeting at least 1. This is because this use case can be done relatively simply by combining the results of multiple queries which can still be sent in the same request. However, be aware of duplicates in the results.
```
{
  coolersLeaderSkillFirstHalf: characters(categories: ["transformation boost", "terrifying conq"]) {
    ...comparisonFields
  }
  coolersLeaderSkillSecondHalf: characters(categories: ["transformation boost", "movie bosses"]) {
    ...comparisonFields
  }
}

fragment comparisonFields on Character {
  name
  title
  id
}
```

Result
```
{
  "data": {
    "coolersLeaderSkillFirstHalf": [
      {
        "name": "Cooler",
        "title": "Almighty Cleave",
        "id": "1911"
      },
      {
        "name": "Frieza (Full Power)",
        "title": "Battle Prowess Fully Unleashed",
        "id": "12274"
      },

      ...

      ],
    "coolersLeaderSkillSecondHalf": [
      {
        "name": "Cooler",
        "title": "Almighty Cleave",
        "id": "1911"
      },
      {
        "name": "Super Saiyan Broly",
        "title": "Endless Evolution of the Warrior Race",
        "id": "11600"
      },

      ...

    ]
  }
}      
```

### Example 4
Be aware of query strings that will unintentionally match multiple results
```
{
  characters(categories: ["super saiyan"]) {
    name
    title
    categories
  }
}
```
Result:
```
{
  "data": {
    "characters": [
      {
        "name": "Super Saiyan Trunks (Future)",
        "title": "A Gift From the Past",
        "categories": [
          "Hybrid Saiyans",
          "Future Saga",
          "Time Travelers",
          "Vegeta's Family",
          "Super Saiyans",
          "Androids/Cell Saga",
          "Bond of Master and Disciple",
          "Revenge",
          "Battle of Wits",
          "Entrusted Will",
          "Bond of Parent and Child"
        ]
      },

      ...
      
      {
        "name": "Super Saiyan God Goku",
        "title": "Accelerated Battle",
        "categories": [
          "Realm of Gods",
          "Pure Saiyans",
          "Movie Heroes",
          "Goku's Family",
          "Kamehameha",
          "Turtle School",
          "Miraculous Awakening",
          "Legendary Existence",
          "Bond of Friendship",
          "Accelerated Battle",
          "Battle of Fate",
          "Power Beyond Super Saiyan",
          "Bond of Parent and Child"
        ]
      },

      ...

      {
        "name": "Super Saiyan 3 Trunks (Teen)",
        "title": "All-New Power",
        "categories": [
          "Hybrid Saiyans",
          "Super Saiyan 3",
          "Time Travelers",
          "Vegeta's Family",
          "Bond of Master and Disciple",
          "Dragon Ball Heroes",
          "Battle of Wits",
          "Crossover",
          "Entrusted Will",
          "Power Beyond Super Saiyan"
        ]
      }
```
# Roadmap
This beta is to gather feedback to formulate the roadmap so anything here is just possibilities at this point. 
1. Nothing, it's perfect.
2. IDK give me some feedback


# Instructions for running locally
1. Clone the repo
2. From the project root, run the following command:
```
npm install
```
3. Run the following command:
```
npm run dev
```
(I've been a bit lazy in testing this from a clean install so let me know if there are any issues and I'll update these instructions.)

# Notes

1. All of the data comes from scraping the Dokkan Wiki: https://dbz-dokkanbattle.fandom.com/wiki/Dragon_Ball_Z_Dokkan_Battle_Wiki (that project is also on my GitHub).
I would appreciate it if people could correct any data they find is wrong either on the wiki or let me know about mistakes in the scraping process. The site can be really inconsistent in how it handles the same concept e.g., EZA or transformations. The /data folder in this project has the latest set of data in a json format. I will try to keep this API updated with latest characters are they are uploaded to the wiki but not guarantees.
2. This is my first real project using the JS/TS ecosystem and GraphQL. I also don't typically  personally release things that I expect others to use. Any feedback on code / style / performance / whatever is appreciated.