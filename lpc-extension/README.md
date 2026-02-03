# LPC Language Support for VS Code

This extension provides comprehensive language support for LPC (Lars Pensjö C) files used in MUD driver libraries, specifically designed for the AMLP (Advanced MUD LPC Platform) driver.

## Features

- **Syntax Highlighting** for LPC files (.lpc extension)
  - LPC-specific keywords (inherit, include, varargs, nomask, etc.)
  - Scope resolution operator (::)
  - Room/object setup functions
  - All 24 AMLP driver efuns
  
- **Code Snippets** for common LPC patterns
  - Room and object templates
  - Function declarations
  - Control structures
  - Common operations
  
- **Language Features**
  - Comment toggling (// and /* */)
  - Bracket matching and auto-closing
  - Code folding
  - Proper LPC syntax validation

## Installation

### Method 1: Direct Installation (Recommended)

1. **Download and Extract** the extension archive
2. **Copy** the `lpc-extension` folder to your VS Code extensions directory:
   - **Windows:** `%USERPROFILE%\.vscode\extensions\`
   - **macOS/Linux:** `~/.vscode/extensions/`
3. **Restart** VS Code completely
4. **Rename** your LPC library files from `.c` to `.lpc`
5. Open any `.lpc` file - you should see LPC syntax highlighting!

### Method 2: Development Mode (F5)

1. Open the `lpc-extension` folder in VS Code
2. Press `F5` to launch Extension Development Host
3. Open your AMLP library in the new window
4. Test with `.lpc` files

## Usage with AMLP Driver

Your project structure should look like:

```
your-mud-project/
├── driver/              # C source files (keep as .c)
│   ├── compiler.c       # Regular C syntax highlighting
│   ├── lexer.c          # Regular C syntax highlighting
│   └── vm.c             # Regular C syntax highlighting
└── library/             # LPC library files (rename to .lpc)
    ├── std/
    │   ├── room.lpc     # LPC syntax highlighting
    │   ├── object.lpc   # LPC syntax highlighting
    │   └── player.lpc   # LPC syntax highlighting
    ├── domains/
    │   └── start/
    │       └── room/
    │           └── void.lpc  # LPC syntax highlighting
    └── master.lpc       # LPC syntax highlighting
```

## Code Snippets

Type these prefixes and press Tab to expand:

### Room Creation
**Trigger:** `room`
```lpc
// Room Name

#include <globals.h>

inherit DIR_STD + "/room";

void create() {
    ::create();
    
    set_id("room_id");
    set_short("Short Description");
    set_long(
        "Long description of the room.\n"
    );
    
    // add_exit("direction", "path/to/room");
}
```

### Object Creation
**Trigger:** `object`
```lpc
// Object Name

#include <globals.h>

inherit DIR_STD + "/object";

void create() {
    ::create();
    
    set_id("object_id");
    set_short("a short description");
    set_long("A longer description.");
}
```

### Other Snippets
- `inherit` - Add inherit statement
- `include` - Add include statement
- `create` - Create function with super call
- `func` - Void function template
- `funcr` - Function with return value
- `exit` - Add room exit
- `if`, `for`, `foreach`, `switch` - Control structures
- `mapping`, `array` - Data structure declarations
- `to` - this_object()
- `tp` - this_player()
- `clone` - clone_object()
- `call` - call_other()

## Syntax Highlighting

### LPC-Specific Keywords
- **Control:** if, else, while, for, foreach, switch, etc.
- **Modifiers:** static, private, protected, public, varargs, nomask, nosave
- **Special:** inherit, include
- **Operators:** :: (scope resolution)

### AMLP Driver Efuns
All 24 built-in functions from your driver are recognized:

**String Functions:** strlen, substring, explode, implode, upper_case, lower_case, trim

**Array Functions:** sizeof, arrayp, sort_array, reverse_array

**Math Functions:** abs, sqrt, pow, random, min, max

**Type Checking:** intp, floatp, stringp, objectp, mappingp

**I/O Functions:** write, printf

### Common Room/Object Functions
Functions like `set_short`, `set_long`, `set_id`, `add_exit`, `query_short`, `query_long` are highlighted as they're commonly used in MUD development.

## Customization

### Adding Your Own Keywords

Edit `syntaxes/lpc.tmLanguage.json`:

```json
"keywords": {
  "patterns": [
    {
      "name": "keyword.control.lpc",
      "match": "\\b(if|else|your_custom_keyword)\\b"
    }
  ]
}
```

### Adding Custom Snippets

Edit `snippets/lpc.json`:

```json
"Your Snippet Name": {
  "prefix": "trigger",
  "body": [
    "code line 1",
    "code line 2"
  ],
  "description": "What this snippet does"
}
```

## Project Integration

### Renaming Files in Bulk

**Linux/macOS:**
```bash
cd /path/to/your/amlp-library
find . -name "*.c" -type f -exec sh -c 'mv "$1" "${1%.c}.lpc"' _ {} \;
```

**Windows PowerShell:**
```powershell
cd C:\path\to\your\amlp-library
Get-ChildItem -Recurse -Filter "*.c" | Rename-Item -NewName {$_.Name -replace '\.c$','.lpc'}
```

**Note:** Only rename files in your library directory, not your driver!

## Why .lpc Extension?

**Benefits:**
- ✅ Clear separation between C driver code and LPC library code
- ✅ Proper syntax highlighting for LPC-specific constructs
- ✅ No confusion between what's compiled (C) vs interpreted (LPC)
- ✅ Better tooling support and IDE features
- ✅ Follows modern best practices for language-specific extensions

## Troubleshooting

### Extension Not Activating
1. Verify the extension is in `.vscode/extensions/lpc-extension/`
2. Restart VS Code completely (not just reload window)
3. Check file extension is `.lpc` not `.c`

### Snippets Not Working
1. Type the trigger word (e.g., `room`)
2. Press `Tab` (not Enter)
3. If no suggestions appear, press `Ctrl+Space` to trigger IntelliSense

### Wrong Language Selected
1. Click the language indicator in bottom-right
2. Select "LPC" from the list
3. Or use "Configure File Association for '.lpc'" to make it permanent

## AMLP Driver Compatibility

This extension is specifically designed for the AMLP driver:
- **Driver Repository:** https://github.com/Thurtea/amlp-driver
- **Library Repository:** https://github.com/Thurtea/amlp-library

All efuns and patterns match the AMLP driver implementation (Phase 6 complete - 220 tests passing).

---

**Author:** Built for the AMLP MUD Platform  
**License:** Free for use with AMLP and other LPC MUD projects  
**Support:** For issues or feature requests, file on the AMLP driver repo

