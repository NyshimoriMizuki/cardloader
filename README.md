# Card Loader

> **Note:** Most of the shown GUI content is in **PT-BR**.

A small **study project** built to practice:

- Building a UI with [iced](https://iced.rs) widgets.
- Loading Files from disk at runtime (used [`rfd`](https://docs.rs/rfd) for this).
- Applying a bit style to a widgets.

It just loads a JSON "card" that has a title, a description and a numeric rating.

---

## What it does

1. Opens a native file picker (defaulting to the `samples/` directory).
2. Reads the chosen `.json` file and deserializes it into an `Item`.
3. Renders a **card** with:
   - The item's **name**.
   - A **rating bar** whose color encodes the score:
     - 🔴 Red — `rate < 2.0`
     - 🟡 Amber — `2.0 ≤ rate < 3.5`
     - 🟢 Green — `rate ≥ 3.5`
   - The item's **description**.
4. If no file is selected or the JSON fails to parse, the card falls back to a **default placeholder** (`<name>`, `<description>`, `rate = 0`).

> The rating is always clamped to `0.0..=5.0` before being stored, so a malformed file cannot break the layout.

---

## Expected JSON format

The loader expects a single object matching the `Item` struct, like:

```json
{
  "name": "Hollow Knight",
  "rate": 4.5,
  "description": "Explorar um reino subterrâneo cheio de mistérios, inimigos e segredos."
}