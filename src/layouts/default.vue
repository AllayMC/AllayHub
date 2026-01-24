<template>
  <div class="pointer-events-none fixed inset-0 z-[-1]">
    <div id="fixed-background-teleport" class="relative"></div>
  </div>
  <div class="pointer-events-none absolute inset-0 z-[-1]">
    <div id="absolute-background-teleport" class="relative"></div>
  </div>
  <div
    ref="main_page"
    class="layout"
    :class="{
      'expanded-mobile-nav': isBrowseMenuOpen,
      'modrinth-parent__no-modal-blurs': !cosmetics.advancedRendering,
    }"
  >
    <header
      class="experimental-styles-within desktop-only relative z-[5] mx-auto grid max-w-[1280px] grid-cols-[1fr_auto] items-center gap-2 px-6 py-4 lg:grid-cols-[auto_1fr_auto]"
    >
      <div>
        <NuxtLink
          to="/"
          :aria-label="formatMessage(messages.modrinthHomePage)"
          class="group hover:brightness-[--hover-brightness] focus-visible:brightness-[--hover-brightness]"
        >
          <img
            src="~/assets/icons/allay-chan-40x.png"
            alt=""
            aria-hidden="true"
            class="mr-1 h-7 w-auto transition-transform group-active:scale-[0.98]"
          />
          <TextLogo
            aria-hidden="true"
            class="h-7 w-auto translate-y-[3px] text-contrast transition-transform group-active:scale-[0.98]"
          />
        </NuxtLink>
      </div>
      <div
        :class="`col-span-2 row-start-2 flex flex-wrap justify-center ${flags.projectTypesPrimaryNav ? 'gap-2' : 'gap-4'} lg:col-span-1 lg:row-start-auto`"
      >
        <template v-if="flags.projectTypesPrimaryNav">
          <ButtonStyled
            type="transparent"
            :highlighted="
              route.name === 'discover-plugins' ||
              route.path.startsWith('/plugin/')
            "
            :highlighted-style="
              route.name === 'discover-plugins'
                ? 'main-nav-primary'
                : 'main-nav-secondary'
            "
          >
            <nuxt-link to="/discover/plugins">
              <PlugIcon aria-hidden="true" />
              {{ formatMessage(commonProjectTypeCategoryMessages.plugin) }}
            </nuxt-link>
          </ButtonStyled>
        </template>
        <template v-else>
          <ButtonStyled
            type="transparent"
            :highlighted="isDiscovering || isDiscoveringSubpage"
            :highlighted-style="
              isDiscoveringSubpage ? 'main-nav-secondary' : 'main-nav-primary'
            "
          >
            <nuxt-link to="/discover/plugins">
              <PlugIcon aria-hidden="true" />
              <span class="hidden md:contents">{{
                formatMessage(commonProjectTypeCategoryMessages.plugin)
              }}</span>
              <span class="contents md:hidden">{{
                formatMessage(navMenuMessages.discover)
              }}</span>
            </nuxt-link>
          </ButtonStyled>
          <ButtonStyled type="transparent" :highlighted="route.name === 'app'">
            <nuxt-link to="/app">
              <DownloadIcon aria-hidden="true" />
              <span class="hidden md:contents">{{
                formatMessage(navMenuMessages.getModrinthApp)
              }}</span>
              <span class="contents md:hidden">{{
                formatMessage(navMenuMessages.modrinthApp)
              }}</span>
            </nuxt-link>
          </ButtonStyled>
        </template>
      </div>
      <div class="flex items-center gap-1">
        <ButtonStyled circular>
          <nuxt-link
            :v-tooltip="formatMessage(commonMessages.settingsLabel)"
            to="/settings"
          >
            <SettingsIcon
              :aria-label="formatMessage(commonMessages.settingsLabel)"
            />
          </nuxt-link>
        </ButtonStyled>
      </div>
    </header>
    <header class="mobile-navigation mobile-only">
      <div
        class="nav-menu nav-menu-browse"
        :class="{ expanded: isBrowseMenuOpen }"
        @focusin="isBrowseMenuOpen = true"
        @focusout="isBrowseMenuOpen = false"
      >
        <div class="links cascade-links">
          <NuxtLink
            v-for="navRoute in navRoutes"
            :key="navRoute.href"
            :to="navRoute.href"
            class="iconified-button"
          >
            {{ navRoute.label }}
          </NuxtLink>
        </div>
      </div>
      <div
        class="nav-menu nav-menu-mobile"
        :class="{ expanded: isMobileMenuOpen }"
        @focusin="isMobileMenuOpen = true"
        @focusout="isMobileMenuOpen = false"
      >
        <div class="links">
          <NuxtLink class="iconified-button" to="/settings">
            <SettingsIcon aria-hidden="true" />
            {{ formatMessage(commonMessages.settingsLabel) }}
          </NuxtLink>
          <button class="iconified-button" @click="changeTheme">
            <ClientOnly>
              <MoonIcon v-if="$theme.active === 'light'" class="icon" />
              <SunIcon v-else class="icon" />
              <template #fallback>
                <SunIcon class="icon" />
              </template>
            </ClientOnly>
            <span class="dropdown-item__text">
              {{ formatMessage(messages.changeTheme) }}
            </span>
          </button>
        </div>
      </div>
      <div
        class="mobile-navbar"
        :class="{ expanded: isBrowseMenuOpen || isMobileMenuOpen }"
      >
        <NuxtLink
          to="/"
          class="tab button-animation"
          :title="formatMessage(navMenuMessages.home)"
          :aria-label="formatMessage(navMenuMessages.home)"
        >
          <HomeIcon aria-hidden="true" />
        </NuxtLink>
        <button
          class="tab button-animation"
          :class="{ 'router-link-exact-active': isBrowseMenuOpen }"
          :title="formatMessage(navMenuMessages.search)"
          :aria-label="formatMessage(navMenuMessages.search)"
          @click="toggleBrowseMenu()"
        >
          <SearchIcon aria-hidden="true" class="smaller" />
          {{ formatMessage(navMenuMessages.search) }}
        </button>
      </div>
    </header>
    <slot />
    <ModrinthFooter />
  </div>
