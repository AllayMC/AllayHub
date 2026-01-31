<template>
  <div>
    <div
      v-if="expandedGalleryItem != null"
      class="expanded-image-modal"
      @click="expandedGalleryItem = null"
    >
      <div class="content">
        <img
          class="image"
          :class="{ 'zoomed-in': zoomedIn }"
          :src="
            expandedGalleryItem.url
              ? expandedGalleryItem.url
              : '/placeholder.svg'
          "
          :alt="
            expandedGalleryItem.title
              ? expandedGalleryItem.title
              : 'gallery-image'
          "
          @click.stop
        />

        <div class="floating" @click.stop>
          <div class="text">
            <h2 v-if="expandedGalleryItem.title">
              {{ expandedGalleryItem.title }}
            </h2>
            <p v-if="expandedGalleryItem.description">
              {{ expandedGalleryItem.description }}
            </p>
          </div>
          <div class="controls">
            <div class="buttons">
              <button
                class="close circle-button"
                @click="expandedGalleryItem = null"
              >
                <XIcon aria-hidden="true" />
              </button>
              <a
                class="open circle-button"
                target="_blank"
                :href="
                  expandedGalleryItem.url
                    ? expandedGalleryItem.url
                    : '/placeholder.svg'
                "
              >
                <ExternalIcon aria-hidden="true" />
              </a>
              <button class="circle-button" @click="zoomedIn = !zoomedIn">
                <ExpandIcon v-if="!zoomedIn" aria-hidden="true" />
                <ContractIcon v-else aria-hidden="true" />
              </button>
              <button
                v-if="project.gallery.length > 1"
                class="previous circle-button"
                @click="previousImage()"
              >
                <LeftArrowIcon aria-hidden="true" />
              </button>
              <button
                v-if="project.gallery.length > 1"
                class="next circle-button"
                @click="nextImage()"
              >
                <RightArrowIcon aria-hidden="true" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-if="project.gallery.length" class="items">
      <div
        v-for="(item, index) in project.gallery"
        :key="index"
        class="card gallery-item"
      >
        <a class="gallery-thumbnail" @click="expandImage(item, index)">
          <img
            :src="item.url ? item.url : '/placeholder.svg'"
            :alt="item.title ? item.title : 'gallery-image'"
          />
        </a>
        <div class="gallery-body">
          <div class="gallery-info">
            <h2 v-if="item.title">
              {{ item.title }}
            </h2>
            <p v-if="item.description">
              {{ item.description }}
            </p>
          </div>
        </div>
        <div class="gallery-bottom">
          <div v-if="item.created" class="gallery-created">
            <CalendarIcon aria-hidden="true" aria-label="Date created" />
            {{ $dayjs(item.created).format('MMMM D, YYYY') }}
          </div>
        </div>
      </div>
    </div>
    <template v-else>
      <p class="ml-2">No images in gallery.</p>
    </template>
  </div>
</template>

<script setup lang="ts">
import {
  CalendarIcon,
  ContractIcon,
  ExpandIcon,
  ExternalIcon,
  LeftArrowIcon,
  RightArrowIcon,
  XIcon,
} from '@modrinth/assets'

interface GalleryItem {
  url: string
  title?: string
  description?: string
  created?: string
}

const props = defineProps<{
  project: AllayIndex.ProjectView
}>()

const title = `${props.project.title} - Gallery`
const description = `View ${props.project.gallery?.length || 0} images of ${props.project.title} on AllayHub.`

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description,
})

const { $dayjs } = useNuxtApp()

const expandedGalleryItem = ref<GalleryItem | null>(null)
const expandedGalleryIndex = ref(0)
const zoomedIn = ref(false)

function nextImage(): void {
  expandedGalleryIndex.value++
  if (expandedGalleryIndex.value >= props.project.gallery.length) {
    expandedGalleryIndex.value = 0
  }
  expandedGalleryItem.value = props.project.gallery[
    expandedGalleryIndex.value
  ] as GalleryItem
}

function previousImage(): void {
  expandedGalleryIndex.value--
  if (expandedGalleryIndex.value < 0) {
    expandedGalleryIndex.value = props.project.gallery.length - 1
  }
  expandedGalleryItem.value = props.project.gallery[
    expandedGalleryIndex.value
  ] as GalleryItem
}

function expandImage(item: GalleryItem, index: number): void {
  expandedGalleryItem.value = item
  expandedGalleryIndex.value = index
  zoomedIn.value = false
}

// Keyboard navigation
onMounted(() => {
  const keyListener = (e: KeyboardEvent) => {
    if (expandedGalleryItem.value) {
      e.preventDefault()
      if (e.key === 'Escape') {
        expandedGalleryItem.value = null
      } else if (e.key === 'ArrowLeft') {
        e.stopPropagation()
        previousImage()
      } else if (e.key === 'ArrowRight') {
        e.stopPropagation()
        nextImage()
      }
    }
  }

  document.addEventListener('keydown', keyListener)

  onUnmounted(() => {
    document.removeEventListener('keydown', keyListener)
  })
})
</script>

