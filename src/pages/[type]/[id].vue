<template>
  <Teleport v-if="flags.projectBackground" to="#fixed-background-teleport">
    <ProjectBackgroundGradient :project="project" />
  </Teleport>
  <div class="experimental-styles-within">
    <div
      class="over-the-top-download-animation"
      :class="{ 'animation-hidden': !overTheTopDownloadAnimation }"
    >
      <div>
        <div
          class="animation-ring-3 flex items-center justify-center rounded-full border-4 border-solid border-brand bg-brand-highlight opacity-40"
        ></div>

        <div
          class="animation-ring-2 flex items-center justify-center rounded-full border-4 border-solid border-brand bg-brand-highlight opacity-60"
        ></div>

        <div
          class="animation-ring-1 flex items-center justify-center rounded-full border-4 border-solid border-brand bg-brand-highlight"
        >
          <DownloadIcon class="h-20 w-20 text-contrast" />
        </div>
      </div>
    </div>
    <NewModal
      ref="downloadModal"
      :on-show="
        () => {
          navigateTo({ query: route.query, hash: '#download' })
        }
      "
      :on-hide="
        () => {
          navigateTo({ query: route.query, hash: '' })
        }
      "
    >
      <template #title>
        <Avatar
          :src="project.icon_url"
          :alt="project.title"
          class="icon"
          size="32px"
        />
        <div class="truncate text-lg font-extrabold text-contrast">
          {{ formatMessage(messages.downloadTitle, { title: project.title }) }}
        </div>
      </template>
      <template #default>
        <div class="mx-auto flex max-w-[40rem] flex-col gap-4 md:w-[30rem]">
          <div class="mx-auto flex w-fit flex-col gap-2">
            <ButtonStyled v-if="project.game_versions.length === 1">
              <div class="disabled button-like">
                <GameIcon aria-hidden="true" />
                {{
                  currentGameVersion
                    ? formatMessage(messages.gameVersionLabel, {
                        version: currentGameVersion,
                      })
                    : formatMessage(messages.gameVersionError)
                }}
                <InfoIcon
                  v-tooltip="
                    formatMessage(messages.gameVersionTooltip, {
                      title: project.title,
                      version: currentGameVersion,
                    })
                  "
                  class="ml-auto size-5"
                />
              </div>
            </ButtonStyled>
            <Accordion
              v-else
              ref="gameVersionAccordion"
              class="accordion-with-bg"
            >
              <template #title>
                <GameIcon aria-hidden="true" />
                {{
                  currentGameVersion
                    ? formatMessage(messages.gameVersionLabel, {
                        version: currentGameVersion,
                      })
                    : formatMessage(messages.selectGameVersion)
                }}
              </template>
              <div class="iconified-input mb-2 flex w-full">
                <label for="game-versions-filtering" hidden>{{
                  formatMessage(messages.searchGameVersionsLabel)
                }}</label>
                <SearchIcon aria-hidden="true" />
                <input
                  id="game-versions-filtering"
                  ref="gameVersionFilterInput"
                  v-model="versionFilter"
                  type="search"
                  autocomplete="off"
                  :placeholder="formatMessage(messages.searchGameVersions)"
                />
              </div>
              <ScrollablePanel
                :class="project.game_versions.length > 4 ? 'h-[15rem]' : ''"
              >
                <ButtonStyled
                  v-for="gameVersion in project.game_versions
                    .filter(
                      (x) =>
                        (versionFilter && x.includes(versionFilter)) ||
                        (!versionFilter &&
                          (showAllVersions || isReleaseGameVersion(x))),
                    )
                    .slice()
                    .reverse()"
                  :key="gameVersion"
                  :color="
                    currentGameVersion === gameVersion ? 'brand' : 'standard'
                  "
                >
                  <button
                    v-tooltip="
                      !possibleGameVersions.includes(gameVersion)
                        ? formatMessage(
                            messages.gameVersionUnsupportedTooltip,
                            {
                              title: project.title,
                              gameVersion: gameVersion,
                            },
                          )
                        : null
                    "
                    :class="{
                      'looks-disabled !text-brand-red':
                        !possibleGameVersions.includes(gameVersion),
                    }"
                    @click="
                      () => {
                        userSelectedGameVersion = gameVersion
                        gameVersionAccordion.close()

                        navigateTo({
                          query: {
                            ...route.query,
                            version: userSelectedGameVersion,
                          },
                          hash: route.hash,
                        })
                      }
                    "
                  >
                    {{ gameVersion }}
                    <CheckIcon v-if="userSelectedGameVersion === gameVersion" />
                  </button>
                </ButtonStyled>
              </ScrollablePanel>
            </Accordion>
          </div>
          <AutomaticAccordion div class="flex flex-col gap-2">
            <VersionSummary
              v-if="filteredRelease"
              :version="filteredRelease"
              @on-download="onDownload"
              @on-navigate="downloadModal.hide"
            />
            <VersionSummary
              v-if="filteredBeta"
              :version="filteredBeta"
              @on-download="onDownload"
              @on-navigate="downloadModal.hide"
            />
            <p v-if="currentGameVersion && !filteredRelease && !filteredBeta">
              {{
                formatMessage(messages.noVersionsAvailable, {
                  gameVersion: currentGameVersion,
                })
              }}
            </p>
          </AutomaticAccordion>
        </div>
      </template>
    </NewModal>
    <div
      class="new-page sidebar"
      :class="{
        'alt-layout': cosmetics.leftContentLayout,
        'checklist-open': !flags.alwaysShowChecklistAsPopup,
        'checklist-collapsed': !flags.alwaysShowChecklistAsPopup,
      }"
    >
      <div class="normal-page__header relative my-4">
        <ProjectHeader :project="project as never" :member="!!currentMember">
          <template #actions>
            <div class="contents sm:hidden">
              <ButtonStyled
                size="large"
                circular
                :color="
                  route.name === 'type-id-version-version'
                    ? `standard`
                    : `brand`
                "
              >
                <button
                  :aria-label="formatMessage(commonMessages.downloadButton)"
                  class="flex sm:hidden"
                  @click="(event: Event) => downloadModal.show(event)"
                >
                  <DownloadIcon aria-hidden="true" />
                </button>
              </ButtonStyled>
            </div>
          </template>
        </ProjectHeader>
        <MessageBanner
          v-if="project.status === 'archived'"
          message-type="warning"
          class="my-4"
        >
          {{
            formatMessage(messages.archivedMessage, { title: project.title })
          }}
        </MessageBanner>
      </div>

      <div class="normal-page__sidebar">
        <ProjectSidebarCompatibility
          v-if="project.id"
          :is-outdated="projectIsOutdated"
          :uses-server-api="projectUsesServerApi"
          class="card flex-card experimental-styles-within"
        />
        <ProjectSidebarLinks
          v-if="project.id"
          :project="project"
          :link-target="'_blank'"
          class="card flex-card experimental-styles-within"
        />
        <ProjectSidebarCreators
          v-if="project.id"
          :organization="organization"
          :members="members"
          :org-link="(slug: string) => `/organization/${slug}`"
          class="card flex-card experimental-styles-within"
        />
        <div class="card flex-card experimental-styles-within">
          <h2>{{ formatMessage(detailsMessages.title) }}</h2>

          <div class="details-list">
            <div class="details-list__item">
              <BookTextIcon aria-hidden="true" />
              <div>
                {{ formatMessage(messages.licensedLabel) }}
                <a
                  v-if="licenseUrl"
                  class="text-link hover:underline"
                  :href="licenseUrl"
                  target="_blank"
                  rel="noopener nofollow ugc"
                >
                  {{ licenseIdDisplay }}
                  <ExternalIcon
                    aria-hidden="true"
                    class="external-icon ml-1 mt-[-1px] inline"
                  />
                </a>
                <span v-else>{{ licenseIdDisplay }}</span>
              </div>
            </div>

            <div
              v-if="project.approved"
              v-tooltip="
                $dayjs(project.approved).format('MMMM D, YYYY [at] h:mm A')
              "
              class="details-list__item"
            >
              <CalendarIcon aria-hidden="true" />
              <div>
                {{
                  formatMessage(detailsMessages.published, {
                    date: publishedDate,
                  })
                }}
              </div>
            </div>

            <div
              v-else
              v-tooltip="
                $dayjs(project.published).format('MMMM D, YYYY [at] h:mm A')
              "
              class="details-list__item"
            >
              <CalendarIcon aria-hidden="true" />
              <div>
                {{
                  formatMessage(detailsMessages.created, { date: createdDate })
                }}
              </div>
            </div>

            <div
              v-if="project.status === 'processing' && project.queued"
              v-tooltip="
                $dayjs(project.queued).format('MMMM D, YYYY [at] h:mm A')
              "
              class="details-list__item"
            >
              <ScaleIcon aria-hidden="true" />
              <div>
                {{
                  formatMessage(detailsMessages.submitted, {
                    date: submittedDate,
                  })
                }}
              </div>
            </div>

            <div
              v-if="versions.length > 0 && project.updated"
              v-tooltip="
                $dayjs(project.updated).format('MMMM D, YYYY [at] h:mm A')
              "
              class="details-list__item"
            >
              <VersionIcon aria-hidden="true" />
              <div>
                {{
                  formatMessage(detailsMessages.updated, { date: updatedDate })
                }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="normal-page__content">
        <div class="overflow-x-auto">
          <NavTabs :links="navLinks" class="mb-4" />
        </div>
        <NuxtPage
          v-model:project="project"
          v-model:versions="versions"
          v-model:members="members"
          v-model:all-members="allMembers"
          v-model:dependencies="dependencies"
          v-model:organization="organization"
          :current-member="currentMember"
          :reset-project="resetProject"
          :reset-versions="resetVersions"
          :reset-organization="resetOrganization"
          :reset-members="resetMembers"
          :route="route"
          @on-download="triggerDownloadAnimation"
          @delete-version="deleteVersion"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  BookTextIcon,
  CalendarIcon,
  CheckIcon,
  DownloadIcon,
  ExternalIcon,
  GameIcon,
  InfoIcon,
  ScaleIcon,
  SearchIcon,
  VersionIcon,
} from '@modrinth/assets'
import {
  Avatar,
  ButtonStyled,
  commonMessages,
  defineMessages,
  NewModal,
  ProjectBackgroundGradient,
  ProjectHeader,
  ProjectSidebarCompatibility,
  ProjectSidebarCreators,
  ProjectSidebarLinks,
  provideProjectPageContext,
  ScrollablePanel,
  useRelativeTime,
  useVIntl,
} from '@modrinth/ui'
import VersionSummary from '@modrinth/ui/src/components/version/VersionSummary.vue'
import { formatProjectType } from '@modrinth/utils'
import dayjs from 'dayjs'

