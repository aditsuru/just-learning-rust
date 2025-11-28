That "Rust Book Amnesia" is extremely common. It happens because reading about ownership is different from _feeling_ the compiler yell at you about it.

Since you want to go deep on Data Structures, we are going to spend extra time here. This is where the architecture of your application lives.

We will split this into two parts:

1.  **Structs** (and how they interact with Ownership).
2.  **The String Dilemma** (`String` vs `&str`) - This usually hits right when you start using Structs.

---

### Part 1: Structs (Data Shapes)

You know structs from C. In Rust, they are similar, but they replace Classes. Rust has no `class` keyword. Instead, we have **Structs** (data) and **Impls** (behavior).

#### The Rule of Struct Ownership

If a Struct owns data (like a `String`), and the Struct moves, the data moves with it.

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
    };

    let user2 = user1; // MOVES user1 to user2.

    // println!("{}", user1.email); // ERROR: user1 is gone.
}
```

#### Adding Behavior (`impl`)

In JS/Python, you put methods inside the class. In Rust, we separate them.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

// separate block for logic
impl Rectangle {
    // 1. A Method (Takes &self)
    // We use &self because we want to read the data, not consume the struct.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 2. An Associated Function (Static method)
    // Does NOT take self. Called like Rectangle::square(10)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

**Key Note on `&self`:**

- `fn area(&self)` -> Borrows the struct (Read-only). Most common.
- `fn grow(&mut self)` -> Mutably borrows (Changes the struct).
- `fn destroy(self)` -> Takes ownership (Consumes the struct). You rarely do this unless you want to transform the struct into something else and prevent the original from being used.

---

### Part 2: The String vs &str War

This is the #1 confusion point when defining Structs.

You will often ask: _"Should I put `String` or `&str` inside my struct?"_

#### The Types

1.  **`String` (The Owner)**

    - Lives on the **Heap**.
    - It owns the text data.
    - It can grow, shrink, change.
    - **Rule of Thumb:** Use this inside Structs. You want the Struct to _own_ its name.

2.  **`&str` (The Borrower / String Slice)**
    - It is a pointer (reference) to some text owned by someone else (or hardcoded in the binary).
    - It is immutable.
    - **Rule of Thumb:** Use this for function arguments (`fn print(s: &str)`).

#### Why this crashes

If you try to use `&str` in a struct without advanced knowledge (Lifetimes), it fails.

```rust
// ERROR: Missing lifetime specifier
struct User {
    username: &str,
}
```

**Why?** Rust asks: _"This struct holds a reference to a string. Who owns that string? What if the owner dies but this Struct is still alive?"_

To keep it safe, Rust bans this by default.
**The Pro Tip:** Always use `String` inside structs until you are an expert optimization engineer.

---

### 🛑 Mini-Challenge 4 (Structs & Strings)

We are going to build a simple **Book** system.

**Requirements:**

1.  Create a struct `Book` with fields: `title` (String), `pages` (u32).
2.  Implement a method `format_summary` that returns a String like `"Title: [title], Pages: [pages]"`.
3.  Implement a method `tear_page` that decreases `pages` by 1.

**Boilerplate to start:**

```rust
struct Book {
    // TODO: Define fields
}

impl Book {
    // TODO: format_summary (&self)

    // TODO: tear_page (&mut self)
}

fn main() {
    // 1. Create a book "Rust 101", 100 pages.
    // 2. Tear a page.
    // 3. Print the summary.
}
```

**Watch out for:**

- When creating the String in `format_summary`, you might need the `format!` macro (works like `println!` but returns a string).
  - Usage: `format!("Title: {}, Pages: {}", self.title, self.pages)`
- Make sure your `main` variable is mutable if you plan to tear a page!

Give it a shot.
