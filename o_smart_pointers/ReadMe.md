- A pointer is a variable which stores a memory address.
- This memory address refers to some other data in memory.
- For example: references.
- But references, except pointing to a certain data inside the memory, have no other special capabilities which smart pointers have so no overhead.
- Smart pointers: They are data structures that act like a pointer but have meta data and extra capabilities.
- For example: a reference counting smart pointer.
- A reference counting smart pointer allows a single data to have multiple owners, keeping track of the owners and once there are no more owners, it cleans up the data.
- In many cases, smart pointers own the data that they point to unlike references which only borrow the data.
- Some of the smart pointers are vectors and strings.
- Mostly smart pointers are implemented using structs with dref and drop traits.
- dref trait: allows instances of the struct to be treated like references.
- It let us to write code that could work both with references and smart pointers at the same time.
- drop trait: Allows to customize the code that is run when an instance of our smart pointer goes out of scope.
- There are many smart pointers. We will be covering a few of them.

1. Box smart pointer.
