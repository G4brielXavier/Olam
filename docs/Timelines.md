# ⏳ Timelines

A **Timeline** is a structured set of Years and Ages. It represents the chronological flow of your world, with a clear beginning, middle, and end.

## 📖 Summary

- [⏳ Timelines](#-timelines)
  - [📖 Summary](#-summary)
  - [⚙️ How it Works](#️-how-it-works)
    - [Customization](#customization)
    - [Time Progression](#time-progression)
  - [✨ Creation \& Management](#-creation--management)
    - [`olam add`](#olam-add)
    - [`olam expun`](#olam-expun)
    - [`olam void`](#olam-void)
  - [📅 Year Logic](#-year-logic)
  - [🏛️ Age Logic](#️-age-logic)
    - [Age Structure](#age-structure)

---

## ⚙️ How it Works

A timeline in **Olam** consists of two main structures:
- **Year:** A specific point in time containing notes and events.
- **Age:** A span of time that contains a beginning, an end, and internal events.

### Customization
You are not stuck with Earth-standard terms. Using the `olam set` command, you can customize:
- **Year Name:** Change "Year" to "Ano", "Cycle", etc.
- **Age Name:** Change "Age" to "Era", "Aeon", etc.
- **Max Year:** Define the chronological limit of the timeline.

### Time Progression
When you add a Year or an Age, Olam tracks the progression:
1. Adding `olam year 10` sets the current marker to **10**.
2. Adding an Age with `olam age "Fire" --disso 30` means that age spans 30 years. 
3. The marker moves to **40** (10 + 30).

---

## ✨ Creation & Management

### `olam add`
Creates a new timeline within the current Space.
> **Note:** Must be in `INSPACE` stage.

```bash
olam add "Great War" -i
```
*`-i` creates and immediately immerses you into the timeline.*

### `olam expun`
*Aliases: `ex`* <br>
Permanently deletes a specific timeline.

```bash
olam expun <HASH>
```

### `olam void`
*Aliases: `vo`* <br>
Permanently deletes a specific Year or Age.

Removing a Year:
```bash
olam vo 10
```

Removing an Age:
```bash
olam vo --age "AgeName"
```

Removing a Year inside an Age (`< Events >`):
```bash
olam vo -a "AgeName" -y 10
```

Removing a note inside an Age (`< Between >`):
```bash
olam vo -a "AgeName" --pop
```

---

## 📅 Year Logic

The `year` command adds notes to a specific point in time. Olam is smart: if you add multiple notes to the same year, it groups them automatically.

```bash
olam year 0 "In the beginning, the world was silent."
olam year 0 "The first spark appeared."
```

**Preview Output:**
```text
-< Year 0 >-

    • In the beginning, the world was silent.
    • The first spark appeared.
```

---

## 🏛️ Age Logic

An **Age** represents a period. Use `--disso` (dissolve) to define how long that age lasts.

```bash
olam age "Age of Giants" --disso 500
```

### Age Structure
Ages have two internal containers:
1. **`< Between >`**: For general notes about the period without a specific date.
2. **`< Events >`**: For events anchored to a specific year *within* that age.

**Adding a "Between" note:**
```bash
olam age "Age of Giants" -b "Giants ruled the northern mountains."
```

**Adding a specific event inside an Age:**
```bash
# Syntax: olam year <AGE_NAME> <YEAR_OFFSET> <MESSAGE>
olam year -a "Age of Giants" 250 "The fall of the First King."
```
*Note: The year (250) must be within the Age's limit (0 to 500).*