import { navigateTo } from '#app'
import Accordion from '~/components/ui/Accordion.vue'
import AutomaticAccordion from '~/components/ui/AutomaticAccordion.vue'
import MessageBanner from '~/components/ui/MessageBanner.vue'
import NavTabs from '~/components/ui/NavTabs.vue'
import { usePlugin, useApiVersions } from '~/composables/usePlugins'
import type AllayIndex from '~/types/allayhub-index'
import { LATEST_API_VERSION } from '~/types/allayhub'
import { isPluginOutdated } from '~/utils/version'

const route = useNativeRoute()

const { data: apiVersionsData } = useApiVersions()
const flags = useFeatureFlags()
const cosmetics = useCosmetics()

const { formatMessage } = useVIntl()

const downloadModal = ref()
const overTheTopDownloadAnimation = ref()

const userSelectedGameVersion = ref<string | null>(null)
const showAllVersions = ref(false)

const versionFilter = ref('')

const currentGameVersion = computed(() => {
  return (
    userSelectedGameVersion.value ||
    (project.value.game_versions.length === 1 && project.value.game_versions[0])
  )
})

// API version this plugin supports
const possibleGameVersions = computed(() => {
  return project.value.api_version ? [project.value.api_version] : []
})

// In AllayHub, all API versions are considered "release" versions
const releaseVersions = computed(() => {
  const set = new Set()
  const versions = apiVersionsData.value?.versions || []
  for (const v of versions) {
    if (v?.version) set.add(v.version)
  }
  return set
})

