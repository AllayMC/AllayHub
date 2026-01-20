<template>
  <div class="mb-3 flex flex-col gap-3">
    <div class="flex flex-wrap justify-between gap-2">
      <VersionFilterControl
        ref="versionFilters"
        :versions="versions"
        :base-id="`${baseId}-filter`"
        @update:query="updateQuery"
      />

      <ButtonStyled
        v-if="openModal"
        :color="createVersionButtonSecondary ? 'standard' : 'green'"
      >
        <button @click="openModal"><PlusIcon /> Create version</button>
      </ButtonStyled>

      <Pagination
        v-if="!openModal"
        :page="currentPage"
        class="mt-auto"
        :count="Math.ceil(filteredVersions.length / pageSize)"
        @switch-page="switchPage"
      />
    </div>

    <div
      v-if="openModal && filteredVersions.length > pageSize"
      class="flex flex-wrap items-center justify-between gap-2"
    >
      <span>
        Showing {{ (currentPage - 1) * pageSize + 1 }} to
        {{ Math.min(currentPage * pageSize, filteredVersions.length) }} of
        {{ filteredVersions.length }}
      </span>

      <Pagination
        :page="currentPage"
        class="mt-auto"
        :count="Math.ceil(filteredVersions.length / pageSize)"
        @switch-page="switchPage"
      />
    </div>
  </div>
  <div
    v-if="versions.length > 0"
    class="flex flex-col gap-4 rounded-2xl bg-bg-raised px-6 pb-8 pt-4 supports-[grid-template-columns:subgrid]:grid supports-[grid-template-columns:subgrid]:grid-cols-[1fr_min-content] sm:px-8 supports-[grid-template-columns:subgrid]:sm:grid-cols-[min-content_auto_auto_auto_auto_min-content]"
    :class="[
      hasMultipleEnvironments
        ? 'has-environment supports-[grid-template-columns:subgrid]:xl:grid-cols-[min-content_auto_auto_auto_auto_auto_auto_min-content]'
        : 'no-environment supports-[grid-template-columns:subgrid]:xl:grid-cols-[min-content_auto_auto_auto_auto_auto_min-content]',
    ]"
  >
    <div class="versions-grid-row">
      <div class="w-9 max-sm:hidden"></div>
      <div class="text-sm font-bold text-contrast max-sm:hidden">Name</div>
      <div
        class="text-sm font-bold text-contrast max-sm:hidden sm:max-xl:collapse sm:max-xl:hidden"
      >
        Version
      </div>
      <div
        class="text-sm font-bold text-contrast max-sm:hidden sm:max-xl:collapse sm:max-xl:hidden"
      >
        Published
      </div>
      <div
        class="text-sm font-bold text-contrast max-sm:hidden sm:max-xl:collapse sm:max-xl:hidden"
      >
        Downloads
      </div>
      <div
        class="text-sm font-bold text-contrast max-sm:hidden xl:collapse xl:hidden"
      >
        Compatibility
      </div>
      <div
        class="text-sm font-bold text-contrast max-sm:hidden xl:collapse xl:hidden"
      >
        Stats
      </div>
      <div class="w-9 max-sm:hidden"></div>
    </div>
    <template v-for="(version, index) in currentVersions" :key="index">
      <!-- Row divider -->
      <div
        class="versions-grid-row h-px w-full bg-surface-5"
        :class="{
          'max-sm:!hidden': index === 0,
        }"
      ></div>
      <div class="versions-grid-row group relative">
        <AutoLink
          v-if="!!versionLink"
          class="absolute inset-[calc(-1rem-2px)_-2rem] before:absolute before:inset-0 before:transition-all before:content-[''] hover:before:backdrop-brightness-110"
          :to="versionLink?.(version)"
        />
        <div class="flex flex-col justify-center gap-2 sm:contents">
          <div class="flex flex-row items-center gap-2 sm:contents">
            <div class="self-center">
              <div class="relative z-[1] cursor-pointer">
                <VersionChannelIndicator
                  v-tooltip="
                    `Toggle filter for ${version.prerelease ? 'beta' : 'release'}`
                  "
                  :channel="version.prerelease ? 'beta' : 'release'"
                  @click="
                    versionFilters?.toggleFilter(
                      'channel',
                      version.prerelease ? 'beta' : 'release',
                    )
                  "
                />
              </div>
            </div>
            <div
              class="pointer-events-none relative z-[1] flex flex-col justify-center"
              :class="{
                'group-hover:underline': !!versionLink,
              }"
            >
              <div class="font-bold text-contrast">
                {{ version.name }}
              </div>
            </div>
          </div>
          <div
            class="pointer-events-none z-[1] flex items-center font-medium max-sm:hidden sm:max-xl:hidden"
          >
            {{ version.version }}
          </div>
          <div class="flex flex-col justify-center gap-2 sm:contents">
            <div
              class="flex flex-col justify-center gap-1 max-sm:flex-row max-sm:justify-start max-sm:gap-3 xl:contents"
            >
              <div
                v-tooltip="
                  formatMessage(commonMessages.dateAtTimeTooltip, {
                    date: new Date(version.published_at),
                    time: new Date(version.published_at),
                  })
                "
                class="z-[1] flex cursor-help items-center gap-1 text-nowrap font-medium xl:self-center"
              >
                <CalendarIcon class="xl:hidden" />
                {{ formatRelativeTime(new Date(version.published_at)) }}
              </div>
              <div
                class="pointer-events-none z-[1] flex items-center gap-1 font-medium xl:self-center"
              >
                <DownloadIcon class="xl:hidden" />
                {{ formatNumber(version.downloads) }}
              </div>
            </div>
          </div>
        </div>
        <div class="z-[1] flex items-start justify-end gap-1 sm:items-center">
          <slot name="actions" :version="version"></slot>
        </div>
        <div
          v-if="showFiles"
          class="tag-list pointer-events-none relative z-[1] col-span-full"
        >
          <div
            v-for="(file, fileIdx) in version.files"
            :key="`platform-tag-${fileIdx}`"
            :class="`flex items-center gap-1 text-wrap rounded-full bg-button-bg px-2 py-0.5 text-xs font-medium ${file.primary || fileIdx === 0 ? 'bg-brand-highlight text-contrast' : 'text-primary'}`"
          >
            <StarIcon v-if="file.primary || fileIdx === 0" class="shrink-0" />
            {{ file.filename }} - {{ formatBytes(file.size) }}
          </div>
        </div>
      </div>
    </template>
  </div>
  <div class="mt-3 flex">
    <Pagination
      :page="currentPage"
      class="ml-auto"
      :count="Math.ceil(filteredVersions.length / pageSize)"
      @switch-page="switchPage"
    />
  </div>
