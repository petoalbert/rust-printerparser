## TODO
- [*] parsing / printing blender files using the same schema description
- [*] simple commit and checkout operations
- [ ] streaming parsing / printing
- [ ] combinator that fails of the rest of the input is not empty (`complete` in nom?)
- [ ] create lib.rs for the parser/printer stuff, and move json to an examples file
- [ ] branching with basic conflict resolution
- [ ] backend with user management and api

## "Research" ideas
- [ ] a printer that consumes the input (would this require code duplication? how much?)
- [ ] continuation passing style?
- [ ] do blocks have a name? Would be nice to display in log and conflict resolution messages