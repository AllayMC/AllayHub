<template>
  <div class="button-base group flex gap-3 rounded-xl bg-bg-raised p-4">
    <div class="icon">
      <Avatar :src="project.icon_url" size="96px" class="search-icon" />
    </div>
    <div class="flex flex-col gap-2 overflow-hidden">
      <div class="no-wrap gap-2 overflow-hidden text-ellipsis">
        <span class="m-0 text-lg font-extrabold leading-none text-contrast">{{
          project.title
        }}</span>
        <span v-if="project.author" class="text-secondary">
          by {{ project.author }}</span
        >
      </div>
      <div class="m-0 line-clamp-2">
        {{ project.description }}
      </div>
      <div class="no-wrap mt-auto flex items-center gap-1">
        <TagsIcon class="h-4 w-4 shrink-0" />
        <div
          v-for="tag in categories"
          :key="tag"
          class="flex gap-1 rounded-full bg-button-bg px-[0.375rem] py-0.5 text-sm font-semibold text-secondary"
        >
          {{ formatCategory(tag) }}
        </div>
      </div>
    </div>
    <div class="ml-auto flex shrink-0 flex-col items-end gap-2">
      <div class="flex items-center gap-2">
        <DownloadIcon class="shrink-0" />
        <span>
          {{ formatNumber(project.downloads) }}
          <span class="text-secondary">downloads</span>
        </span>
      </div>
      <div class="flex items-center gap-2">
        <HeartIcon class="shrink-0" />
        <span>
          {{ formatNumber(project.follows ?? project.followers) }}
          <span class="text-secondary">followers</span>
        </span>
      </div>
      <div class="relative mt-auto">
        <div
          :class="{
            'transition-all group-focus-within:scale-95 group-focus-within:opacity-0 group-hover:-translate-y-3 group-hover:scale-95 group-hover:opacity-0':
              $slots.actions,
          }"
          class="flex items-center gap-2"
        >
          <HistoryIcon class="shrink-0" />
          <span>
            <span class="text-secondary">Updated</span>
            {{ formatRelativeTime(project.date_modified ?? project.updated) }}
          </span>
        </div>
        <div
          class="absolute bottom-0 right-0 w-fit translate-y-3 scale-95 opacity-0 transition-all group-focus-within:scale-100 group-focus-within:opacity-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
        >
          <slot name="actions" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {
  DownloadIcon,
  HeartIcon,
  HistoryIcon,
  TagsIcon,
} from '@modrinth/assets'
import { formatCategory, formatNumber } from '@modrinth/utils'

import { useRelativeTime } from '../../composables'
import Avatar from '../base/Avatar.vue'

const formatRelativeTime = useRelativeTime()

defineProps({
  project: {
    type: Object,
    required: true,
  },
  categories: {
    type: Array,
    required: true,
  },
})
</script>
