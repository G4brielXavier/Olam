# 🌌 Spaces

In **Olam**, a **Space** is the foundation of your worldbuilding. It acts as a sovereign container where you manage your timelines and chronology. You can think of a Space as a single project or a distinct fictional universe.

## 📖 Summary

- [🌌 Spaces](#-spaces)
  - [📖 Summary](#-summary)
  - [✨ Creating Spaces (`evoke`)](#-creating-spaces-evoke)
    - [⚡ Quick Immersion](#-quick-immersion)
  - [🗑️ Deleting Spaces (`efface`)](#️-deleting-spaces-efface)

---

## ✨ Creating Spaces (`evoke`)
*Aliases: `ev`*

The `evoke` command is used to bring a new Space into existence.

```bash
olam evoke "Space Name"
# or
olam ev "Space Name"
```

### ⚡ Quick Immersion
If you want to create a Space and dive into it immediately, use the `--immerse` or `-i` flag:

```bash
olam evoke "MySpace" -i
```

---

## 🗑️ Deleting Spaces (`efface`)
*Aliases: `ef`*

If a world no longer serves your story, you can use `efface` to permanently remove it.

> **Warning:** This action is irreversible and will delete all timelines within that Space.

```bash
olam efface <HASH>
# or
olam ef <HASH>
```

> **Note:** You can find the Space's `<HASH>` by running `olam ls` at the `OUTSPACE` stage.
