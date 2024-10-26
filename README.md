# 🗄️ Distributed Minecraft Server

*This repository is very much at a work in progress*. Do not expect a build anytime soon

- [🗄️ Distributed Minecraft Server](#️-distributed-minecraft-server)
  - [TODO](#todo)
  - [Running](#running)
  - [Backend](#backend)
    - [Providers](#providers)
    - [Parsers](#parsers)
    - [Misc](#misc)

## TODO

- [x] Mod Parsing
- [x] Plugin/Mod Backend Libraries
  - [x] Modrinth
  - [x] Hangar
  - [ ] Poggit
  - [ ] Curseforge
- [x] Maven Artifact Resolver
- [x] Server backend architecture
- [ ] Aternos Clone

## Running

> [!NOTE]
> If you are not planning to add Curseforge support, skip this.
>
> To omit Curseforge support, compile `dms-backend`
> without the `provider-curse` feature. More on customizing the backend [here](#backend)

Curseforge requires an API Key, so submit an application [here](https://forms.monday.com/forms/dce5ccb7afda9a1c21dab1a1aa1d84eb). Once you have an API key, add this to your `.env` file:

```env
CURSE_API_KEY=<key goes here>
```

Remember to escape any `$` characters as `dotenv` uses
this character for substitution.

## Backend

Compiling `dms-backend` only gives you a bare-bones server backend, only supporting minecraft vanilla. If this is not what you desire, you may download precompiled backend binaries or simply compile the backend with the following:

### Providers

> *Adds support for different mod hosting platforms*

- `provider-modrinth`: Enables Modrinth support
- `provider-curse`: Enables Curseforge support
- `provider-hangar`: Enables Hangar support
- `all-providers`: *Implies `parser-modrinth`, `parser-curse` and `parser-hangar`*

### Parsers

> *DMS can still work without these, just lacking some minor features*

- `parser-modpack`: Enables modpack parsing. *Without this, modpacks from any provider will not be supported*
- `parser-modjar`: Enables `.jar` file parsing. This adds dependency checking which can be useful when adding mods which are not provided.
- `all-parsers`: *Implies `parser-modpack` and `parser-modjar`*

### Misc

- `server-utils`: Please do not compile the backend without this :)
- `dms`: Implies *all providers, all parsers, and server utilities*. Precompiled binaries are built with this feature
