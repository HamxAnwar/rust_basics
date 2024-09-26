// In majority of cases, ownership is clear.
// But there are cases where we might have multiple owners.
// For example, we have a graph with multiple edges, pointing to the same node.
// Conceptually that node is owned by all the edges.
// To enable this multiple ownerships, we can use a reference counting (RC) smart pointer which will help track the number of references to a value.
// When there exist no more references, the value gets cleaned up.
// RCSP are used when we have to allocate a value on the heap and multiple parts of our program needs to access that value and we donot know, which part will use it last at compile time.
// In case we know which part uses it lastly, we can make this last part owner of the value and ownership rules will be applied.

fn main() {
    println!("Hello, world!");
}