</template>
<script setup>
import {
  DownloadIcon,
  HomeIcon,
  MoonIcon,
  PlugIcon,
  SearchIcon,
  SettingsIcon,
  SunIcon,
} from '@modrinth/assets'
import {
  ButtonStyled,
  commonMessages,
  commonProjectTypeCategoryMessages,
  defineMessages,
  useVIntl,
} from '@modrinth/ui'

import TextLogo from '~/components/brand/TextLogo.vue'
import ModrinthFooter from '~/components/ui/ModrinthFooter.vue'
import { getProjectTypeMessage } from '~/utils/i18n-project-type.ts'

const { formatMessage } = useVIntl()

const cosmetics = useCosmetics()
const flags = useFeatureFlags()

const config = useRuntimeConfig()
const route = useNativeRoute()
const link = config.public.siteUrl + route.path.replace(/\/+$/, '')

const navMenuMessages = defineMessages({
  home: {
    id: 'layout.nav.home',
    defaultMessage: 'Home',
  },
  search: {
    id: 'layout.nav.search',
    defaultMessage: 'Search',
  },
  discoverContent: {
    id: 'layout.nav.discover-content',
    defaultMessage: 'Discover content',
  },
  discover: {
    id: 'layout.nav.discover',
    defaultMessage: 'Discover',
  },
  getModrinthApp: {
    id: 'layout.nav.get-allay-launcher',
    defaultMessage: 'Get Launcher',
  },
  modrinthApp: {
    id: 'layout.nav.allay-launcher',
    defaultMessage: 'Launcher',
  },
})

