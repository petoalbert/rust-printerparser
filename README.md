## TODO
- [x] parsing / printing blender files using the same schema description
- [x] simple commit and checkout operations
- [ ] streaming parsing / printing
- [x] combinator that fails of the rest of the input is not empty (`complete` in nom?)
  - strangely makes the parser fail, but the blend file is OK? needs investigation, maybe unused data is dumped after the `ENDB` marker?
- [x] create lib.rs for the parser/printer stuff, and move json to an examples file
- [x] branching
- [ ] basic conflict resolution
- [ ] desktop app
- [ ] backend with user management and api

## "Research" ideas
- [ ] a printer that consumes the input (Would this require code duplication? How much?)
- [ ] Continuation passing style to eliminate copying?
- [ ] Do blocks have a name? Would be nice to display in log and conflict resolution messages
