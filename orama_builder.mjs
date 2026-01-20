import { writeFileSync } from 'fs'
import { create, insertMultiple } from '@orama/orama'
import { persist } from '@orama/plugin-data-persistence'
import { parse as semverParse } from 'semver'

function getMajorVersion(version) {
  const parsed = semverParse(version)
  return parsed ? parsed.major : 0
}

function processExclamationPrefix(doc) {
  const result = { ...doc }
  for (const key of Object.keys(result)) {
    if (key.startsWith('!')) {
      const normalKey = key.slice(1)
      if (!(normalKey in result) || result[normalKey] === undefined) {
        result[normalKey] = result[key]
      }
      delete result[key]
    }
  }
  return result
}

async function main() {
  const outputPath = process.argv[2]
  if (!outputPath) {
    console.error('Usage: node orama_builder.mjs <output_path>')
    process.exit(1)
  }

  let input = ''
  for await (const chunk of process.stdin) {
    input += chunk
  }

  const rawDocs = JSON.parse(input)
  const docs = rawDocs.map((doc) => {
    const processed = processExclamationPrefix(doc)
    return {
      ...processed,
      api_major: getMajorVersion(processed.api_version),
    }
  })

  const db = await create({
    schema: {
      name: 'string',
      owner: 'string',
      categories: 'enum[]',
      license: 'enum',
      api_major: 'number',
      downloads: 'number',
      stars: 'number',
      created_at: 'number',
      updated_at: 'number',
    },
    sort: { enabled: true },
    components: {
      tokenizer: {
        stemming: false,
      },
    },
  })

  await insertMultiple(db, docs)

  // Persist to binary
  const serialized = await persist(db, 'seqproto')
  writeFileSync(outputPath, Buffer.from(serialized))

  console.log(`Indexed ${docs.length} documents`)
}

main().catch((e) => {
  console.error('Build failed:', e)
  process.exit(1)
})
