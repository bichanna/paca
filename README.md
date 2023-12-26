<div align="center">
  <h1><b>The Paca Programming Language</b></h1>
  <p>
    <strong>An experimental nibbler</strong>
  </p>
</div>

Nothing is implemented yet, but here are some examples.

```rust
type option<T> = either<T, none>;
```

```rust
import std.collections.array;

def main() -> none {
    let names: [string] = array->init<string>("Nobu", "June");
    array->append(names, "Shivam", "Arya", "Brogan", "Erin");
    let popped: option<string> = array->pop(names);
    println(popped); // either(left => "Erin")
    let tenth: option<string> = array->get(names, 99);
    println(tenth); // either(right => none)
    println(names); // ["Nobu", "June", "Shivam", "Arya", "Brogan"]
    println(array->length(names)); // 5
}
```

```rust
def main() -> none {
    let a: option<int> = left(123);
    let b = unwrapl(a); // `b: int = 123`
    
    let c: option<int> = right(none);
    let d = unwrapl_or(c, 321); // `d: int = 321`
    let e = unwrapr(c) // `e: none = none`
    let f = unwrapl(c); // The code panics and exits the whole program.
}
```

```rust
import std.fs->read_file;
import std.io->Error;

def main() -> none {
    let file_content: either<string, Error> = read_file("./whatever.txt");
    if has_left(file_content) {
        println(unwrapl(file_content));
    } else {
        println(unwrapr(file_content)->to_string());
    }
}
```

```rust
import std.hash->Hashable;
import std.cmp->Equal;
import std.convert->From;
import std.collections.tuple->first, second;
import std.collections.array;

export HashMap;

struct Entry<K: Hashable, V> {
    $key: K,
    $val: V,
}

struct HashMap<K: Hashable, V> {
    entries: [option<Entry<K, V>>],
    length: int,
}

impl methods for Entry<K: Hashable, V> {
    def init(key: K, val: V) -> Self {
        return Self {
            key => key,
            val => val,
        };
    }
}

impl From<(K: Hashable, V)> for Entry {
    def from(value: (K: Hashable, V)) -> Self {
        return Self->init(
            first(value),
            second(value)
        );
    }
}

impl methods for HashMap<K: Hashable, V> {
    def init(*init_raw_entries: [(K, V)]) -> Self {
        let entries = array->map(init_raw_entries) {(raw_entry) ->
            return Entryfrom(raw)
        };
        let length = array->length(entries);
        return Self {
            entries => entries,
            length => length,
        };
    }
    
    def length(self) -> int {
        return self.length;
    }
    
    def get(self, key: K) -> option<V> {
        let result: option<V> = right(none);
        array->for_each(self.entries) {(quit, entry) -> 
            if entry->key->hash() == key->hash() {
                result = left(entry->val);
                quit();
            }
        )};
        return result;
    }
    
    def put(self, key: K, val: V) -> none {
        let exit = false;
        array->for_each(self.entries) {(quit, entry) ->
            if entry->key->hash() == key->hash() {
                entry->val = val;
                exit = true;
                quit();
            }
        };
        if exit {
            return;
        }
        let new_entry = Entry->init(key, val);
        array->append(self.entries, new_entry);
        self.length += 1;
    }
    
    // other methods...
}
```