<template>
  <section class="experimental-styles-within overflow-visible">
    <ProjectPageVersions
      v-if="versions.length"
      :project="project"
      :versions="versions as never"
      :show-files="flags.showVersionFilesInTable"
      :current-member="false"
      :game-versions="gameVersions"
      :base-id="baseDropdownId"
      :version-link="getVersionLink"
    >
      <template #actions="{ version }">
        <ButtonStyled circular type="transparent">
          <a
            v-tooltip="`Download`"
            :href="getPrimaryFile(version).url"
            class="hover:!bg-button-bg [&>svg]:!text-green"
            aria-label="Download"
            @click="emit('onDownload')"
          >
            <DownloadIcon aria-hidden="true" />
          </a>
        </ButtonStyled>
        <ButtonStyled circular type="transparent">
          <OverflowMenu
            v-tooltip="'More options'"
            class="hover:!bg-button-bg"
            :dropdown-id="`${baseDropdownId}-${version.id}`"
            :options="[
              {
                id: 'download',
                color: 'primary',
                hoverFilled: true,
                link: getPrimaryFile(version).url,
                action: () => {
                  emit('onDownload')
                },
              },
              {
                id: 'new-tab',
                action: () => {},
                link: `/${project.project_type}/${
                  project.slug ? project.slug : project.id
                }/version/${encodeURI(getVersionUrlEnding(version))}`,
                external: true,
              },
              {
                id: 'copy-link',
                action: () => {
                  const config = useRuntimeConfig()
                  copyToClipboard(
                    `${config.public.siteUrl}/${project.project_type}/${
                      project.slug ? project.slug : project.id
                    }/version/${encodeURI(getVersionUrlEnding(version))}`,
                  )
                },
              },
              { divider: true, shown: flags.developerMode },
              {
                id: 'copy-id',
                action: () => {
                  copyToClipboard(version.id)
                },
                shown: flags.developerMode,
              },
            ]"
            aria-label="More options"
          >
            <MoreVerticalIcon aria-hidden="true" />
            <template #download>
              <DownloadIcon aria-hidden="true" />
              Download
            </template>
            <template #new-tab>
              <ExternalIcon aria-hidden="true" />
              Open in new tab
            </template>
            <template #copy-link>
              <LinkIcon aria-hidden="true" />
              Copy link
            </template>
            <template #copy-id>
              <ClipboardCopyIcon aria-hidden="true" />
              Copy ID
            </template>
          </OverflowMenu>
        </ButtonStyled>
      </template>
    </ProjectPageVersions>
    <template v-else>
      <p class="ml-2">No versions available for this plugin.</p>
    </template>
  </section>
</template>

<script setup lang="ts">
import {
  ClipboardCopyIcon,
  DownloadIcon,
  ExternalIcon,
  LinkIcon,
  MoreVerticalIcon,
} from '@modrinth/assets'
import { ButtonStyled, OverflowMenu, ProjectPageVersions } from '@modrinth/ui'
import { API_VERSIONS } from '~/types/allayhub'

const props = defineProps<{
  project: AllayIndex.ProjectView
  versions: AllayIndex.Version[]
}>()

const flags = useFeatureFlags()

// Game versions for filter dropdown
const gameVersions = computed(() => {
  return API_VERSIONS.map((v, index) => ({
    version: v.version,
    version_type: 'release' as const,
    date: v.release_date,
    major: index === 0,
  }))
})

const emit = defineEmits<{
  onDownload: []
}>()

const baseDropdownId = useId()

function getPrimaryFile(version: AllayIndex.Version): AllayIndex.VersionFile {
  return version.files.find((x) => x.primary) || version.files[0]
}

function getVersionLink(version: AllayIndex.Version): string {
  return `/${props.project.project_type}/${
    props.project.slug ? props.project.slug : props.project.id
  }/version/${encodeURI(version.version)}`
}

function getVersionUrlEnding(version: AllayIndex.Version): string {
  return version.version
}

async function copyToClipboard(text: string): Promise<void> {
  await navigator.clipboard.writeText(text)
}
</script>
