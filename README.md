# mdBook Gruvbox Theme

A [Gruvbox](https://github.com/morhetz/gruvbox) dark color scheme for [mdBook](https://rust-lang.github.io/mdBook/) documentation.

## Preview

The theme applies Gruvbox's warm, retro color palette to your mdBook documentation:

- **Background**: `#282828` (dark0)
- **Foreground**: `#ebdbb2` (light1)
- **Links**: `#fe8019` (bright orange)
- **Accents**: `#fabd2f` (bright yellow)
- **Code**: `#1d2021` background with syntax highlighting

## Installation

### Prerequisites

- [mdBook](https://rust-lang.github.io/mdBook/) 0.4+ installed

### Setup

1. **Get the Gruvbox theme**:

   Download and extract the latest release into your book directory:

   ```bash
   curl -sL https://github.com/bfowle/mdbook-gruvbox/releases/latest/download/mdbook-gruvbox.zip -o mdbook-gruvbox.zip
   unzip mdbook-gruvbox.zip -d your-book/
   rm mdbook-gruvbox.zip
   ```

2. **Configure your book.toml**:

   ```toml
   [output.html]
   theme = "gruvbox"
   ```

3. **Build documentation**:

   ```bash
   mdbook build
   ```

## CSS Variables

The theme defines all Gruvbox colors as CSS variables in `:root`, making customization easy. Each color is defined once and referenced throughout using `var(--gruvbox-*)`.

### Backgrounds
| Variable | Hex | Usage |
|----------|-----|-------|
| `--gruvbox-bg0-hard` | `#1d2021` | Sidebar, code blocks |
| `--gruvbox-bg0` | `#282828` | Main background |
| `--gruvbox-bg0-soft` | `#32302f` | Alternate table rows |
| `--gruvbox-bg1` | `#3c3836` | Searchbar, tables, quotes |
| `--gruvbox-bg2` | `#504945` | Hover states, scrollbar |
| `--gruvbox-bg3` | `#665c54` | Borders |
| `--gruvbox-bg4` | `#7c6f64` | — |

### Foregrounds
| Variable | Hex | Usage |
|----------|-----|-------|
| `--gruvbox-fg0` | `#fbf1c7` | — |
| `--gruvbox-fg1` | `#ebdbb2` | Primary text |
| `--gruvbox-fg2` | `#d5c4a1` | — |
| `--gruvbox-fg3` | `#bdae93` | — |
| `--gruvbox-fg4` | `#a89984` | Secondary text, icons |
| `--gruvbox-gray` | `#928374` | Comments |

### Accent Colors (Bright)
| Variable | Hex | Usage |
|----------|-----|-------|
| `--gruvbox-red` | `#fb4934` | Keywords |
| `--gruvbox-green` | `#b8bb26` | Strings |
| `--gruvbox-yellow` | `#fabd2f` | Active sidebar, types |
| `--gruvbox-blue` | `#83a598` | Attributes, variables |
| `--gruvbox-purple` | `#d3869b` | Numbers, literals |
| `--gruvbox-aqua` | `#8ec07c` | Inline code, selectors |
| `--gruvbox-orange` | `#fe8019` | Links, built-ins |

### Accent Colors (Dim)
| Variable | Hex | Usage |
|----------|-----|-------|
| `--gruvbox-red-dim` | `#cc241d` | — |
| `--gruvbox-green-dim` | `#98971a` | — |
| `--gruvbox-yellow-dim` | `#d79921` | Search highlight, meta |
| `--gruvbox-blue-dim` | `#458588` | — |
| `--gruvbox-purple-dim` | `#b16286` | — |
| `--gruvbox-aqua-dim` | `#689d6a` | — |
| `--gruvbox-orange-dim` | `#d65d0e` | — |

## Customization

### Changing the Link Color

Edit `gruvbox/css/variables.css` to use a different Gruvbox color for links:

```css
/* Use blue instead of orange for links */
--links: var(--gruvbox-blue);
```

### Changing the Active Sidebar Color

```css
/* Use aqua instead of yellow */
--sidebar-active: var(--gruvbox-aqua);
```

### Creating a Light Theme Variant

Override the Gruvbox variables in `:root` with the light palette values:

```css
:root {
    /* Swap backgrounds and foregrounds for light mode */
    --gruvbox-bg0: #fbf1c7;  /* was fg0 */
    --gruvbox-bg1: #ebdbb2;  /* was fg1 */
    --gruvbox-fg0: #282828;  /* was bg0 */
    --gruvbox-fg1: #3c3836;  /* was bg1 */
    /* Use faded accent colors for light backgrounds */
    --gruvbox-red: #9d0006;
    --gruvbox-green: #79740e;
    /* ... etc */
}
```

## Compatibility

- mdBook 0.4+
- Modern browsers (Chrome, Firefox, Safari, Edge)

## Credits

- [Gruvbox](https://github.com/morhetz/gruvbox) by Pavel Pertsev (morhetz)
- [mdBook](https://rust-lang.github.io/mdBook/) by the Rust community

## License

MIT License - see [LICENSE](LICENSE)