function isReleaseGameVersion(ver: string): boolean {
  return releaseVersions.value.has(ver)
}

const gameVersionAccordion = ref()

const formatRelativeTime = useRelativeTime()

const detailsMessages = defineMessages({
  title: {
    id: 'project.about.details.title',
    defaultMessage: 'Details',
  },
  licensed: {
    id: 'project.about.details.licensed',
    defaultMessage: 'Licensed {license}',
  },
  created: {
    id: 'project.about.details.created',
    defaultMessage: 'Created {date}',
  },
  submitted: {
    id: 'project.about.details.submitted',
    defaultMessage: 'Submitted {date}',
  },
  published: {
    id: 'project.about.details.published',
    defaultMessage: 'Published {date}',
  },
  updated: {
    id: 'project.about.details.updated',
    defaultMessage: 'Updated {date}',
  },
})

const messages = defineMessages({
  archivedMessage: {
    id: 'project.status.archived.message',
    defaultMessage:
      '{title} has been archived. {title} will not receive any further updates unless the author decides to unarchive the project.',
  },
  changelogTab: {
    id: 'project.navigation.changelog',
    defaultMessage: 'Changelog',
  },
  createNewCollection: {
    id: 'project.collections.create-new',
    defaultMessage: 'Create new collection',
  },
  descriptionTab: {
    id: 'project.description.title',
    defaultMessage: 'Description',
  },
  downloadTitle: {
    id: 'project.download.title',
    defaultMessage: 'Download {title}',
  },
  downloadsStat: {
    id: 'project.stats.downloads-label',
    defaultMessage: 'download{count, plural, one {} other {s}}',
  },
  errorLoadingProject: {
    id: 'project.error.loading',
    defaultMessage: 'Error loading project data{message}',
  },
  environmentMigrationMessage: {
    id: 'project.environment.migration.message',
    defaultMessage:
      "We've updated the Environments system and new options are now available. Please verify that the metadata is correct.",
  },
  environmentMigrationTitle: {
    id: 'project.environment.migration.title',
    defaultMessage: 'Please review environment metadata',
  },
  environmentMigrationNoPermissionMessage: {
    id: 'project.environment.migration-no-permission.message',
    defaultMessage:
      "We've updated the Environments system and new options are now available. You don't have permission to modify these settings, but please let another member of the project know that the environment metadata needs to be verified.",
  },
  environmentMigrationNoPermissionTitle: {
    id: 'project.environment.migration-no-permission.title',
    defaultMessage: 'Environment metadata needs to be reviewed',
  },
  environmentMigrationLink: {
    id: 'project.environment.migration.learn-more',
    defaultMessage: 'Learn more about this change',
  },
  followersStat: {
    id: 'project.stats.followers-label',
    defaultMessage: 'follower{count, plural, one {} other {s}}',
  },
  galleryTab: {
    id: 'project.gallery.title',
    defaultMessage: 'Gallery',
  },
  gameVersionError: {
    id: 'project.download.game-version-error',
    defaultMessage: 'Error: no game versions found',
  },
  gameVersionLabel: {
    id: 'project.download.game-version',
    defaultMessage: 'Game version: {version}',
  },
  gameVersionTooltip: {
    id: 'project.download.game-version-tooltip',
    defaultMessage: '{title} is only available for {version}',
  },
  gameVersionUnsupportedTooltip: {
    id: 'project.download.game-version-unsupported-tooltip',
    defaultMessage: '{title} does not support {gameVersion} for {platform}',
  },
  licensedLabel: {
    id: 'project.details.licensed',
    defaultMessage: 'Licensed',
  },
  moderationTab: {
    id: 'project.moderation.title',
    defaultMessage: 'Moderation',
  },
  noCollectionsFound: {
    id: 'project.collections.none-found',
    defaultMessage: 'No collections found.',
  },
  noVersionsAvailable: {
    id: 'project.download.no-versions-available',
    defaultMessage: 'No versions available for {gameVersion} and {platform}.',
  },
  pageNotFound: {
    id: 'project.error.page-not-found',
    defaultMessage: 'The page could not be found',
  },
  platformError: {
    id: 'project.download.platform-error',
    defaultMessage: 'Error: no platforms found',
  },
  platformLabel: {
    id: 'project.download.platform',
    defaultMessage: 'Platform: {platform}',
  },
  platformTooltip: {
    id: 'project.download.platform-tooltip',
    defaultMessage: '{title} is only available for {platform}',
  },
  platformUnsupportedTooltip: {
    id: 'project.download.platform-unsupported-tooltip',
    defaultMessage: '{title} does not support {platform} for {gameVersion}',
  },
  projectIconUpdated: {
    id: 'project.notification.icon-updated.title',
    defaultMessage: 'Project icon updated',
  },
  projectIconUpdatedMessage: {
    id: 'project.notification.icon-updated.message',
    defaultMessage: "Your project's icon has been updated.",
  },
  projectNotFound: {
    id: 'project.error.project-not-found',
    defaultMessage: 'Project not found',
  },
  projectUpdated: {
    id: 'project.notification.updated.title',
    defaultMessage: 'Project updated',
  },
  projectUpdatedMessage: {
    id: 'project.notification.updated.message',
    defaultMessage: 'Your project has been updated.',
  },
  reviewEnvironmentSettings: {
    id: 'project.environment.migration.review-button',
    defaultMessage: 'Review environment settings',
  },
  reviewProject: {
    id: 'project.actions.review-project',
    defaultMessage: 'Review project',
  },
  searchGameVersions: {
    id: 'project.download.search-game-versions',
    defaultMessage: 'Search game versions...',
  },
  searchGameVersionsLabel: {
    id: 'project.download.search-game-versions-label',
    defaultMessage: 'Search game versions...',
  },
  selectGameVersion: {
    id: 'project.download.select-game-version',
    defaultMessage: 'Select game version',
  },
  selectPlatform: {
    id: 'project.download.select-platform',
    defaultMessage: 'Select platform',
  },
  settingsTitle: {
    id: 'project.settings.title',
    defaultMessage: 'Settings',
  },
  versionsTab: {
    id: 'project.versions.title',
    defaultMessage: 'Versions',
  },
  dependenciesTab: {
    id: 'project.dependencies.title',
    defaultMessage: 'Dependencies',
  },
  visitProjectsDashboard: {
    id: 'project.settings.visit-dashboard',
    defaultMessage: 'Visit projects dashboard',
  },
})

