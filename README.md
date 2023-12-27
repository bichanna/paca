<div align="center">
  <h1><b>The Paca Programming Language</b></h1>
  <p>
    <strong>An experimental nibbler</strong>
  </p>
</div>

Nothing is implemented yet, but here are some examples.

```rust
export Either, Option;

enum Either<L, R> {
    Left(L),
    Right(R)
}

enum Option<T> {
    Some(T),
    None,
}
```

```rust
import std::collections::Array;

def main() void {
    let names: []str = Array::init<str>("Nobu", "June");
    Array::append(names, "Shivam", "Arya", "Brogan", "Erin");
    let popped: Option<str> = Array::pop(names);
    println(popped); // Option->Some("Erin")
    let tenth: Option<str> = Array::get(names, 99);
    println(tenth); // Option->None
    println(names); // ["Nobu", "June", "Shivam", "Arya", "Brogan"]
    println(Array::length(names)); // 5
}
```

```rust
def main() void {
    let a: Option<int> = Option::Some(123);
    let b = a->unwrap(); // `b: int = 123`
    
    let c: Option<int> = Option::None;
    let d = c->unwrap_or(321); // `d: int = 321`
}
```

```rust
import std::fs::read_file;
import std::io;

def main() void {
    let file_content: Either<str, io::Error> = read_file("./whatever.txt");
    match file_content {
        Either::Left(content) => println(content),
        Either::Right(err) => println(err),
    }
}
```

```rust
import std::hash::Hashable;
import std::cmp::Equal;
import std::convert::From;
import std::collections::tuple::(first, second);
import std::collections::Array;

struct Entry<K: Hashable, V> {
    $key: K,
    $val: V,
}

struct HashMap<K: Hashable, V> {
    entries: []Option<Entry<K, V>>,
    length: int,
}

impl methods for Entry<K: Hashable, V> {
    def init(key: K, val: V) Self {
        return Self {
            key => key,
            val => val,
        };
    }
}

impl From<(K: Hashable, V)> for Entry {
    def from(value: (K: Hashable, V)) Self {
        return Self::init(
            first(value),
            second(value)
        );
    }
}

impl methods for HashMap<K: Hashable, V> {
    def init(*init_raw_entries: [](K, V)) Self {
        let entries = Array::map(init_raw_entries) { (raw_entry) = Entry::from(raw) };
        let length = Array::length(entries);
        return Self {
            entries => entries,
            length => length,
        };
    }
    
    def length(self) int {
        return self->length;
    }
    
    def get(self, key: K) Option<V> {
        let result: Option<V> = Option::None;
        Array::for_each(self->entries) { (quit, entry) =>
            if entry->key->hash() == key->hash() {
                result = Option::Some(entry->val);
                quit();
            }
        )};
        return result;
    }
    
    def put(self, key: K, val: V) void {
        let exit = false;
        Array::for_each(self->entries) { (quit, entry) =>
            if entry->key->hash() == key->hash() {
                entry->val = val;
                exit = true;
                quit();
            }
        };
        if exit {
            return;
        }
        let new_entry = Entry::init(key, val);
        Array::append(self->entries, new_entry);
        self->length += 1;
    }
    
    def keys(self) []K {
        return Array::map(self->entries) { (entry) = entry->key };
    }
    
    def vals(self) []V {
        return Array::map(self->entries) { (entry) = entry->val }
    }
    
    // other methods...
}
```