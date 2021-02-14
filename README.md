Roadmap:
 √ basics: fn calls, fn dec, if/else, prints
 √ higher order functions (pass as arg, return)
 √ closures
 √ methods
 ⚙ infix operators :(
 ⚙ types
 - namespacing
 √ for .. in loops
   √ while loops
   √ option: some/none (kind of roughed it in for now)
 - "build" for loops
 - mut
 - pattern matching
 - generators?
 - dict literals
 - start writing standard library
 - garbage collection
 - structural sharing
 - imports, namespaces etc
 - Double type (i32s only supported currently)

An accessible functional-ish programming language. What Python did for procedural, OO programming, I want to do to functional programming.

## Design principles
- Readability as a top priority: the syntax should be expressive and quick to scan
- Prefer immutability as much as possible, providing tools to make it ergonomic
- Give the programmer as few footguns as possible

### Readability

I've chosen a Ruby-ish syntax because it looks clean. Lots of brackets and punctuation symbols make
a lanuage ugly.
Declarative programs are more readable. Declarative means tell the computer what you want it to do,
without having to worry so much about how to do it. SQL is a great example of this.
A primary goal is to make the language as declarative as possible. I will continue experimenting with
ways to make this possible.

### Prefer immutability

...but keep it ergonomic.

In my experience, functional languages encourage you to write lots of recursive functions. Or at least when
I worked with SML in college, my professor did. To process a list, you'd do something like splitting it into
the first element and then a list of the remaining elements, operate on the first element, and recur, saving
your output as a parameter.

In Haskell there are monads that kind of make stuff like this easier, but monads are notoriously confusing.
And monads aside, while I find Haskell to be fascinating in many ways, it seems impractical to be because
the code is so _dense_. It has a complex type system, and lots of novel concepts (monads, monoids) to grapple
with.

My idea is to keep the immutability part around but provide more iterative structures that are easier to
debug and reason about.

(Flesh this out more)