const createdDate = computed(() =>
  project.value.published
    ? formatRelativeTime(new Date(project.value.published))
    : 'unknown',
)
const submittedDate = computed(() =>
  project.value.queued
    ? formatRelativeTime(new Date(project.value.queued))
    : 'unknown',
)
const publishedDate = computed(() =>
  project.value.approved
    ? formatRelativeTime(new Date(project.value.approved))
    : 'unknown',
)
const updatedDate = computed(() =>
  project.value.updated
    ? formatRelativeTime(new Date(project.value.updated))
    : 'unknown',
)

const licenseIdDisplay = computed(() => {
  const id = project.value.license.id

  if (id === 'LicenseRef-All-Rights-Reserved') {
    return 'ARR'
  } else if (id.includes('LicenseRef')) {
    return id.replaceAll('LicenseRef-', '').replaceAll('-', ' ')
  } else {
    return id
  }
})

// License URL from backend (empty for ARR)
const licenseUrl = computed(() => project.value.license.url || null)

const filteredVersions = computed(() => {
  const gameVer = currentGameVersion.value
  if (!gameVer) return []
  // All versions share the same API version at plugin level
  if (project.value.api_version !== gameVer) return []
  return versions.value
})

const filteredRelease = computed(() => {
  return filteredVersions.value.find((x) => !x.prerelease)
})