const messages = defineMessages({
  toggleMenu: {
    id: 'layout.menu-toggle.action',
    defaultMessage: 'Toggle menu',
  },
  yourAvatarAlt: {
    id: 'layout.avatar.alt',
    defaultMessage: 'Your avatar',
  },
  changeTheme: {
    id: 'layout.action.change-theme',
    defaultMessage: 'Change theme',
  },
  modrinthHomePage: {
    id: 'layout.nav.allayhub-home-page',
    defaultMessage: 'AllayHub home page',
  },
  createNew: {
    id: 'layout.action.create-new',
    defaultMessage: 'Create new...',
  },
  reviewProjects: {
    id: 'layout.action.review-projects',
    defaultMessage: 'Project review',
  },
  techReview: {
    id: 'layout.action.tech-review',
    defaultMessage: 'Tech review',
  },
  reports: {
    id: 'layout.action.reports',
    defaultMessage: 'Review reports',
  },
  lookupByEmail: {
    id: 'layout.action.lookup-by-email',
    defaultMessage: 'Lookup by email',
  },
  fileLookup: {
    id: 'layout.action.file-lookup',
    defaultMessage: 'File lookup',
  },
  manageServerNotices: {
    id: 'layout.action.manage-server-notices',
    defaultMessage: 'Manage server notices',
  },
  manageAffiliates: {
    id: 'layout.action.manage-affiliates',
    defaultMessage: 'Manage affiliate links',
  },
  newProject: {
    id: 'layout.action.new-project',
    defaultMessage: 'New project',
  },
  newCollection: {
    id: 'layout.action.new-collection',
    defaultMessage: 'New collection',
  },
  newOrganization: {
    id: 'layout.action.new-organization',
    defaultMessage: 'New organization',
  },
  profile: {
    id: 'layout.nav.profile',
    defaultMessage: 'Profile',
  },
  savedProjects: {
    id: 'layout.nav.saved-projects',
    defaultMessage: 'Saved projects',
  },
  featureFlags: {
    id: 'layout.nav.feature-flags',
    defaultMessage: 'Feature flags',
  },
  projects: {
    id: 'layout.nav.projects',
    defaultMessage: 'Projects',
  },
  organizations: {
    id: 'layout.nav.organizations',
    defaultMessage: 'Organizations',
  },
  revenue: {
    id: 'layout.nav.revenue',
    defaultMessage: 'Revenue',
  },
  analytics: {
    id: 'layout.nav.analytics',
    defaultMessage: 'Analytics',
  },
  activeReports: {
    id: 'layout.nav.active-reports',
    defaultMessage: 'Active reports',
  },
  myServers: {
    id: 'layout.nav.my-servers',
    defaultMessage: 'My servers',
  },
  openMenu: {
    id: 'layout.mobile.open-menu',
    defaultMessage: 'Open menu',
  },
  closeMenu: {
    id: 'layout.mobile.close-menu',
    defaultMessage: 'Close menu',
  },
})

useHead({
  link: [
    {
      rel: 'canonical',
      href: link,
    },
  ],
})
useSeoMeta({
  title: 'AllayHub',
  description: () =>
    formatMessage({
      id: 'layout.meta.description',
      defaultMessage:
        'Discover AllayMC plugins on AllayHub. ' +
        'Browse and find plugins for your AllayMC Bedrock server with a modern, easy to use interface.',
    }),
  publisher: 'AllayHub',
  themeColor: '#1bd96a',
  colorScheme: 'dark light',

  // OpenGraph
  ogTitle: 'AllayHub',
  ogSiteName: 'AllayHub',
  ogDescription: () =>
    formatMessage({
      id: 'layout.meta.og-description',
      defaultMessage: 'Discover AllayMC plugins!',
    }),
  ogType: 'website',
  ogUrl: link,

  // Twitter
  twitterCard: 'summary',
})

const isMobileMenuOpen = ref(false)
const isBrowseMenuOpen = ref(false)
const navRoutes = computed(() => [
  {
    id: 'plugins',
    label: formatMessage(getProjectTypeMessage('plugin', true)),
    href: '/discover/plugins',
  },
])

const isDiscovering = computed(
  () => route.name && route.name.startsWith('discover-') && !route.query.sid,
)

const isDiscoveringSubpage = computed(
  () => route.name && route.name.startsWith('type-id') && !route.query.sid,
)

onMounted(() => {
  if (window && import.meta.client) {
    window.history.scrollRestoration = 'auto'
  }
})

watch(
  () => route.path,
  () => {
    isMobileMenuOpen.value = false
    isBrowseMenuOpen.value = false

    if (import.meta.client) {
      document.body.style.overflowY = 'scroll'
      document.body.setAttribute('tabindex', '-1')
      document.body.removeAttribute('tabindex')
    }
  },
)

function toggleBrowseMenu() {
  isBrowseMenuOpen.value = !isBrowseMenuOpen.value

  if (isBrowseMenuOpen.value) {
    isMobileMenuOpen.value = false
  }
}

const { cycle: changeTheme } = useTheme()
</script>

<style lang="scss">
@import '~/assets/styles/global.scss';

.layout {
  min-height: 100vh;
  display: block;

  @media screen and (min-width: 1024px) {
    min-height: calc(100vh - var(--spacing-card-bg));
  }

  main {
    grid-area: main;
  }
}

@media (min-width: 1024px) {
  .layout {
    main {
      .alpha-alert {
        margin: 1rem;

        .wrapper {
          padding: 1rem 2rem 1rem 1rem;
        }
      }
    }
  }
}

