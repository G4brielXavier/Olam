# ⚓ Navigation & State

Navigation in **Olam** is built around a "depth" system. Instead of passing long paths to every command, you "immerse" yourself into a context and execute commands naturally within that scope.

## 🎚️ Understanding Stages

Olam tracks your current position in the hierarchy using three stages:

| Stage | Meaning | Available Commands |
| :--- | :--- | :--- |
| `OUTSPACE` | Surface level. | Space management (`evoke`, `ls`, `immerse`). |
| `INSPACE` | Inside a specific Space. | Timeline management (`add`, `ls`, `immerse`). |
| `INTIMELINE` | Inside a specific Timeline. | Chronology management (`year`, `age`, `event`, `tl`). |

> **Pro Tip:** Use `olam stage` at any time to check exactly where you are.

---

## 📥 `olam immerse`
*Aliases: `im`*

Use **Immerse** to go deeper into your world structure. 

- **From `OUTSPACE`**: Dive into a specific `Space`.
- **From `INSPACE`**: Dive into a specific `Timeline`.
- **From `INTIMELINE`**: You are already at maximum depth.

```bash
olam immerse <HASH>
# or
olam im <HASH>
```

### ⚡ Quick Dive
You can create a Space or Timeline and jump into it immediately using the `--immerse` or `-i` flag:

```bash
olam evoke "MySpace" -i
olam add "MyTimeline" -i
```

---

## 📤 `olam emerge`
*Aliases: `em`*

Use **Emerge** to go back up one level in the hierarchy.

- **From `INTIMELINE`**: Return to the parent `Space` (Stage: `INSPACE`).
- **From `INSPACE`**: Return to the surface (Stage: `OUTSPACE`).

```bash
olam emerge
# or
olam em
```

### 🌊 Surface Jump
To return directly to the surface (`OUTSPACE`) regardless of your current depth, use:

```bash
olam emerge --out
```

---

## 🔍 Contextual Commands

### `olam ls`
The `ls` command is context-aware. It lists what is relevant to your current stage:
- **In `OUTSPACE`**: Lists all available **Spaces**.
- **In `INSPACE`**: Lists all **Timelines** within the current Space.

```bash
olam ls
```

### `olam stage`
Shows your current stage (`OUTSPACE`, `INSPACE`, or `INTIMELINE`) and the path of your current immersion.

```bash
olam stage
```

---

## ⚙️ `olam settings`
*Aliases: `set`*

Used to modify the metadata of your current context.

### In Space (`INSPACE`)
Modify the global container:
- `--title`: Rename the space.
- `--desc`: Update the space description.

```bash
olam set --title "NewName" --desc "A new description for my world."
```

### In Timeline (`INTIMELINE`)
Configure the chronology rules and custom naming:
- `--title`: Rename the timeline.
- `--desc`: Update the description.
- `--yearname`: Change the unit name for years (e.g., "Cycle").
- `--agename`: Change the unit name for ages (e.g., "Era").
- `--max`: Set the maximum year/unit limit for this timeline.

**Example: Localizing and setting limits**
```bash
# Changing "Year" to "Ano" and setting a 4000-year limit
olam set --yearname "Ano" --max 4000
```