const filteredBeta = computed(() => {
  return filteredVersions.value.find(
    (x) =>
      x.prerelease &&
      (!filteredRelease.value ||
        dayjs(x.published_at).isAfter(
          dayjs(filteredRelease.value.published_at),
        )),
  )
})

const projectId = computed(() => {
  const id = route.params.id
  return Array.isArray(id) ? id[0] : id
})

// Load plugin data from AllayHub Index
// Using await triggers the top progress bar during page navigation
const { data: pluginData, error: pluginError } = await usePlugin(projectId)

// Helper: Convert Unix timestamp to ISO string
function toISOString(ts: number): string {
  return new Date(ts * 1000).toISOString()
}

// Transform raw versions from JSON to Version[] for template
function transformVersions(
  rawVersions: AllayIndex.RawVersion[],
  projectId: string,
  authorId: string,
): AllayIndex.Version[] {
  return rawVersions.map((raw) => ({
    id: `${projectId}_${raw.version}`,
    project_id: projectId,
    name: raw.name,
    version: raw.version,
    changelog: raw.changelog || undefined,
    published_at: new Date(raw.published_at * 1000).toISOString(),
    downloads: raw.downloads,
    prerelease: raw.prerelease,
    author_id: authorId,
    files: raw.files,
  }))
}

