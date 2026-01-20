use std::fs;
use std::path::Path;

pub const CATEGORIES: &[(&str, &str, &str)] = &[
    (
        "adventure",
        "Adventure",
        "Adventure and exploration plugins",
    ),
    ("cursed", "Cursed", "Cursed and challenge plugins"),
    (
        "decoration",
        "Decoration",
        "Decoration and building plugins",
    ),
    ("economy", "Economy", "Economy and trading plugins"),
    ("equipment", "Equipment", "Equipment and gear plugins"),
    ("food", "Food", "Food and farming plugins"),
    (
        "game-mechanics",
        "Game Mechanics",
        "Game mechanics modification plugins",
    ),
    ("library", "Library", "API libraries for developers"),
    ("magic", "Magic", "Magic and spells plugins"),
    ("management", "Management", "Server management plugins"),
    ("minigame", "Minigame", "Minigame plugins"),
    ("mobs", "Mobs", "Mob related plugins"),
    (
        "optimization",
        "Optimization",
        "Performance optimization plugins",
    ),
    ("social", "Social", "Social and communication plugins"),
    ("storage", "Storage", "Storage and inventory plugins"),
    (
        "technology",
        "Technology",
        "Technology and automation plugins",
    ),
    ("transportation", "Transportation", "Transportation plugins"),
    ("utility", "Utility", "General utility plugins"),
    (
        "world-generation",
        "World Generation",
        "World generation plugins",
    ),
];

pub fn generate_allayhub_ts() -> String {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");

    let mut ts = String::new();

    ts.push_str("export interface Category {\n");
    ts.push_str("  id: string\n");
    ts.push_str("  name: string\n");
    ts.push_str("  description: string\n");
    ts.push_str("}\n\n");

    ts.push_str("export interface ApiVersion {\n");
    ts.push_str("  version: string\n");
    ts.push_str("  release_date: string\n");
    ts.push_str("}\n\n");

    ts.push_str("export interface ServerVersion {\n");
    ts.push_str("  version: string\n");
    ts.push_str("  api_version: string\n");
    ts.push_str("  release_date: string\n");
    ts.push_str("}\n\n");

    ts.push_str("export interface IndexMeta {\n");
    ts.push_str("  updated_at: string\n");
    ts.push_str("  index_version: string\n");
    ts.push_str("  generator: string\n");
    ts.push_str("}\n\n");

    ts.push_str("export interface Loader {\n");
    ts.push_str("  id: string\n");
    ts.push_str("  name: string\n");
    ts.push_str("  icon?: string\n");
    ts.push_str("}\n\n");

    ts.push_str("export const CATEGORIES: Category[] = [\n");
    for (id, name, desc) in CATEGORIES {
        ts.push_str(&format!(
            "  {{ id: '{}', name: '{}', description: '{}' }},\n",
            id, name, desc
        ));
    }
    ts.push_str("]\n\n");

    ts.push_str("export const API_VERSIONS: ApiVersion[] = [\n");
    ts.push_str("  { version: '1.0.0', release_date: '2026-01-01' },\n");
    ts.push_str("]\n\n");

    ts.push_str("export const LATEST_API_VERSION = '1.0.0'\n\n");

    ts.push_str("export const SERVER_VERSIONS: ServerVersion[] = [\n");
    ts.push_str("  { version: '1.0.0', api_version: '1.0.0', release_date: '2026-01-01' },\n");
    ts.push_str("]\n\n");

    ts.push_str("export const INDEX_META: IndexMeta = {\n");
    ts.push_str(&format!("  updated_at: '{}',\n", now));
    ts.push_str("  index_version: '1.0',\n");
    ts.push_str("  generator: 'allayindexer',\n");
    ts.push_str("}\n\n");

    ts.push_str("export const LOADERS: Loader[] = [\n");
    ts.push_str("  { id: 'plugin', name: 'Plugin', icon: 'plugin' },\n");
    ts.push_str("]\n\n");

    ts.push_str("export type LoaderId = 'plugin'\n\n");

    ts.push_str("export type CategoryId =\n");
    for (id, _, _) in CATEGORIES {
        ts.push_str(&format!("  | '{}'\n", id));
    }
    ts.push('\n');

    ts.push_str("export function getCategoryById(id: string): Category | undefined {\n");
    ts.push_str("  return CATEGORIES.find((c) => c.id === id)\n");
    ts.push_str("}\n\n");

    ts.push_str("export function getCategoryName(id: string): string {\n");
    ts.push_str("  return getCategoryById(id)?.name ?? id\n");
    ts.push_str("}\n\n");

    ts.push_str("export function getCategoryIds(): string[] {\n");
    ts.push_str("  return CATEGORIES.map((c) => c.id)\n");
    ts.push_str("}\n\n");

    ts.push_str("export function getLoaderById(id: string): Loader | undefined {\n");
    ts.push_str("  return LOADERS.find((l) => l.id === id)\n");
    ts.push_str("}\n\n");

    ts.push_str("export function getLoaderName(id: string): string {\n");
    ts.push_str("  return getLoaderById(id)?.name ?? id\n");
    ts.push_str("}\n\n");

    ts.push_str("export function getLoaderIcon(id: string): string | undefined {\n");
    ts.push_str("  return getLoaderById(id)?.icon\n");
    ts.push_str("}\n");

    ts
}

pub fn write_allayhub_ts(output_path: &Path) -> bool {
    let content = generate_allayhub_ts();
    match fs::write(output_path, content) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Failed to write allayhub.ts: {}", e);
            false
        }
    }
}