</template>
<script setup lang="ts">
import {
  CalendarIcon,
  DownloadIcon,
  PlusIcon,
  StarIcon,
} from '@modrinth/assets'
import { ButtonStyled } from '@modrinth/ui'
import { formatBytes, formatNumber, type Version } from '@modrinth/utils'
import { computed, type Ref, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { useRelativeTime } from '../../composables'
import { useVIntl } from '../../composables/i18n'
import { commonMessages } from '../../utils/common-messages'
import AutoLink from '../base/AutoLink.vue'
import {
  Pagination,
  VersionChannelIndicator,
  VersionFilterControl,
} from '../index'

const { formatMessage } = useVIntl()
const formatRelativeTime = useRelativeTime()

const props = withDefaults(
  defineProps<{
    baseId?: string
    project: {
      project_type: string
      slug?: string
      id: string
    }
    versions: Version[]
    showFiles?: boolean
    currentMember?: boolean
    versionLink?: (version: Version) => string
    openModal?: () => void
    createVersionButtonSecondary?: boolean
    hasMultipleEnvironments?: boolean
  }>(),
  {
    baseId: undefined,
    showFiles: false,
    currentMember: false,
    versionLink: undefined,
    hasMultipleEnvironments: false,
  },
)

const currentPage: Ref<number> = ref(1)
const pageSize: Ref<number> = ref(20)
const versionFilters: Ref<InstanceType<typeof VersionFilterControl> | null> =
  ref(null)

const selectedChannels: Ref<string[]> = computed(
  () => versionFilters.value?.selectedChannels ?? [],
)

const filteredVersions = computed(() => {
  return props.versions.filter(
    (version) =>
      selectedChannels.value.length === 0 ||
      selectedChannels.value.includes(version.prerelease ? 'beta' : 'release'),
  )
})

const currentVersions = computed(() =>
  filteredVersions.value.slice(
    (currentPage.value - 1) * pageSize.value,
    currentPage.value * pageSize.value,
  ),
)

const route = useRoute()
const router = useRouter()

if (route.query.page) {
  currentPage.value = Number(route.query.page) || 1
}

function switchPage(page: number) {
  currentPage.value = page

  router.replace({
    query: {
      ...route.query,
      page: currentPage.value !== 1 ? currentPage.value : undefined,
    },
  })

  window.scrollTo({ top: 0, behavior: 'smooth' })
}

function updateQuery(
  newQueries: Record<string, string | string[] | undefined | null>,
) {
  if (newQueries.page) {
    currentPage.value = Number(newQueries.page)
  } else if (newQueries.page === undefined) {
    currentPage.value = 1
  }

  router.replace({
    query: {
      ...route.query,
      ...newQueries,
    },
  })
}
</script>
<style scoped>
.versions-grid-row {
  @apply grid grid-cols-[1fr_min-content] gap-4 supports-[grid-template-columns:subgrid]:col-span-full supports-[grid-template-columns:subgrid]:!grid-cols-subgrid sm:grid-cols-[min-content_1fr_1fr_1fr_1fr_min-content];
}

.has-environment .versions-grid-row {
  @apply xl:grid-cols-[min-content_1fr_1fr_1fr_1fr_1fr_1fr_min-content];
}

.no-environment .versions-grid-row {
  @apply xl:grid-cols-[min-content_1fr_1fr_1fr_1fr_1fr_min-content];
}
</style>