// Transform AllayIndex.Plugin to ProjectView for template display
function transformPluginToProject(
  plugin: AllayIndex.Plugin | null,
): AllayIndex.ProjectView | null {
  if (!plugin) return null

  const authorId = plugin.authors[0]?.name || 'unknown'

  return {
    id: plugin.id,
    slug: plugin.id, // Use id as slug (they're the same)
    project_type: 'plugin',
    title: plugin.name,
    description: plugin.summary,
    body: plugin.description,
    icon_url: plugin.icon_url,
    status: 'approved',
    license: {
      id: plugin.license.id,
      name: plugin.license.name,
      url: plugin.license.url || null,
    },
    downloads: plugin.downloads,
    followers: plugin.stars,
    categories: plugin.categories,
    game_versions: plugin.api_version ? [plugin.api_version] : [],
    loaders: ['allay'],
    gallery: plugin.gallery || [],
    versions: transformVersions(plugin.versions, plugin.id, authorId),
    published: toISOString(plugin.created_at),
    updated: toISOString(plugin.updated_at),
    approved: toISOString(plugin.created_at),
    queued: toISOString(plugin.created_at),
    color: 0x1bd96a,
    issues_url: plugin.source ? `${plugin.source}/issues` : '',
    source_url: plugin.source,
    wiki_url: plugin.links?.wiki || '',
    discord_url: plugin.links?.discord || '',
    donation_url: { platform: '', url: '' },
    api_version: plugin.api_version,
    server_version: plugin.server_version || undefined,
    dependencies:
      plugin.dependencies?.map((d) => ({
        plugin_id: d.plugin_id,
        version_range: d.version_range,
        dependency_type: d.dependency_type as 'required' | 'optional',
      })) || [],
  }
}

// Create MemberView from plugin authors
function createMembersFromAuthors(
  plugin: AllayIndex.Plugin | null,
): AllayIndex.MemberView[] {
  if (!plugin) return []
  return plugin.authors.map((author, index) => ({
    id: `member-${plugin.id}-${index}`,
    team_id: `team-${plugin.id}`,
    user: {
      id: `user-${index}`,
      username: author.name,
      avatar_url: author.avatar_url || '/placeholder.png',
    },
    role: index === 0 ? 'Owner' : 'Member',
    is_owner: index === 0,
    accepted: true,
    permissions: 1023,
    payouts_split: index === 0 ? 100 : 0,
  }))
}

// Default project for loading state
const defaultProject: AllayIndex.ProjectView = {
  id: '',
  slug: '',
  project_type: 'plugin',
  title: 'Loading...',
  description: '',
  body: '',
  status: 'approved',
  license: { id: 'Unknown', name: 'Unknown', url: null },
  downloads: 0,
  followers: 0,
  categories: [],
  game_versions: [],
  loaders: ['allay'],
  gallery: [],
  versions: [],
  published: '',
  updated: '',
  approved: '',
  queued: '',
  color: 0x1bd96a,
  issues_url: '',
  source_url: '',
  wiki_url: '',
  discord_url: '',
  donation_url: { platform: '', url: '' },
  api_version: '',
  server_version: undefined,
  dependencies: [],
}

// Transform plugin data (already loaded via await)
const transformedProject = computed(() =>
  transformPluginToProject(pluginData.value),
)

if (pluginError.value || !pluginData.value) {
  throw createError({
    fatal: true,
    statusCode: 404,
    message: formatMessage(messages.projectNotFound),
  })
}

