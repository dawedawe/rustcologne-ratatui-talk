---
theme : "night"
transition: "slide"
highlightTheme: "monokai"
logoImg: "logo.png"
slideNumber: false
title: "Ratatui"
---

# Ratatui

> A Rust crate for cooking up Terminal User Interfaces

<small>dawe / rust.cologne</small>

---

## Project Basics

- fork/successor of tui-rs
- well [documented](https://ratatui.rs/)
- well maintaned

---

## Terminal Basics

- What's a TUI?
- 2 buffers/screens, that's how `vim`, `less`, ... work
- [raw mode](https://en.wikipedia.org/wiki/POSIX_terminal_interface#History)
- restore needed at end of tui app

---

## Skelethon of a Ratatui app

- initialize the terminal
- a main loop that:
  - draws the UI
  - handles input events
- restore the terminal state

---

## initializing the terminal
```rust
fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}
```

---

## the main loop

```rust
fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let some_widget = Paragraph::new("Hello rust.cologne");
            frame.render_widget(some_widget, frame.area());
        })?;

        if matches!(event::read()?, Event::Key(key) if key.kind == KeyEventKind::Press) {
            break Ok(());
        }
    }
}
```

---

## restoring the terminal

```rust
ratatui::restore();
```

---

# Layout

- splits terminal space into areas
- [constraint-based](https://ratatui.rs/concepts/layout/) approach

---

# Widget Overview
- fills an area of a layout
- basic needs covered by Ratatui built-in [widgets](https://ratatui.rs/concepts/widgets/)
- more available as crates

---

## Widget trait

```rust
pub trait Widget {
    /// Draws the current state of the widget in the given buffer.
    fn render(self, area: Rect, buf: &mut Buffer);
}
```

---

# Styling

- fore-/background colors
- font modifiers

---

## Architectures / Application Patterns

- [Elm](https://en.wikipedia.org/wiki/Elm_(programming_language)#The_Elm_Architecture_(TEA_pattern))
- [Component](https://ratatui.rs/concepts/application-patterns/component-architecture/)
- [Flux](https://ratatui.rs/concepts/application-patterns/flux-architecture/)

---

# [Backends](https://ratatui.rs/concepts/backends/comparison/)

default:  
- crossterm  

alternatives:
- termion
- termwiz

---

## Rendering

- [Immediate Mode](https://ratatui.rs/concepts/rendering/)
  - redraw UI for every frame
  - no permanent widget objects
- but: final flush of buffer to terminal is diff-based

---

# Conclusion & Future

- fun
- good way to go beyond the basics of Rust
- ratzilla for TUIs in the [browser](https://github.com/orhun/ratzilla)

---

# References

- [ratatui](https://ratatui.rs/)
- [EuroRust](https://www.youtube.com/watch?v=hWG51Mc1DlM)
- [Developer Voices](https://www.youtube.com/watch?v=CNVmIocyDOQ&t=1s)
- [slides](https://www.github.com/dawedawe/rustcologne-ratatui-talk)