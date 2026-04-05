# 🎭 Full Usage Example: The History of Westeros

This guide demonstrates a complete workflow in **Olam** using the **A Song of Ice and Fire** (Game of Thrones) universe as a base.

## 1. Initializing the Universe

First, we create our "Space" for the world of Westeros.

```bash
# Create the space 'Westeros' and immerse in it
olam evoke "Westeros" -i
```

## 2. Creating a Timeline

Let's create the primary timeline: "The Iron Throne Era".

```bash
# Create the timeline and immerse
olam add "Iron Throne Chronology" -i
```

## 3. Customizing the Calendar

In Westeros, years are counted as **BC** (Before Conquest) and **AC** (After Conquest). Let's adjust our naming.

```bash
# Rename 'Year' to 'AC' (After Conquest)
olam set --yearname "AC" --max 8000
```

## 4. Recording the Conquest (Years)

Let's anchor the beginning of the Targaryen dynasty.

```bash
# Year 0: Aegon's Coronation
olam year 0 "Aegon I Targaryen is crowned King of the Six Kingdoms."
olam year 1 "Construction of the Red Keep begins in King's Landing."
```

## 5. Defining a Dynasty (Ages)

The Targaryen Reign is a perfect example of an **Age** in Olam. Let's define the first 130 years leading up to the civil war.

```bash
# Define the 'Targaryen Era' with a 130-year span (until the Dance)
olam age "Targaryen Era" --disso 130
```

### Adding Lore to the Era
We can add general notes about the dynasty and specific internal events.

```bash
# A general note about the power of the dragons
olam age "Targaryen Era" -b "Dragons were used to maintain the King's Peace across the realm."

# A specific event: The birth of Prince Jaehaerys
olam year -a "Targaryen Era" 34 "Prince Jaehaerys Targaryen is born."
```

## 6. Timeline Progression

After the Age is dissolved or moves forward, we can record the later history.

```bash
# This event happens 150 years later
olam year 281 "The Year of the False Spring and the Tourney at Harrenhal."
olam year 283 "Robert's Rebellion ends; Robert Baratheon takes the Throne."
```

## 7. Previewing the History

Now, let's see how Olam renders the history of the Seven Kingdoms.

```bash
olam tl
```

**Output Preview:**
```text
(olam) ~Westeros.IronThroneChronology

-< AC 0 >-

    • Aegon I Targaryen is crowned King of the Six Kingdoms.

-< AC 1 >-

    • Construction of the Red Keep begins in King's Landing.

-< Targaryen Era - AC 2 >-

    < Between >

        • Dragons were used to maintain the King's Peace across the realm.

    < Events >

        -< AC 34 >- 
            
            • Prince Jaehaerys Targaryen is born.

-< Targaryen Era Dissolved - AC 130 >-

-< AC 281 >-

    • The Year of the False Spring and the Tourney at Harrenhal.

-< AC 283 >-

    • Robert's Rebellion ends; Robert Baratheon takes the Throne.
```

---

## 🚀 Key Commands Used:
- `evoke -i`: Create and enter the Westeros Space.
- `set --yearname "AC"`: Customize the year unit to After Conquest.
- `year <N> <MSG>`: Record specific Targaryen milestones.
- `age <NAME> --disso <N>`: Create the Targaryen Era span.
- `tl`: Visualize the history of the Iron Throne.