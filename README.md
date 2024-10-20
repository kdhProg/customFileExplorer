
<p align="center">
<img width="700" src="./static/banner/main_banner.png"/>
</p>
<br>
<br>
<p align="center">
<b>
  Simple File explorer Project by using Tauri and SvelteKit
</b>
</p>
<br>
<br>

# Overview

we developed a file explorer project with key strengths such as fast search speed, customizable themes, and multi-language support. This tool allows users to efficiently manage and locate files while offering a personalized user experience through theme and language settings.

## Main Features

- Search with Various options
- Selectable Util buttons & UI sets

## Feature Details

### Utils
- All utility buttons can be placed on the screen according to user settings.
- Cut / Copy / Paste
  - Separate operations exist for Copy with Paste and Cut with Paste. As a result, the copy and cut clipboards are independent, allowing both tasks to proceed simultaneously without conflicts. Additionally, clipboard contents can be visually inspected.

  - Copy principle: A copy receives the suffix _copy. If a name conflict occurs, numbering is applied.

- Delete

- Create New Folder / Create New File
  - For new files, they are generated in the format new.txt to prevent name conflicts. If a conflict occurs, numbering is applied.
- shortcut
  - copy : ctrl + c
  - copy-paste : ctrl + v
  - cut : ctrl + x
  - cut-paste : ctrl + shift + x
  - delete(move to recycle bin) : del
  - select all : ctrl + a
- Todos : customize shortcut / configurable newly created file or folder's name(if file, type too)

### UI configurable settings
  - File viewer each icon's area size
  - entire themes
    - Default / Linux-theme / Retro-theme / Dark-theme
  - Languague
    - now available : EN, KR
  - Util buttons deployment


### Search basic features
- Asynchronous processing
- Multi-threaded parallel processing
- Cache based on search history
- Real-time search result display
- Search cancellation supported


### Advanced Search features
- Thread pool configuration
  - Default / 4 / 8 / 16 / 32
- Search targets can be specified
  - Folders & files
  - Files only
  - Folders only
- Search within file contents:
  - Be aware that this may cause performance degradation.
  - Applied only to text files.
  - Special similarity-based search methods do not search within file contents.
- File attribute filtering:
  - File size: Up to 100GB
    - Folder size filtering is not yet supported: Recursive traversal of folders may cause significant performance degradation.
  - Creation date
  - Modification date
  - File type: Input as a list of extensions. Example: .txt .pdf
  - File owner:
    - Retrieving file owner via shell commands may cause performance issues.
    - Requires administrator privileges.
- Search symbolic links:
  - Be cautious, as symbolic links can create circular references.
  - No logic to handle circular references has been implemented yet.

- Search methods:
  - If access permission issues occur, the search will skip the path and continue.
  - Basic search: Checks if the search string is contained within the target (contains operation).
  - Regular expressions: Must be written according to Rust regex syntax.
  - Fuzzy Matching 1: Damerau-Levenshtein distance
  - Fuzzy Matching 2: Jaccard Similarity
      - The threshold used for fuzzy matching is specified in a separate JSON file.
Option to generate search logs.

## Tech

- Tauri / Rust
- SvelteKit

## System Architecture
<p>
<img width="500px" src="./static/architecture/system_architecture.PNG">
</p>

## Backend API Documents
- It will be located at src-tauri / target / doc / app
- It seems docs sometimes makes initial dev build slower
- so i recommand not to make all docs which contains deps

## Custom icons
- directory icons
<p>
<img width="500px" src="./static/banner/dir_icons_banner.png">
</p>

- file icons
<p>
<img width="500px" src="./static/banner/icons_banner.png">
</p>

## ScreenShots
<p>Intro screen</p>
<p>
<img width="300px" src="./static/screenshots/splash.png">
</p>

<p>Main screen</p>
<p>
<img width="300px" src="./static/screenshots/main_theme1.png">
</p>

<p>Modal</p>
<p>
<img width="300px" src="./static/screenshots/modal1.png">
</p>

<p>XP-style theme</p>
<p>
<img width="300px" src="./static/screenshots/main_theme2.png">
</p>

<p>SF-style theme</p>
<p>
<img width="300px" src="./static/screenshots/main_theme3.png">
</p>

<p>Linux-style theme</p>
<p>
<img width="300px" src="./static/screenshots/main_theme4.png">
</p>


## Projects Settings
### development env
- npm 10.5.0
- rust & cargo 1.79.0

- Download

```bash
git clone https://github.com/kdhProg/customFileExplorer
```
or
```bash
gh repo clone kdhProg/customFileExplorer
```

## Todos

- Optimize Searching API
- More UI Themes
- User-Customizable UI
- Unzip functions

## Commands

Frontend dev server

```bash
npm run dev
```

Frontend build
```bash
npm run build
```

Development Build 
```bash
cargo tauri dev
```
or
```bash
npm run tauri dev
```

Rust Docs Build ( Not root directory, should move to "src-tauri")
(includes all dependencies - it will make all of docs which contains deps)
```bash
cargo doc --open
```
(without dependencies, only user-written API)
```bash
cargo doc --no-deps --open
```


### Tauri Sveltekit reference

>[ref](https://tauri.app/ko/v1/guides/getting-started/setup/sveltekit/)

