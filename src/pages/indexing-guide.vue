<script setup lang="ts">
import { renderHighlightedString } from '@modrinth/utils'
import { defineMessages, useVIntl } from '@modrinth/ui'
import indexingGuideContent from '~/assets/docs/PLUGIN_INDEXING.md?raw'

const { formatMessage } = useVIntl()

const messages = defineMessages({
  title: {
    id: 'indexing-guide.title',
    defaultMessage: 'Plugin Indexing Guide',
  },
  description: {
    id: 'indexing-guide.description',
    defaultMessage: 'Learn how NukkitHub indexes plugins from GitHub repositories',
  },
})

const title = formatMessage(messages.title)
const description = formatMessage(messages.description)

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description,
})
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1 class="page-title">{{ formatMessage(messages.title) }}</h1>
      <p class="page-description">{{ formatMessage(messages.description) }}</p>
    </div>
    <div class="markdown-body" v-html="renderHighlightedString(indexingGuideContent)" />
  </div>
</template>

<style scoped lang="scss">
.page-container {
  width: calc(100% - 2 * var(--spacing-card-md));
  max-width: 900px;
  margin-inline: auto;
  box-sizing: border-box;
  margin-block: var(--spacing-card-md);
  padding: var(--spacing-card-lg);
  background: var(--color-raised-bg);
  border-radius: var(--size-rounded-card);
}

.page-header {
  margin-bottom: 2rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--color-divider);
}

.page-title {
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--color-contrast);
  margin: 0 0 0.75rem 0;
}

.page-description {
  font-size: 1.125rem;
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.6;
}

.markdown-body {
  width: 100%;
  max-width: 100%;
  color: var(--color-text);
  line-height: 1.7;

  :deep(h1),
  :deep(h2),
  :deep(h3),
  :deep(h4),
  :deep(h5),
  :deep(h6) {
    color: var(--color-contrast);
    margin-top: 1.5em;
    margin-bottom: 0.75em;
    font-weight: 600;
    line-height: 1.3;
  }

  :deep(h1) {
    font-size: 2rem;
    border-bottom: 1px solid var(--color-divider);
    padding-bottom: 0.5rem;
  }

  :deep(h2) {
    font-size: 1.75rem;
    border-bottom: 1px solid var(--color-divider);
    padding-bottom: 0.4rem;
  }

  :deep(h3) {
    font-size: 1.5rem;
  }

  :deep(h4) {
    font-size: 1.25rem;
  }

  :deep(p) {
    margin: 1em 0;
  }

  :deep(a) {
    color: var(--color-brand);
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }

  :deep(code) {
    background: var(--color-bg);
    padding: 0.2em 0.4em;
    border-radius: var(--radius-sm);
    font-family: var(--font-mono, monospace);
    font-size: 0.9em;
    color: var(--color-contrast);
  }

  :deep(pre) {
    background: var(--color-bg);
    padding: 1rem;
    border-radius: var(--radius-md);
    overflow-x: auto;
    margin: 1.5em 0;

    code {
      background: none;
      padding: 0;
      color: var(--color-text);
      font-size: 0.875rem;
    }
  }

  :deep(blockquote) {
    border-left: 4px solid var(--color-brand);
    padding-left: 1rem;
    margin: 1.5em 0;
    color: var(--color-text-secondary);
    background: var(--color-bg);
    padding: 0.5rem 1rem;
    border-radius: var(--radius-sm);

    p {
      margin: 0.5em 0;
    }
  }

  :deep(ul),
  :deep(ol) {
    margin: 1em 0;
    padding-left: 2em;

    li {
      margin: 0.5em 0;
    }
  }

  :deep(table) {
    width: 100%;
    max-width: 100%;
    display: table;
    table-layout: fixed;
    border-collapse: collapse;
    margin: 1.5em 0;
    background: var(--color-raised-bg);
    border-radius: var(--radius-md);
    overflow: hidden;
    border: 1px solid var(--color-divider);

    th,
    td {
      padding: 0.75rem 1rem;
      text-align: left;
      border-bottom: 1px solid var(--color-divider);
      word-wrap: break-word;
      overflow-wrap: break-word;
      background: var(--color-raised-bg);
    }

    th {
      background: var(--color-button-bg);
      font-weight: 600;
      color: var(--color-contrast);
      border-bottom: 2px solid var(--color-divider);
    }

    tbody tr:hover {
      background: var(--color-bg);
    }

    tr:last-child td {
      border-bottom: none;
    }
  }

  :deep(strong) {
    font-weight: 600;
    color: var(--color-contrast);
  }

  :deep(hr) {
    border: none;
    border-top: 1px solid var(--color-divider);
    margin: 2em 0;
  }
}

@media (max-width: 768px) {
  .page-container {
    padding: var(--spacing-card-md);
  }

  .page-title {
    font-size: 2rem;
  }

  .page-description {
    font-size: 1rem;
  }

  .markdown-body {
    :deep(h1) {
      font-size: 1.75rem;
    }

    :deep(h2) {
      font-size: 1.5rem;
    }

    :deep(h3) {
      font-size: 1.25rem;
    }

    :deep(table) {
      font-size: 0.875rem;
    }
  }
}
</style>