// Use reactive refs that update when data loads
const project = computed<AllayIndex.ProjectView>(
  () => transformedProject.value || defaultProject,
)
const projectV3 = computed(() => ({
  ...project.value,
  environment: ['server'],
  project_types: ['plugin'],
  side_types_migration_review_status: null,
}))
const allMembers = computed(() => createMembersFromAuthors(pluginData.value))
const dependencies = ref({ projects: [], versions: [] })
const versions = computed(() => project.value?.versions || [])
const organization = ref(null)

// No-op refresh functions - static data doesn't need refresh
const resetProject = async () => {}
const resetVersions = async () => {}
const resetMembers = async () => {}
const resetOrganization = async () => {}

// Only redirect after data is loaded (id must exist)
if (
  project.value.id &&
  (project.value.project_type !== route.params.type ||
    (route.params.id !== project.value.id &&
      !flags.value.disablePrettyProjectUrlRedirects))
) {
  let path = route.fullPath.split('/')
  path.splice(0, 3)
  path = path.filter((x) => x)

  await navigateTo(
    `/${project.value.project_type}/${project.value.id}${
      path.length > 0 ? `/${path.join('/')}` : ''
    }`,
    { redirectCode: 301, replace: true },
  )
}

// Members should be an array of all members, sorted with owner first
const members = computed(() => {
  const acceptedMembers = allMembers.value.filter((x) => x.accepted)
  const owner = acceptedMembers.find((x) => x.is_owner)

  const rest =
    acceptedMembers.filter((x) => !owner || x.user.id !== owner.user.id) || []

  rest.sort((a, b) => {
    if (a.role === b.role) {
      return a.user.username.localeCompare(b.user.username)
    } else {
      return a.role.localeCompare(b.role)
    }
  })

  return owner ? [owner, ...rest] : rest
})

const currentMember = computed(() => {
  return null
})

const projectTypeDisplay = computed(() => formatProjectType('plugin'))

// Compatibility warnings
const projectIsOutdated = computed(() =>
  isPluginOutdated(project.value.api_version, LATEST_API_VERSION),
)
const projectUsesServerApi = computed(() => !!project.value.server_version)

const title = computed(
  () => `${project.value.title} - Allay ${projectTypeDisplay.value}`,
)
const description = computed(
  () =>
    `${project.value.description} - Download the Allay ${projectTypeDisplay.value} ${
      project.value.title
    } by ${members.value.find((x) => x.is_owner)?.user?.username || 'a creator'} on AllayHub`,
)

useSeoMeta({
  title: () => title.value,
  description: () => description.value,
  ogTitle: () => title.value,
  ogDescription: () => project.value.description,
  ogImage: () => project.value.icon_url ?? '/placeholder.png',
  robots: () =>
    project.value.status === 'approved' || project.value.status === 'archived'
      ? 'all'
      : 'noindex',
})

const { version } = route.query

if (
  project.value.game_versions.length > 0 &&
  project.value.game_versions.every((v) => !isReleaseGameVersion(v))
) {
  showAllVersions.value = true
}

if (
  typeof version === 'string' &&
  project.value.game_versions.includes(version)
) {
  userSelectedGameVersion.value = version
}

watch(downloadModal, (modal) => {
  if (!modal) return

  // route.hash returns everything in the hash string, including the # itself
  if (route.hash === '#download') {
    modal.show()
  }
})

function closeDownloadModal(event: Event): void {
  downloadModal.value?.hide(event)
  userSelectedGameVersion.value = null
  showAllVersions.value = false
}

function triggerDownloadAnimation(): void {
  overTheTopDownloadAnimation.value = true
  setTimeout(() => (overTheTopDownloadAnimation.value = false), 500)
}

function onDownload(event: Event): void {
  triggerDownloadAnimation()
  setTimeout(() => {
    closeDownloadModal(event)
  }, 400)
}

async function deleteVersion(_id: string): Promise<void> {
  // No-op in static mode
}