<style lang="scss" scoped>
.expanded-image-modal {
  position: fixed;
  z-index: 20;
  overflow: auto;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: #000000;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;

  .content {
    position: relative;
    width: calc(100vw - 2 * var(--spacing-card-lg));
    height: calc(100vh - 2 * var(--spacing-card-lg));

    .circle-button {
      padding: 0.5rem;
      line-height: 1;
      display: flex;
      max-width: 2rem;
      color: var(--color-button-text);
      background-color: var(--color-button-bg);
      border-radius: var(--size-rounded-max);
      margin: 0;
      box-shadow: inset 0px -1px 1px rgb(17 24 39 / 10%);

      &:not(:last-child) {
        margin-right: 0.5rem;
      }

      &:hover {
        background-color: var(--color-button-bg-hover) !important;

        svg {
          color: var(--color-button-text-hover) !important;
        }
      }

      &:active {
        background-color: var(--color-button-bg-active) !important;

        svg {
          color: var(--color-button-text-active) !important;
        }
      }

      svg {
        height: 1rem;
        width: 1rem;
      }
    }

    .image {
      position: absolute;
      left: 50%;
      top: 50%;
      transform: translate(-50%, -50%);
      max-width: calc(100vw - 2 * var(--spacing-card-lg));
      max-height: calc(100vh - 2 * var(--spacing-card-lg));
      border-radius: var(--size-rounded-card);

      &.zoomed-in {
        object-fit: cover;
        width: auto;
        height: calc(100vh - 2 * var(--spacing-card-lg));
        max-width: calc(100vw - 2 * var(--spacing-card-lg));
      }
    }
    .floating {
      position: absolute;
      left: 50%;
      transform: translateX(-50%);
      bottom: var(--spacing-card-md);
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: var(--spacing-card-sm);
      transition: opacity 0.25s ease-in-out;
      opacity: 1;
      padding: 2rem 2rem 0 2rem;

      &:not(&:hover) {
        opacity: 0.4;
        .text {
          transform: translateY(2.5rem) scale(0.8);
          opacity: 0;
        }
        .controls {
          transform: translateY(0.25rem) scale(0.9);
        }
      }

      .text {
        display: flex;
        flex-direction: column;
        max-width: 40rem;
        transition:
          opacity 0.25s ease-in-out,
          transform 0.25s ease-in-out;
        text-shadow: 1px 1px 10px #000000d4;
        margin-bottom: 0.25rem;
        gap: 0.5rem;

        h2 {
          color: var(--dark-color-text-dark);
          font-size: 1.25rem;
          text-align: center;
          margin: 0;
        }

        p {
          color: var(--dark-color-text);
          margin: 0;
        }
      }
      .controls {
        background-color: var(--color-raised-bg);
        padding: var(--spacing-card-md);
        border-radius: var(--size-rounded-card);
        transition:
          opacity 0.25s ease-in-out,
          transform 0.25s ease-in-out;
      }
    }
  }
}

.buttons {
  display: flex;

  button {
    margin-right: 0.5rem;
  }
}

.items {
  display: grid;
  grid-template-rows: 1fr;
  grid-template-columns: 1fr;
  grid-gap: var(--spacing-card-md);

  @media screen and (min-width: 1024px) {
    grid-template-columns: 1fr 1fr 1fr;
  }
}

.gallery-item {
  display: flex;
  flex-direction: column;
  padding: 0;

  img {
    width: 100%;
    margin-top: 0;
    margin-bottom: 0;
    border-radius: var(--size-rounded-card) var(--size-rounded-card) 0 0;

    min-height: 10rem;
    object-fit: cover;
  }

  .gallery-body {
    width: calc(100% - 2 * var(--spacing-card-md));
    padding: var(--spacing-card-sm) var(--spacing-card-md);
    overflow-wrap: anywhere;

    .gallery-info {
      h2 {
        margin-bottom: 0.5rem;
      }

      p {
        margin: 0 0 0.5rem 0;
      }
    }
  }

  .gallery-bottom {
    width: calc(100% - 2 * var(--spacing-card-md));
    padding: 0 var(--spacing-card-md) var(--spacing-card-sm)
      var(--spacing-card-md);

    .gallery-created {
      display: flex;
      align-items: center;
      margin-bottom: 0.5rem;
      color: var(--color-icon);

      svg {
        width: 1rem;
        height: 1rem;
        margin-right: 0.25rem;
      }
    }
  }

  .gallery-thumbnail {
    cursor: pointer;

    img {
      transition: filter 0.25s ease-in-out;

      &:hover {
        filter: brightness(0.7);
      }
    }
  }
}
</style>
