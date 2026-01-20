<script setup lang="ts" generic="T">
import type { RouteLocationRaw } from 'vue-router'

import AutoLink from '../base/AutoLink.vue'
import Avatar from '../base/Avatar.vue'
import Checkbox from '../base/Checkbox.vue'

export interface ContentCreator {
  name: string
  type: 'user' | 'organization'
  id: string
  link?: string | RouteLocationRaw
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  linkProps?: any
}

export interface ContentProject {
  id: string
  link?: string | RouteLocationRaw
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  linkProps?: any
}

export interface ContentItem<T> {
  path: string
  disabled: boolean
  filename: string
  data: T

  icon?: string
  title?: string
  project?: ContentProject
  creator?: ContentCreator

  version?: string
  versionId?: string
}

withDefaults(
  defineProps<{
    item: ContentItem<T>
    last?: boolean
  }>(),
  {
    last: false,
  },
)

const model = defineModel<boolean>()
</script>
<template>
  <div
    class="relative grid h-[64px] grid-cols-[min-content,4fr,3fr,2fr] items-center gap-3 border-0 border-solid border-b-button-bg p-2"
    :class="{ 'border-b-[1px]': !last }"
  >
    <Checkbox v-model="model" :description="``" class="select-checkbox" />
    <div
      class="flex items-center gap-2 font-medium text-contrast"
      :class="{ 'opacity-50': item.disabled }"
    >
      <AutoLink
        :to="item.project?.link ?? ''"
        tabindex="-1"
        v-bind="item.project?.linkProps ?? {}"
      >
        <Avatar
          :src="item.icon ?? ''"
          :class="{ grayscale: item.disabled }"
          size="48px"
        />
      </AutoLink>
      <div class="flex flex-col">
        <AutoLink
          :to="item.project?.link ?? ''"
          v-bind="item.project?.linkProps ?? {}"
        >
          <div
            class="line-clamp-1 text-contrast"
            :class="{ 'line-through': item.disabled }"
          >
            {{ item.title ?? item.filename }}
          </div>
        </AutoLink>
        <AutoLink
          :to="item.creator?.link ?? ''"
          v-bind="item.creator?.linkProps ?? {}"
        >
          <div
            class="line-clamp-1 break-all"
            :class="{ 'opacity-50': item.disabled }"
          >
            <slot v-if="item.creator && item.creator.name" :item="item">
              <span class="text-secondary"> by {{ item.creator.name }} </span>
            </slot>
          </div>
        </AutoLink>
      </div>
    </div>
    <div
      class="flex max-w-60 flex-col"
      :class="{ 'opacity-50': item.disabled }"
    >
      <div v-if="item.version" class="line-clamp-1 break-all">
        <slot :creator="item.creator">
          {{ item.version }}
        </slot>
      </div>
      <div class="line-clamp-1 break-all text-xs text-secondary">
        {{ item.filename }}
      </div>
    </div>
    <div class="flex items-center justify-end gap-1">
      <slot name="actions" :item="item" />
    </div>
  </div>
</template>