const navLinks = computed(() => {
  const projectUrl = `/${project.value.project_type}/${project.value.slug ? project.value.slug : project.value.id}`

  return [
    {
      label: formatMessage(messages.descriptionTab),
      href: projectUrl,
    },
    {
      label: formatMessage(messages.galleryTab),
      href: `${projectUrl}/gallery`,
      shown: project.value.gallery && project.value.gallery.length > 0,
    },
    {
      label: formatMessage(messages.changelogTab),
      href: `${projectUrl}/changelog`,
      shown: versions.value.length > 0,
    },
    {
      label: formatMessage(messages.versionsTab),
      href: `${projectUrl}/versions`,
      shown: versions.value.length > 0 || !!currentMember.value,
      subpages: [`${projectUrl}/version/`],
    },
    {
      label: formatMessage(messages.dependenciesTab),
      href: `${projectUrl}/dependencies`,
      shown:
        project.value.dependencies && project.value.dependencies.length > 0,
    },
  ]
})

provideProjectPageContext({
  projectV2: project,
  projectV3,
  refreshProject: resetProject,
  refreshVersions: resetVersions,
  currentMember,
})
</script>

<style lang="scss" scoped>
.settings-header {
  display: flex;
  flex-direction: row;
  gap: var(--spacing-card-sm);
  align-items: center;
  margin-bottom: var(--spacing-card-bg);

  .settings-header__icon {
    flex-shrink: 0;
  }

  .settings-header__text {
    h1 {
      font-size: var(--font-size-md);
      margin-top: 0;
      margin-bottom: var(--spacing-card-sm);
    }
  }
}

.popout-checkbox {
  padding: var(--gap-sm) var(--gap-md);
  white-space: nowrap;

  &:hover {
    filter: brightness(0.95);
  }
}

.popout-heading {
  padding: var(--gap-sm) var(--gap-md);
  padding-bottom: 0;
  font-size: var(--font-size-nm);
  color: var(--color-secondary);
}

.collection-button {
  margin: var(--gap-sm) var(--gap-md);
  white-space: nowrap;
}

.menu-text {
  padding: 0 var(--gap-md);
  font-size: var(--font-size-nm);
  color: var(--color-secondary);
}

.menu-search {
  margin: var(--gap-sm) var(--gap-md);
  width: calc(100% - var(--gap-md) * 2);
}

.collections-list {
  max-height: 40rem;
  overflow-y: auto;
  background-color: var(--color-bg);
  border-radius: var(--radius-md);
  margin: var(--gap-sm) var(--gap-md);
  padding: var(--gap-sm);
}

.normal-page__info:empty {
  display: none;
}

:deep(.accordion-with-bg) {
  @apply rounded-2xl bg-bg p-2;
  --scrollable-pane-bg: var(--color-bg);
}

.over-the-top-download-animation {
  position: fixed;
  z-index: 100;
  inset: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  pointer-events: none;
  scale: 0.5;
  transition: all 0.5s ease-out;
  opacity: 1;

  &.animation-hidden {
    scale: 0.8;
    opacity: 0;

    .animation-ring-1 {
      width: 25rem;
      height: 25rem;
    }

    .animation-ring-2 {
      width: 50rem;
      height: 50rem;
    }

    .animation-ring-3 {
      width: 100rem;
      height: 100rem;
    }
  }

  > div {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    width: fit-content;
    height: fit-content;

    > * {
      position: absolute;
      scale: 1;
      transition: all 0.2s ease-out;
      width: 20rem;
      height: 20rem;
    }
  }
}

@media (hover: none) and (max-width: 767px) {
  .modrinth-app-section {
    display: none;
  }
}

.servers-popup {
  box-shadow:
    0 0 12px 1px rgba(0, 175, 92, 0.6),
    var(--shadow-floating);

  &::before {
    width: 0;
    height: 0;
    border-left: 6px solid transparent;
    border-right: 6px solid transparent;
    border-bottom: 6px solid var(--color-button-bg);
    content: ' ';
    position: absolute;
    top: -7px;
    left: 17px;
  }
  &::after {
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-bottom: 5px solid var(--color-raised-bg);
    content: ' ';
    position: absolute;
    top: -5px;
    left: 18px;
  }
}

.moderation-checklist {
  position: fixed;
  bottom: 1rem;
  right: 1rem;
  overflow-y: auto;
  z-index: 50;

  > div {
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.3);
  }
}
</style>
