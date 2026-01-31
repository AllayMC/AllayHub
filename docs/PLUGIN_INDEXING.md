# Plugin Indexing Guide

This document describes how AllayHub indexes plugins from GitHub repositories. Follow these guidelines to ensure your plugin is properly discovered and displayed.

## Discovery Requirements

For your repository to be indexed as an Allay plugin, it must meet **all** of the following criteria:

1. **Public GitHub repository** - Private repositories are not indexed
2. **Not archived** - Archived repositories are excluded
3. **Not a template** - Template repositories are excluded
4. **Not a `noindex` topic** - Repositories with the `noindex` topic are excluded (see [Plugin Removal](#plugin-removal))
5. **Discoverable** - At least one of:
   - Contains `org.allaymc` in `build.gradle` or `build.gradle.kts`
   - Has the `allaymc-plugin` topic on the repository

## Discovery Methods

AllayHub uses two complementary methods to discover plugins:

### 1. Code Search

Searches for repositories containing `org.allaymc` in Gradle build files. This is the primary discovery method.

> **Note:** GitHub's code search index may have delays or gaps, especially for:
> - Newly created repositories
> - Repositories with low activity
> - Repositories generated from templates

### 2. Topic Search (Recommended)

Searches for repositories with the `allaymc-plugin` topic. This method is more reliable because:

- Repository metadata is indexed faster than code content
- Works even if the code search index hasn't caught up
- Explicitly signals that the repository is an Allay plugin

**To add the topic:** Go to your repository → About (gear icon) → Topics → Add `allaymc-plugin`

> **Note:** Forked repositories are indexed only when discovered via the topic search method. If your plugin is a fork, add the `allaymc-plugin` topic to ensure it is discovered.

## Plugin ID Format

Each plugin is assigned an ID in the `owner/name` format (all lowercase), where:

- `owner` is the GitHub repository owner (user or organization)
- `name` is the plugin name (from AllayGradle DSL, `plugin.json`, or repository name as fallback)

For example, a plugin named "MyPlugin" in a repository owned by "CoolLoong" would have the ID `coolloong/myplugin`.

Plugin data files are stored in a nested directory structure: `AllayHubIndex/{owner}/{name}.json`.

Frontend URLs follow the same pattern: `/plugin/{owner}/{name}`.

## Plugin Metadata

### AllayGradle DSL (Recommended)

Define plugin metadata directly in `build.gradle.kts` using [AllayGradle](https://github.com/AllayMC/AllayGradle):

```kotlin
allay {
    api = "0.23.0"
    plugin {
        entrance = "com.example.MyPlugin"
        name = "My Plugin"
        version = "1.0.0"
        description = "A short description of my plugin"
        authors += "AuthorName"
        website = "https://example.com"
        apiVersion = ">=0.23.0"
        dependencies += dependency("OtherPlugin", "1.0.0")
        dependencies += dependency("OptionalPlugin", optional = true)
    }
}
```

### plugin.json / extension.json (Alternative)

Alternatively, the indexer reads metadata from `plugin.json` or `extension.json` in the resources directory:

- Root module: `src/main/resources/plugin.json`
- Submodule: `<module>/src/main/resources/plugin.json`

```json
{
  "entrance": "com.example.MyPlugin",
  "name": "My Plugin",
  "version": "1.0.0",
  "authors": ["AuthorName"],
  "description": "A short description of my plugin",
  "website": "https://example.com",
  "api_version": ">=0.14.0",
  "dependencies": [
    {
      "name": "OtherPlugin",
      "version": "1.0.0",
      "optional": false
    }
  ]
}
```

#### Template Variables

You can use template variables in `plugin.json`:

| Variable | Description |
|----------|-------------|
| `${project.version}` | Replaced with version from `build.gradle.kts` |
| `${description}` | Replaced with description from `build.gradle.kts` |
| `${project.description}` | Same as `${description}` |
| `@DESCRIPTION@` | Same as `${description}` |

### Fallback Values

| Field | Fallback Source |
|-------|-----------------|
| `name` | Repository name |
| `description` | Repository description |
| `icon_url` | Repository owner's avatar |

## Icon / Logo

Place a logo file **anywhere in the repository** to use as your plugin icon. The indexer searches for these filenames in order (matches `**/filename`):

1. `logo.png`
2. `icon.png`
3. `logo.jpg`
4. `icon.jpg`
5. `logo.svg`
6. `icon.svg`
7. `logo.webp`
8. `icon.webp`

If no logo is found, the repository owner's GitHub avatar is used.

## Gallery Images

### From Repository

Place numbered gallery images **anywhere in the repository** (matches `**/galleryN.ext`):

```
gallery1.png
gallery2.jpg
gallery3.webp
...
gallery10.png
```

Supported extensions: `png`, `jpg`, `jpeg`, `svg`, `webp`, `gif`

The indexer reads gallery images sequentially (`gallery1`, `gallery2`, ...) and stops at the first missing number.

### From README

Images in your README are automatically extracted and added to the gallery after the numbered gallery images.

### Display Priority

Gallery images are ordered as follows (first image is used as the cover on discover page):

1. Numbered gallery images (`gallery1`, `gallery2`, ...)
2. README images (in order of appearance, deduplicated by URL)

## Categories

Categories are derived from your repository's **GitHub Topics**. Add topics that match the following category IDs:

| Category ID | Description |
|-------------|-------------|
| `adventure` | Adventure and exploration plugins |
| `cursed` | Cursed and challenge plugins |
| `decoration` | Decoration and building plugins |
| `economy` | Economy and trading plugins |
| `equipment` | Equipment and gear plugins |
| `food` | Food and farming plugins |
| `game-mechanics` | Game mechanics modification plugins |
| `library` | API libraries for developers |
| `magic` | Magic and spells plugins |
| `management` | Server management plugins |
| `minigame` | Minigame plugins |
| `mobs` | Mob related plugins |
| `optimization` | Performance optimization plugins |
| `social` | Social and communication plugins |
| `storage` | Storage and inventory plugins |
| `technology` | Technology and automation plugins |
| `transportation` | Transportation plugins |
| `utility` | General utility plugins |
| `world-generation` | World generation plugins |

Topics not matching these IDs are ignored. If no matching topics are found, the plugin defaults to `utility`.

## Releases / Versions

Versions are read from GitHub Releases:

- Draft releases are ignored
- Each release becomes a version entry
- Version number is extracted from the tag name (leading `v` is stripped)
- Release body becomes the changelog
- Files with `.jar` or `.zip` extensions are listed as downloadable files
- Download counts are tracked per release

### Primary File Detection

Among release assets, the indexer selects a primary file:
1. First file containing "allay" in the filename
2. Otherwise, the first `.jar` or `.zip` file

## Authors

Authors are populated from:

1. **Repository owner** - Always listed as first author
2. **Plugin authors** - Additional authors from AllayGradle DSL or `plugin.json`, matched against repository contributors

## License

License information is read from GitHub's detected license:

- SPDX ID is used as the license identifier
- If no license is detected, defaults to "ARR" (All Rights Reserved)

## API Version

The API version requirement is extracted from:

1. `apiVersion` in AllayGradle DSL or `api_version` in `plugin.json`
2. `api` in AllayGradle DSL
3. Version catalog references (resolved from `libs.versions.toml`)

## Example Repository Structure

```
my-allay-plugin/
├── .github/
│   └── img/
│       ├── logo.png            # Plugin icon
│       ├── gallery1.png        # Gallery image 1
│       └── gallery2.png        # Gallery image 2
├── build.gradle.kts            # Plugin metadata (AllayGradle DSL)
├── settings.gradle.kts
├── README.md                   # Description (images extracted to gallery)
└── src/
    └── main/
        ├── java/             # Plugin source code
        └── resources/
```

## Plugin Removal

Plugins are automatically removed from the index when any of the following conditions are met during the update cycle (runs hourly):

| Condition | Description |
|-----------|-------------|
| Repository deleted | The repository no longer exists (404) |
| Repository archived | The repository has been archived |
| Plugin removed | The `plugin.json` or AllayGradle DSL no longer defines the plugin |
| Opted out | The repository has the `noindex` topic |

To manually remove your plugin from AllayHub, add the `noindex` topic to your repository (About → Topics → add `noindex`).

## Tips for Better Indexing

1. **Add `allaymc-plugin` topic** - Ensures your plugin is discovered even if code search hasn't indexed it yet
2. **Use category topics** - Add relevant category topics (see [Categories](#categories)) to your repository
3. **Write a good README** - The README becomes your plugin's detailed description
4. **Create releases** - Only released versions appear in the version list
5. **Add a logo** - A custom logo helps your plugin stand out
6. **Include gallery images** - Screenshots help users understand your plugin
7. **Fill out plugin.json** - Complete metadata improves discoverability