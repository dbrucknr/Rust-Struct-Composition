Traits represent abstractions or a "contract" that a "thing" needs to follow.

- Anything that implements one can do what is defined in the trait.
- They are similar to interfaces in other languages.

They are different from typical OOP patterns in that:

- No inheritance or class heirarchies. Traits describe capabilities, not "is this thing a" relationship.
- You compose behavioprs through traits and the `impl` blocks (composition).

There are 3 types of traits explored in this small project:

1. **Repositories** - are responsible for managing, behvaing as, or speaking to some external datasource.
2. **Services** - are responsible for providing communication definitions to any given repository struct or layer, and ideally would help handle errors from misaligned communication protocols or patterns between the two layers.
3. **Controllers** - are responsible for formatting the results a service layer provides and also help enforce parameter validation as they are an interface between a service definition and some input or event.
