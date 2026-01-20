<template>
  <div v-if="version" class="version-page">
    <div class="version-page__title universal-card">
      <Breadcrumbs
        :current-title="version.name"
        :link-stack="[
          {
            href: getPreviousLink(),
            label: getPreviousLabel(),
          },
        ]"
      />
      <div class="version-header">
        <h2>{{ version.name }}</h2>
      </div>
      <div class="input-group">
        <ButtonStyled v-if="primaryFile" color="brand">
          <a
            v-tooltip="
              primaryFile.filename + ' (' + formatBytes(primaryFile.size) + ')'
            "
            :href="primaryFile.url"
            @click="emit('onDownload')"
          >
            <DownloadIcon aria-hidden="true" />
            Download
          </a>
        </ButtonStyled>
      </div>
    </div>
    <div class="version-page__changelog universal-card">
      <h3>Changelog</h3>
      <div
        class="markdown-body"
        v-html="
          version.changelog
            ? renderHighlightedString(version.changelog)
            : 'No changelog specified.'
        "
      />
    </div>
    <div class="version-page__files universal-card">
      <h3>Files</h3>
      <div
        v-for="file in version.files"
        :key="file.filename"
        :class="{
          file: true,
          primary: primaryFile && primaryFile.filename === file.filename,
        }"
      >
        <FileIcon aria-hidden="true" />
        <span class="filename">
          <strong>{{ file.filename }}</strong>
          <span class="file-size">({{ formatBytes(file.size) }})</span>
          <span
            v-if="primaryFile && primaryFile.filename === file.filename"
            class="file-type"
          >
            Primary
          </span>
        </span>
        <ButtonStyled>
          <a
            :href="file.url"
            class="raised-button"
            :title="`Download ${file.filename}`"
            tabindex="0"
          >
            <DownloadIcon aria-hidden="true" />
            Download
          </a>
        </ButtonStyled>
      </div>
    </div>
    <div class="version-page__metadata">
      <div class="universal-card full-width-inputs">
        <h3>Metadata</h3>
        <div>
          <h4>Release channel</h4>
          <Badge
            v-if="!version.prerelease"
            class="value"
            type="release"
            color="green"
          />
          <Badge v-else class="value" type="beta" color="orange" />
        </div>
        <div>
          <h4>Version number</h4>
          <span>{{ version.version }}</span>
        </div>
        <div>
          <h4>Downloads</h4>
          <span>{{ version.downloads }}</span>
        </div>
        <div>
          <h4>Publication date</h4>
          <span>
            {{
              $dayjs(version.published_at).format('MMMM D, YYYY [at] h:mm A')
            }}
          </span>
        </div>
        <div>
          <h4>Version ID</h4>
          <CopyCode :text="version.id" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { DownloadIcon, FileIcon } from '@modrinth/assets'
import { Badge, ButtonStyled, CopyCode } from '@modrinth/ui'
import { formatBytes, renderHighlightedString } from '@modrinth/utils'

import Breadcrumbs from '~/components/ui/Breadcrumbs.vue'

const props = defineProps<{
  project: AllayIndex.ProjectView
  versions: AllayIndex.Version[]
}>()

const emit = defineEmits(['onDownload'])
const route = useNativeRoute()
const router = useNativeRouter()

// Find the version from props
const version = computed(() => {
  if (route.params.version === 'latest') {
    // Get the latest version
    if (props.versions.length === 0) return null
    return props.versions.reduce((a, b) =>
      a.published_at > b.published_at ? a : b,
    )
  }

  // Try to find by ID first
  let found = props.versions.find((x) => x.id === route.params.version)
  if (!found) {
    // Try to find by version
    found = props.versions.find((x) => x.version === route.params.version)
  }
  return found || null
})

const primaryFile = computed(() => {
  if (!version.value?.files) return null
  return (
    version.value.files.find((file) => file.primary) ?? version.value.files[0]
  )
})

function getPreviousLink() {
  const back = router.options.history.state.back
  if (typeof back === 'string' && back.includes('/versions')) {
    return back
  }
  return `/${props.project.project_type}/${
    props.project.slug ? props.project.slug : props.project.id
  }/versions`
}

function getPreviousLabel() {
  const back = router.options.history.state.back
  return typeof back === 'string' && back.endsWith('/versions')
    ? 'Back to versions'
    : 'All versions'
}

// SEO
const title = computed(
  () => `${version.value?.name || 'Version'} - ${props.project.title}`,
)
const description = computed(
  () =>
    `Download ${props.project.title} ${version.value?.version || ''} on AllayHub. ${version.value?.downloads || 0} downloads.`,
)

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description,
})
</script>

<style lang="scss" scoped>
.version-page {
  display: grid;

  grid-template:
    'title' auto
    'changelog' auto
    'metadata' auto
    'files' auto
    / 1fr;

  @media (min-width: 1200px) {
    grid-template:
      'title title' auto
      'changelog metadata' auto
      'files metadata' auto
      'dummy metadata' 1fr
      / 1fr 20rem;
  }

  column-gap: var(--spacing-card-md);

  .version-page__title {
    grid-area: title;

    .version-header {
      display: flex;
      flex-wrap: wrap;
      align-items: center;
      gap: var(--spacing-card-md);

      h2 {
        margin: 0;
        font-size: var(--font-size-2xl);
        font-weight: bold;
      }
    }

    .input-group {
      margin-top: var(--spacing-card-md);
    }
  }

  h3 {
    font-size: var(--font-size-lg);
    margin: 0 0 0.5rem 0;
  }

  .version-page__changelog {
    grid-area: changelog;
    overflow-x: hidden;
  }

  .version-page__files {
    grid-area: files;

    .file {
      --text-color: var(--color-button-text);
      --background-color: var(--color-button-bg);

      &.primary {
        --background-color: var(--color-brand-highlight);
        --text-color: var(--color-button-text-active);
      }

      display: flex;
      align-items: center;

      font-weight: 500;
      color: var(--text-color);
      background-color: var(--background-color);
      padding: var(--spacing-card-sm) var(--spacing-card-bg);
      border-radius: var(--size-rounded-sm);

      svg {
        min-width: 1.1rem;
        min-height: 1.1rem;
        margin-right: 0.5rem;
      }

      .filename {
        word-wrap: anywhere;
      }

      .file-size {
        margin-left: 1ch;
        font-weight: 400;
        white-space: nowrap;
      }

      .file-type {
        margin-left: 1ch;
        font-style: italic;
        font-weight: 300;
      }

      .raised-button {
        margin-left: auto;
        background-color: var(--color-raised-bg);
      }

      &:not(:first-child) {
        margin-top: 0.5rem;
      }
    }
  }
}

.version-page__metadata {
  grid-area: metadata;

  h4 {
    margin: 1rem 0 0.25rem 0;
  }
}
</style>
