#!/usr/bin/env bun
/**
 * Recursively clean all node_modules folders under the current directory
 */

import { readdir, rm, stat } from 'node:fs/promises'
import { join } from 'node:path'

const ROOT_DIR = process.cwd()

async function findNodeModules(dir: string): Promise<string[]> {
  const results: string[] = []

  try {
    const entries = await readdir(dir, { withFileTypes: true })

    for (const entry of entries) {
      if (!entry.isDirectory()) continue

      const fullPath = join(dir, entry.name)

      if (entry.name === 'node_modules') {
        results.push(fullPath)
        // Do not recurse into node_modules directories
      } else if (!entry.name.startsWith('.')) {
        // Recursively search subdirectories (skip hidden directories)
        results.push(...(await findNodeModules(fullPath)))
      }
    }
  } catch {
    // Ignore inaccessible directories
  }

  return results
}

async function getDirectorySize(dir: string): Promise<number> {
  let size = 0
  try {
    const entries = await readdir(dir, { withFileTypes: true })
    for (const entry of entries) {
      const fullPath = join(dir, entry.name)
      if (entry.isDirectory()) {
        size += await getDirectorySize(fullPath)
      } else {
        const stats = await stat(fullPath)
        size += stats.size
      }
    }
  } catch {
    // Ignore errors
  }
  return size
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`
  if (bytes < 1024 * 1024 * 1024)
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`
}

async function main() {
  console.log('Scanning node_modules directories...')
  console.log(`Root directory: ${ROOT_DIR}\n`)

  const nodeModulesDirs = await findNodeModules(ROOT_DIR)

  if (nodeModulesDirs.length === 0) {
    console.log('No node_modules directories found')
    return
  }

  console.log(`Found ${nodeModulesDirs.length} node_modules directories:\n`)

  let totalSize = 0
  const dirSizes: { path: string; size: number }[] = []

  for (const dir of nodeModulesDirs) {
    const size = await getDirectorySize(dir)
    totalSize += size
    dirSizes.push({ path: dir, size })
    const relativePath = dir.replace(ROOT_DIR, '.')
    console.log(`  - ${relativePath} (${formatSize(size)})`)
  }

  console.log(`\nTotal size: ${formatSize(totalSize)}`)
  console.log('\nDeleting...\n')

  for (const { path, size } of dirSizes) {
    const relativePath = path.replace(ROOT_DIR, '.')
    try {
      await rm(path, { recursive: true, force: true })
      console.log(`  Deleted: ${relativePath}`)
    } catch (error) {
      console.log(`  Failed to delete: ${relativePath}`, error)
    }
  }

  console.log(`\nCleanup complete! Freed space: ${formatSize(totalSize)}`)
}

main()