@media (max-width: 1200px) {
  .app-btn {
    display: none;
  }
}

.mobile-navigation {
  display: none;

  .nav-menu {
    width: 100%;
    position: fixed;
    bottom: calc(var(--size-mobile-navbar-height) - var(--size-rounded-card));
    padding-bottom: var(--size-rounded-card);
    left: 0;
    background-color: var(--color-raised-bg);
    z-index: 11; // 20 = modals, 10 = svg icons
    transform: translateY(100%);
    transition: transform 0.4s cubic-bezier(0.54, 0.84, 0.42, 1);
    border-radius: var(--size-rounded-card) var(--size-rounded-card) 0 0;
    box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0);

    .links,
    .account-container {
      display: grid;
      grid-template-columns: repeat(1, 1fr);
      grid-gap: 1rem;
      justify-content: center;
      padding: 1rem;

      .iconified-button {
        width: 100%;
        max-width: 500px;
        padding: 0.75rem;
        justify-content: center;
        font-weight: 600;
        font-size: 1rem;
        margin: 0 auto;
      }
    }

    .cascade-links {
      @media screen and (min-width: 354px) {
        grid-template-columns: repeat(2, 1fr);
      }

      @media screen and (min-width: 674px) {
        grid-template-columns: repeat(3, 1fr);
      }
    }

    &-browse {
      &.expanded {
        transform: translateY(0);
        box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0.3);
      }
    }

    &-mobile {
      .account-container {
        padding-bottom: 0;

        .account-button {
          padding: var(--spacing-card-md);
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 0.5rem;

          .user-icon {
            width: 2.25rem;
            height: 2.25rem;
          }

          .account-text {
            flex-grow: 0;
          }
        }
      }

      &.expanded {
        transform: translateY(0);
        box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0.3);
      }
    }
  }

  .mobile-navbar {
    display: flex;
    height: calc(
      var(--size-mobile-navbar-height) + env(safe-area-inset-bottom)
    );
    border-radius: var(--size-rounded-card) var(--size-rounded-card) 0 0;
    padding-bottom: env(safe-area-inset-bottom);
    position: fixed;
    left: 0;
    bottom: 0;
    background-color: var(--color-raised-bg);
    box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0.3);
    z-index: 11; // 20 = modals, 10 = svg icons
    width: 100%;
    align-items: center;
    justify-content: space-between;
    transition: border-radius 0.3s ease-out;
    border-top: 2px solid rgba(0, 0, 0, 0);
    box-sizing: border-box;

    &.expanded {
      box-shadow: none;
      border-radius: 0;
    }

    .tab {
      position: relative;
      background: none;
      display: flex;
      flex-basis: 0;
      justify-content: center;
      align-items: center;
      flex-direction: row;
      gap: 0.25rem;
      font-weight: bold;
      padding: 0;
      transition: color ease-in-out 0.15s;
      color: var(--color-text-inactive);
      text-align: center;

      &.browse {
        svg {
          transform: rotate(180deg);
          transition: transform ease-in-out 0.3s;

          &.closed {
            transform: rotate(0deg);
          }
        }
      }

      &.bubble {
        &::after {
          background-color: var(--color-brand);
          border-radius: var(--size-rounded-max);
          content: '';
          height: 0.5rem;
          position: absolute;
          left: 1.5rem;
          top: 0;
          width: 0.5rem;
        }
      }

      svg {
        height: 1.75rem;
        width: 1.75rem;

        &.smaller {
          width: 1.25rem;
          height: 1.25rem;
        }
      }

      .user-icon {
        width: 2rem;
        height: 2rem;
        transition: border ease-in-out 0.15s;
        border: 0 solid var(--color-brand);
        box-sizing: border-box;

        &.expanded {
          border: 2px solid var(--color-brand);
        }
      }

      &:hover,
      &:focus {
        color: var(--color-text);
      }

      &:first-child {
        margin-left: 2rem;
      }

      &:last-child {
        margin-right: 2rem;
      }

      &.router-link-exact-active:not(&.no-active) {
        svg {
          color: var(--color-brand);
        }

        color: var(--color-brand);
      }
    }
  }
}

@media (any-hover: none) and (max-width: 640px) {
  .desktop-only {
    display: none;
  }
}

@media (any-hover: none) and (max-width: 640px) {
  .mobile-navigation {
    display: flex;
  }
}
</style>
